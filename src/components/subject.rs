#![allow(non_snake_case)]
use crate::{comp_try, Schema, State};
use comrak::ComrakOptions;
use dioxus::prelude::*;

#[inline_props]
pub fn Subject(cx: Scope, name: String) -> Element {
    let state = use_shared_state::<State>(cx).unwrap();
    let db_path = state.read().db_path.clone().unwrap();
    let mut db = comp_try!(state, Schema::open(&db_path));
    let sub = comp_try!(state, db.get_sub_by_name(&name))
        .expect("Trying to display non existing subject");
    let html = comrak::markdown_to_html(&sub.description, &ComrakOptions::default());
    render! {
        div {
            dangerous_inner_html: "{html}"
        }
    }
}
