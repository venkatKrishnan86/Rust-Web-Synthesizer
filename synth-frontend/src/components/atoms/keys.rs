use stylist::yew::styled_component;
use yew::prelude::*;

use crate::MIDIKeyboardProperties;

#[derive(PartialEq)]
pub enum KeyColor {
    White,
    Black
}

impl KeyColor {
    pub fn to_string(&self) -> String {
        match self {
            Self::White => "whitekey".to_owned(),
            Self::Black => "blackkey".to_owned(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct KeyProps {
    pub label: char,
    pub key_color: KeyColor,
    pub on_mouse_down: Callback<()>,
    pub on_mouse_up: Callback<()>,
}

#[styled_component(Key)]
pub fn key(props: &KeyProps) -> Html {
    let mouse_down = props.on_mouse_down.clone();
    let mouse_down = Callback::from(move |_| {
        mouse_down.emit(());
    });

    let mouse_up = props.on_mouse_up.clone();
    let mouse_up = Callback::from(move |_| {
        mouse_up.emit(());
    });

    html! {
        <div class = {&props.key_color.to_string()}>
            <button class = "keycodes" onmousedown={&mouse_down} onmouseup={&mouse_up}>{props.label}</button>
        </div>
    }
}


pub fn create_white_keys(props: &MIDIKeyboardProperties) -> Vec<Html> {
    let mut keys = Vec::new();
    let keycodes = vec!['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K'];
    let mouse_down = props.mouse_down.clone();
    let mouse_down = Callback::from(move |_| {
        mouse_down.emit(());
    });

    let mouse_up = props.mouse_up.clone();
    let mouse_up = Callback::from(move |_| {
        mouse_up.emit(());
    });

    for index in 0..8{
        keys.push(html! {
            <Key label={keycodes[index]} key_color={KeyColor::White} on_mouse_down={&mouse_down} on_mouse_up={&mouse_up}/>
        })
    }
    keys
}