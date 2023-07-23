#![allow(non_snake_case)]
use dioxus::prelude::*;

use super::color;

#[inline_props]
fn RawButton<'a>(
    cx: Scope<'a>,
    id: &'a str,
    main_color: &'a str,
    pressed_color: &'a str,
    onclick: EventHandler<'a, MouseEvent>,
    children: Element<'a>,
) -> Element<'a> {
    render! {
         div{
            style {"
                 button.{id} {{
                    border: 1px solid black;
                    background-color: {main_color};
                    border-radius: 50px;
                    width: 8em;
                    height: 2.5em;
                    font-size: 12pt;
                }}   
                button.{id}:active {{
                    background-color: {pressed_color};
                }}
            "},
             button {
                class: "{id}",
                onclick: move |evt| onclick.call(evt),
                children
             }
         }

    }
}

#[inline_props]
pub fn PrimaryButton<'a>(
    cx: Scope<'a>,
    onclick: EventHandler<'a, MouseEvent>,
    children: Element<'a>,
) -> Element<'a> {
    render! {
        RawButton{
            id: "prim",
            main_color: color::PRIMARY,
            pressed_color: color::SECONDARY,
            onclick: move |evt| onclick.call(evt),
            children,
        }
    }
}

#[inline_props]
pub fn SecondaryButton<'a>(
    cx: Scope<'a>,
    onclick: EventHandler<'a, MouseEvent>,
    children: Element<'a>,
) -> Element<'a> {
    render! {
        RawButton{
            id: "sec",
            main_color: color::WHITE,
            pressed_color: color::GREY,
            onclick: move |evt| onclick.call(evt),
            children,
        }
    }
}
