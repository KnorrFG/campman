#![allow(non_snake_case)]
use anyhow::Result;
use dioxus::prelude::{GlobalAttributes, *};
use proc_macros::b64_embed;

use crate::{attempt, components::BImg, ActiveMode, Mode, Schema, State};

const PERSON_ICON: &str = b64_embed!("assets/person_icon.png");
const LOCATION_ICON: &str = b64_embed!("assets/location_icon.png");
const GROUP_ICON: &str = b64_embed!("assets/group_icon.png");
const PENCIL_ICON: &str = b64_embed!("assets/pencil_icon.png");
const TRASH_ICON: &str = b64_embed!("assets/trash_icon.png");

#[derive(PartialEq)]
pub struct ResInfo {
    kind: ResKind,
    name: String,
}

#[derive(PartialEq)]
pub enum ResKind {
    Subject,
    Group,
    Place,
}

pub fn Search(cx: Scope) -> Element {
    let sterm = use_state(&cx, || "".to_string());
    let state = use_shared_state::<State>(&cx).unwrap();
    let sresults = use_state(&cx, || -> Vec<ResInfo> {
        if let Ok(r) = query_entries(&state.read().db_path.as_ref().unwrap(), "") {
            r
        } else {
            vec![]
        }
    });

    render!(
        div {
            // style: "outline: 2px solid black;",
            width: "100%",
            height: "100%",
            display: "flex",
            align_items: "stretch",
            justify_content: "stretch",
            flex_direction: "column",

            div {
                flex: "1 0 auto",
                position: "relative",

                input {
                    style: r#"
                        margin: 0;
                        position: absolute;
                        width: 80%;
                        height: 2em;
                        top: 50%;
                        left: 50%;
                        -ms-transform: translate(-50%, -50%);
                        transform: translate(-50%, -50%);
                        border-radius: 20px;
                        padding-left: 10px;
                    "#,
                    value: "{sterm}",
                    oninput: move |evt| {
                        sterm.set(evt.value.clone());
                        attempt!{ state {
                            Ok(sresults.set(query_entries(&state.read().db_path.as_ref().unwrap(), &evt.value)?))
                        }};
                    },
                }

            }

            div {
                flex: "6 0 auto",
                ul {
                    width: "70%",
                    margin: "auto",
                    overflow: "auto",

                    sresults.iter().map(|x| {
                        rsx!(SearchResult { result: x })
                    })
                }
            }


        }

    )
}

#[inline_props]
pub fn SearchResult<'a>(cx: Scope, result: &'a ResInfo) -> Element {
    let state = use_shared_state::<State>(&cx).unwrap();
    render! {
        div {
            padding: "5px",
            display: "flex",
            align_items: "center",
            gap: "5px",

            if result.kind == ResKind::Subject {
                rsx!{
                    div{
                        onclick: move |_| view_item(state, result),
                        BImg {
                            w: 30,
                            h: 30,
                            data: PERSON_ICON,
                            format: "png",
                        },
                    }
                }
            }

            div {
                flex: 1,
                onclick: move |_| view_item(state, result),
                style: "user-select: none;",
                result.name.clone()
            },

            div {
                onclick: move |_| {
                    assert!(result.kind == ResKind::Subject);
                    state.write().set_mode(Mode::EditingSubject(result.name.clone()))
                },
                BImg {
                    w: 30,
                    h: 30,
                    data: PENCIL_ICON,
                    format: "png",
                }
            },
            BImg {
                w: 30,
                h: 30,
                data: TRASH_ICON,
                format: "png",
            },
        }

    }
}

fn view_item(state: &UseSharedState<State>, info: &ResInfo) {
    assert!(info.kind == ResKind::Subject);
    state
        .write()
        .set_active_mode(ActiveMode::Subject(info.name.clone()));
}

fn query_entries(db_path: &str, query: &str) -> Result<Vec<ResInfo>> {
    let mut db = Schema::open(db_path)?;
    Ok(db
        .query_subj_names(query)?
        .into_iter()
        .map(|x| ResInfo {
            kind: ResKind::Subject,
            name: x,
        })
        .collect())
}
