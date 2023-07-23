#![allow(non_snake_case)]
use std::{
    fs,
    process::{Child, Command},
    time::{Duration, SystemTime},
};

use anyhow::Context;
use comrak::{markdown_to_html, ComrakOptions};
use dioxus::prelude::*;
use futures::{stream, StreamExt};
use log::{debug, trace};
use persistent_structs::PersistentStruct;
use tokio::select;

use crate::{schema::v1::Subject, ActiveMode, Mode, Schema, State};

macro_rules! coro_try {
    ($state:ident, $expr:expr) => {
        match $expr {
            Ok(x) => x,
            Err(e) => {
                $state.write().mode = Mode::Fatal(format!("{e:?}"));
                return;
            }
        }
    };
}

#[inline_props]
pub fn EditingSubject(cx: Scope, name: String) -> Element {
    let html = use_state(cx, || "".to_string());
    let is_err = use_state(cx, || false);
    let state = use_shared_state::<State>(cx).unwrap();

    use_coroutine(cx, |mut rx: UnboundedReceiver<()>| {
        to_owned![html, is_err, state, name];
        let db_path = state.read().db_path.clone().unwrap();

        async move {
            let mut db = coro_try!(state, Schema::open(&db_path));
            // create subject if it doesn't exist, otherwise simply query id
            debug!("Editing Subject with name: {name}");
            let mut sub = match coro_try!(state, db.get_sub_by_name(&name)) {
                Some(sub) => {
                    trace!("Subject already exists");
                    sub
                }
                None => {
                    let sub = Subject {
                        name: name.clone(),
                        description: "".into(),
                    };
                    debug!("Subject not yet existing, creating entry");
                    coro_try!(state, db.insert_subject(&sub));
                    trace!("Insert successful");
                    let sub = coro_try!(state, db.get_sub_by_name(&name)).unwrap();
                    trace!("Query successful");
                    sub
                }
            };
            let mut adapter = coro_try!(state, editor_stream(sub.description.clone()));

            loop {
                select! {
                    _ = rx.next() => {
                        debug!("coroutine interrupted");
                        break;
                    }
                    msg = adapter.next() => {
                        match msg {
                            Some(Ok(s)) => {
                                debug!("got text update");
                                sub.description = s.clone();
                                coro_try!(state, db.update_subject(&sub));
                                trace!("succesfully wrote to db");
                                html.set(markdown_to_html(&s, &ComrakOptions::default()));
                                is_err.set(false);
                            },
                            Some(Err(s)) => {
                                debug!("Got Err: {s}");
                                html.set(s);
                                is_err.set(true);},
                            None => {
                                debug!("Editor closed");
                                state.write().mode = Mode::Active(ActiveMode::Subject(sub.name.to_string()));
                                break;
                            }
                        }
                    }
                }
            }
        }
    });

    if *is_err.get() {
        render! {
            p { "{html}" }
        }
    } else {
        render! {
             div {
                dangerous_inner_html: "{html}"
            }
        }
    }
}

macro_rules! stream_try {
    ($next_state:ident, $($x:tt)*) => {
        match $($x)* {
            Ok(x) => x,
            Err(e) => return Some((Err(format!("{e:?}")), $next_state))
        }
    };
}
/// opens an editor and pipes updates for the edited text through the stream

#[derive(PersistentStruct)]
struct StreamState {
    last_edit: SystemTime,
    process: Child,
    path: String,
}

fn editor_stream(
    content: String,
) -> anyhow::Result<impl futures::Stream<Item = Result<String, String>>> {
    let tmp_file = tempfile::NamedTempFile::new().context("Creating Tempfile")?;
    let tmp_path = fs::canonicalize(tmp_file.into_temp_path())?;
    fs::write(&tmp_path, &content)?;
    trace!("Wrote {}", tmp_path.display());
    let path = tmp_path.to_str().unwrap().to_string();
    let process = Command::new("alacritty")
        .args(["-e", "helix", &path])
        .spawn()
        .context("Spawning Editor")?;

    // this is done so the original content is yielded in the beginning
    let last_edit = fs::metadata(&path)?.modified().unwrap() - Duration::from_secs(1);

    // The unfold function takes the stating state as first arg, and a gunction that gets the current state,
    // and yields a value for next() as well as the next state, or None, if it's over
    Ok(Box::pin(stream::unfold(
        StreamState {
            last_edit,
            process,
            path,
        },
        |mut state: StreamState| async move {
            // try stream is a custom question mark operator that return the results contents,
            // or yield an error message to the stream user. To do that, it needs the state arg
            // so this loop runs until the editor quits, in which case None is returned, which
            // closes the stream
            while stream_try!(
                state,
                state.process.try_wait().context("Checking editor process")
            )
            .is_none()
            {
                tokio::time::sleep(Duration::from_millis(10)).await;
                // get the last_modified ts
                let check_ts = stream_try!(
                    state,
                    fs::metadata(&state.path).context("Getting tmp file meta data")
                )
                .modified()
                .unwrap();
                // if it was modified, yield the content
                if check_ts > state.last_edit {
                    trace!("Textfile was updated");
                    return Some((
                        Ok(stream_try!(
                            state,
                            fs::read_to_string(&state.path).context("reading tmp file")
                        )),
                        state.with_last_edit(check_ts),
                    ));
                }
            }
            trace!("editor closed");
            None
        },
    )))
}
