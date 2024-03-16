use gloo::console::log;
use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use components::atoms::{black_keys::BlackKey, white_keys::create_white_keys};

mod components;

const WHITE_KEYS_CSS: &str = include_str!("UI_components/keys/white_keys.css");
const BLACK_KEYS_CSS: &str = include_str!("UI_components/keys/black_keys.css");

#[derive(Properties, PartialEq)]
pub struct MIDIKeyboardProperties {
    pub mouse_click: (char, Callback<()>)
}

#[styled_component(MIDIKeyboard)]
pub fn midi_keyboard() -> Html {
    let white_keys_style = Style::new(WHITE_KEYS_CSS).unwrap();
    let black_keys_style = Style::new(BLACK_KEYS_CSS).unwrap();
    let mouse_click = Callback::from(move |_| {
        log!("Clicked black key");
    });
    html! {
        <>
            <div class={black_keys_style}>
                <div id="corner-left" class="filler" ></div>
                <BlackKey label='W' on_mouse_click={&mouse_click}/>
                <BlackKey label='E' on_mouse_click={&mouse_click}/>
                <div class="filler"></div>
                <div id="corner-left" class="filler"></div>
                <BlackKey label='T' on_mouse_click={&mouse_click}/>
                <BlackKey label='Y' on_mouse_click={&mouse_click}/>
                <BlackKey label='U' on_mouse_click={&mouse_click}/>
                <div class="filler"></div>
                <div id="corner-left" class="filler"></div>
                <div id="corner-right" class="filler"></div>
            </div>
            <div class={white_keys_style}>
                {create_white_keys()}
            </div>
        </>
    }
}

