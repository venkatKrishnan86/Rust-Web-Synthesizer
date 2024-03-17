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
    pub on_mouse_down: Callback<char>,
    pub on_mouse_up: Callback<char>,
    pub on_key_down: Callback<char>,
    pub on_key_up: Callback<char>,
}

#[styled_component(Key)]
pub fn key(props: &KeyProps) -> Html {
    let mouse_down = props.on_mouse_down.clone();
    let label = props.label.clone();
    let mouse_down = Callback::from(move |_| {
        mouse_down.emit(label);
    });

    let mouse_up = props.on_mouse_up.clone();
    let label = props.label.clone();
    let mouse_up = Callback::from(move |_| {
        mouse_up.emit(label);
    });

    let key_down = props.on_key_down.clone();
    let key_down = Callback::from(move |event: KeyboardEvent| {
        let target= char::from_u32(event.key_code()).unwrap_or('a');
        key_down.emit(target);
    });

    let key_up = props.on_key_up.clone();
    let key_up = Callback::from(move |event: KeyboardEvent| {
        let target= char::from_u32(event.key_code()).unwrap_or('a');
        key_up.emit(target);
    });

    html! {
        <div class = {&props.key_color.to_string()}>
            <button class = "keycodes" onmousedown={&mouse_down} onmouseup={&mouse_up} onkeydown={&key_down} onkeyup={&key_up}>{props.label}</button>
        </div>
    }
}


pub fn create_white_keys(props: &MIDIKeyboardProperties) -> Vec<Html> {
    let mut keys = Vec::new();
    let keycodes = vec!['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K'];

    for index in 0..8{
        let mouse_down = props.mouse_down.clone();
        let mouse_up = props.mouse_up.clone();
        let mouse_down = Callback::from(move |label: char| {
            mouse_down.emit(label)
        });
        let mouse_up = Callback::from(move |label: char| {
            mouse_up.emit(label)
        });

        let key_down = props.key_down.clone();
        let key_up = props.key_up.clone();
        let key_down = Callback::from(move |label: char| {
            key_down.emit(label)
        });
        let key_up = Callback::from(move |label: char| {
            key_up.emit(label)
        });
        keys.push(html! {
            <Key label={&keycodes[index]} key_color={KeyColor::White} on_mouse_down={&mouse_down} on_mouse_up= {&mouse_up} on_key_down={&key_down} on_key_up={&key_up}/>
        })
    }
    keys
}