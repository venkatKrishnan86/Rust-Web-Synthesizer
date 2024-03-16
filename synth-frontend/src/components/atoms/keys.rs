use gloo::console::log;
use stylist::yew::styled_component;
use yew::prelude::*;

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
    pub on_mouse_click: Callback<()>
}

#[styled_component(Key)]
pub fn key(props: &KeyProps) -> Html {
    let mouse_click = props.on_mouse_click.clone();
    let mouse_click = Callback::from(move |_| {
        mouse_click.emit(());
    });
    html! {
        <div class = {&props.key_color.to_string()}>
            <button class = "keycodes" onclick={&mouse_click}>{props.label}</button>
        </div>
    }
}

pub fn create_white_keys() -> Vec<Html> {
    let mut keys = Vec::new();
    let keycodes = vec!['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K'];
    let mouse_click = Callback::from(move |_| {
        log!("Clicked white key");
    });

    for index in 0..8{
        keys.push(html! {
            <Key label={keycodes[index]} key_color={KeyColor::White} on_mouse_click={&mouse_click}/>
        })
    }
    keys
}