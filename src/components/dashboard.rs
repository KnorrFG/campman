#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use log::debug;

use crate::components::PrimaryButton;
use crate::{attempt, ActiveMode, Mode, Schema, State};

pub fn Dashboard(cx: Scope) -> Element {
    let state = use_shared_state::<State>(cx).unwrap();
    let state2 = state.clone();
    render! {
        div {
            width: "100%",
            height: "100%",
            display: "flex",
            justify_content: "center",
            flex_direction: "row",
            align_items: "center",

            PrimaryButton {
                onclick: move |_| {
                    debug!("load clicked");
                    attempt!{ state {
                        let file = {
                            let user_dirs = &state.read().user_dirs;
                            get_db_file(user_dirs)
                        };
                        if let Some(p) = file {
                            Schema::open(&p)?;
                            let mut state = state.write();
                            state.mode = Mode::Active(ActiveMode::Events);
                            state.db_path = Some(p);
                        }
                        Ok(())
                    }}
                },
                "load"
            },

            div {
                min_width: "5em"
            },

            PrimaryButton {
                onclick: move |_| {
                    debug!("new clicked");
                    attempt!{ state2 {
                        if let Some(p) = get_new_db_file(&state2.read().user_dirs){
                            Schema::create_new(&p)?;
                            let mut state2 = state2.write();
                            state2.mode = Mode::Active(ActiveMode::Events);
                            state2.db_path = Some(p);
                        }
                        Ok(())
                    }}
                },
                "new"
            },
        }
    }
}

fn get_new_db_file(user_dirs: &directories::UserDirs) -> Option<String> {
    rfd::FileDialog::new()
        .add_filter("Campman DB", &["db"])
        .set_directory(user_dirs.home_dir())
        .set_file_name("my_campaign.db")
        .save_file()
        .map(|p| p.display().to_string())
}

fn get_db_file(user_dirs: &directories::UserDirs) -> Option<String> {
    rfd::FileDialog::new()
        .add_filter("Campman DB", &["db"])
        .set_directory(user_dirs.home_dir())
        .pick_file()
        .map(|p| p.display().to_string())
}
