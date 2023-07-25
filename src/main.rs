#![allow(non_snake_case)]
use std::process::exit;

use dioxus::prelude::*;
use dioxus_desktop::{use_window, Config, WindowBuilder};

use campman::{
    components::{self, PrimaryButton},
    ActiveMode, Mode, Schema, State,
};

fn main() {
    // launch the dioxus app in a webview
    pretty_env_logger::init();
    dioxus_desktop::launch_cfg(
        App,
        Config::new().with_window(WindowBuilder::new().with_resizable(true)),
    );
}

fn App(cx: Scope) -> Element {
    // if you move those. There will be a lot of panic all over the program.
    // all use_shared_state() calls immediately use unwrap()
    use_shared_state_provider(cx, || State {
        mode: Mode::Dashboard,
        db_path: None,
        user_dirs: directories::UserDirs::new().expect("Couldn't find home dir"),
    });
    let state = use_shared_state::<State>(cx).unwrap();
    let win = use_window(&cx);

    render! {
        div {
            width: "{win.inner_size().width}px",
            height: "{win.inner_size().height}px",
            style {"
                * {{
                    margin: 0;
                    padding: 0;
                    font-family: 'Arial', sans-serif;
                    user-select: none;
                    transition: .2s all;
                }}"},
            Main { mode: state.read().mode.clone() }
        }

    }
}

#[inline_props]
fn Main(cx: Scope, mode: Mode) -> Element {
    match mode {
        Mode::Dashboard => {
            render! { components::Dashboard {} }
        }
        Mode::Error { .. } => {
            render! { components::Error {} }
        }
        Mode::Active(sub_mode) => {
            let Child = match sub_mode {
                ActiveMode::Events => render! { components::Events {} },
                ActiveMode::Search => render! { components::Search {} },
                ActiveMode::NewSubject => render! { components::NewSubject {} },
                ActiveMode::Subject(name) => render! { components::Subject {name: name.clone()} },
            };

            render! {
                TwoColLayout {
                    sidebar: render!(components::Sidebar {}),
                    main: Child
                }
            }
        }
        Mode::Fatal(s) => {
            render! {
                h1 { "Error" },
                p {"{s}"},
                PrimaryButton {
                    onclick: |_| { exit(1) },
                    "Exit"
                }
            }
        }
        Mode::EditingSubject(name) => {
            render! { components::EditingSubject {name: name.clone()} }
        }
    }
}

#[inline_props]
pub fn TwoColLayout<'a>(cx: Scope<'a>, sidebar: Element<'a>, main: Element<'a>) -> Element<'a> {
    render! {
         div {
            // style: "outline: 2px solid black;",
            display: "flex",
            align_items: "stretch",
            justify_items: "stretch",
            width: "100%",
            height: "100%",
            div {
                // style: "outline: 2px solid red;",
                flex: "0 1 15%",
                max_width: "12em",
                sidebar
            },
            div {
                // style: "outline: 2px solid black;",
                flex: 1,
                overflow: "auto",
                main
            }
        }
    }
}
