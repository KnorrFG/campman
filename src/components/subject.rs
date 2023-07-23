#![allow(non_snake_case)]
use dioxus::prelude::*;

#[inline_props]
pub fn Subject(cx: Scope, id: String) -> Element {
    render! {
        "Here be a subject"
    }
}
