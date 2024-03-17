use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use components::atoms::keys::{Key, KeyColor, create_white_keys};

mod components;

const WHITE_KEYS_CSS: &str = include_str!("UI_components/keys/white_keys.css");
const BLACK_KEYS_CSS: &str = include_str!("UI_components/keys/black_keys.css");

#[derive(Properties, PartialEq)]
pub struct MIDIKeyboardProperties {
    pub mouse_down: Callback<char>,
    pub mouse_up: Callback<char>,
}

#[styled_component(MIDIKeyboard)]
pub fn midi_keyboard(props: &MIDIKeyboardProperties) -> Html {
    let white_keys_style = Style::new(WHITE_KEYS_CSS).unwrap();
    let black_keys_style = Style::new(BLACK_KEYS_CSS).unwrap();

    let mouse_down = props.mouse_down.clone();
    let mouse_up = props.mouse_up.clone();
    html! {
        <>
            <div class={black_keys_style}>
                <div id="corner-left" class="filler" ></div>
                <Key label='W' key_color={KeyColor::Black} on_mouse_down={&mouse_down} on_mouse_up={&mouse_up}/>
                <Key label='E' key_color={KeyColor::Black} on_mouse_down={&mouse_down} on_mouse_up={&mouse_up}/>
                <div class="filler"></div>
                <div id="corner-left" class="filler"></div>
                <Key label='T' key_color={KeyColor::Black} on_mouse_down={&mouse_down} on_mouse_up={&mouse_up}/>
                <Key label='Y' key_color={KeyColor::Black} on_mouse_down={&mouse_down} on_mouse_up={&mouse_up}/>
                <Key label='U' key_color={KeyColor::Black} on_mouse_down={&mouse_down} on_mouse_up={&mouse_up}/>
                <div class="filler"></div>
                <div id="corner-left" class="filler"></div>
                <div id="corner-right" class="filler"></div>
            </div>
            <div class={white_keys_style}>
                {create_white_keys(props)}
            </div>
        </>
    }
}

