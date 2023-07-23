#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{Mode, State};

pub fn NewSubject(cx: Scope) -> Element {
    let state = use_shared_state::<State>(cx).unwrap();
    render! {
        form {
            width: "100%",
            height: "100%",
            margin: 0,
            padding: 0,
            display: "flex",
            flex_direction: "row",
            align_items: "center",
            justify_content: "center",
            gap: "1em",
            onsubmit: move |ev| {
                state.write().mode = Mode::EditingSubject(ev.data.values["subject_name"][0].to_string());
            },
            label {
                "SubjectId"
            },
            input { name: "subject_name" },
            input { r#type: "submit" },
        }
    }
}
