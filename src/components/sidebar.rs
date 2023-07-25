#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use log::debug;

use crate::{components::button::SecondaryButton, ActiveMode, Mode, State};

pub fn Sidebar(cx: Scope) -> Element {
    let state = use_shared_state::<State>(cx).unwrap();
    cx.render(rsx!(div {
        style: "
            background-color: #26B3D1;
            padding-top: 15pt;
            padding-bottom: 15pt;
            padding-left: 10pt;
            padding-right: 10pt;
            display: flex;
            flex-direction: column;
            gap: 10px;
            align-items: center;
            width: 100%;
            height: 100%;
        ",
        NewButtons {},
        SecondaryButton {
            onclick: move |_| {
                debug!("Search Clicked");
                state.write().mode = Mode::Active(ActiveMode::Search);
            },
            "Search"
        },
        SecondaryButton { onclick: |_| {}, "Help" },
    }))
}

pub fn NewButtons(cx: Scope) -> Element {
    let state = use_shared_state::<State>(cx).unwrap();
    let style = r"
        outline: 2px solid black;
        background-color: #1D7C92;
        border-radius: 20px;
        padding: 0.5em;
        display: flex;
        flex-direction: column;
        gap: 10px;
        margin: 0pt;
    ";

    cx.render(rsx!(div {
        style: "{style}",
        p { style: "
                color: white;
                font-weight: bold;
                font-size: 20pt;
                margin: 0; ",
            "New",
         },
        SecondaryButton {
            onclick: move |_| {
                debug!("New Subject Clicked");
                state.write().mode = Mode::Active(ActiveMode::NewSubject);
            },
            "Subject" },
        SecondaryButton { onclick: |_| {}, "Group" },
        SecondaryButton { onclick: |_| {}, "Place"},
        SecondaryButton { onclick: |_| {}, "Event" },
    }))
}
