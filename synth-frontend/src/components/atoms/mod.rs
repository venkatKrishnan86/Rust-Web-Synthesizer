pub mod white_keys;
pub mod black_keys;

use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Properties, PartialEq)]
pub struct KeyProps {
    pub label: char,
    pub on_mouse_click: Callback<()>
}

#[styled_component(Key)]
pub fn key(props: &KeyProps) -> Html {
    let mouse_click = props.on_mouse_click.clone();
    let mouse_click = Callback::from(move |_| {
        mouse_click.emit(());
    });
    html! {
        <div class = "whitekey">
            <button class = "keycodes" onclick={&mouse_click}>{props.label}</button>
        </div>
    }
}