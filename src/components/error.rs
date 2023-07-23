#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{Mode, State};

pub fn Error(cx: Scope) -> Element {
    let state = &*use_shared_state::<State>(cx).unwrap();
    let Mode::Error { err, parent } = state.read().mode.clone() else {
        panic!("Error component used without being in Error State");
    };
    render! {
        h1 { "Error" },
        p { err.as_str() },
        button {
            onclick: move |_| { state.write().mode = (*parent).clone()},
            "Ok"
        }
    }
}
