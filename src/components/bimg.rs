#![allow(non_snake_case)]
use dioxus::prelude::*;

#[inline_props]
pub fn BImg(cx: Scope, data: &'static str, format: &'static str, w: i64, h: i64) -> Element {
    render! {
        img {
            src: "data:image/{format};base64, {data}",
            width: *w,
            height: *h,
        }
    }
}
