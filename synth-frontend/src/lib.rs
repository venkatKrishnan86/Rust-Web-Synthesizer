use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use components::molecules::keys::{Key, KeyColor, create_white_keys};

mod components;

const WHITE_KEYS_CSS: &str = include_str!("UI_components/keys/white_keys.css");
const BLACK_KEYS_CSS: &str = include_str!("UI_components/keys/black_keys.css");
const OCTAVE_CHANGE_CSS: &str = include_str!("UI_components/key_controllers/octave_change.css");

#[derive(Properties, PartialEq)]
pub struct MIDIKeyboardProperties {
    pub mouse_down: Callback<char>,
    pub mouse_up: Callback<char>,
    pub key_down: Callback<char>,
    pub key_up: Callback<char>,
}

#[styled_component(MIDIKeyboard)]
pub fn midi_keyboard(props: &MIDIKeyboardProperties) -> Html {
    let white_keys_style = Style::new(WHITE_KEYS_CSS).unwrap();
    let black_keys_style = Style::new(BLACK_KEYS_CSS).unwrap();
    let octave_change_style = Style::new(OCTAVE_CHANGE_CSS).unwrap();

    let mouse_down = props.mouse_down.clone();
    let mouse_up = props.mouse_up.clone();
    let key_down = props.key_down.clone();
    let key_up = props.key_up.clone();
    html! {
        <>
            <div class={black_keys_style}>
                <div id="corner-left" class="filler" ></div>
                <Key label='W' key_color={KeyColor::Black} on_mouse_down={&mouse_down} on_mouse_up={&mouse_up} on_key_down={&key_down} on_key_up={&key_up}/>
                <Key label='E' key_color={KeyColor::Black} on_mouse_down={&mouse_down} on_mouse_up={&mouse_up} on_key_down={&key_down} on_key_up={&key_up}/>
                <div class="filler"></div>
                <div id="corner-left" class="filler"></div>
                <Key label='T' key_color={KeyColor::Black} on_mouse_down={&mouse_down} on_mouse_up={&mouse_up} on_key_down={&key_down} on_key_up={&key_up}/>
                <Key label='Y' key_color={KeyColor::Black} on_mouse_down={&mouse_down} on_mouse_up={&mouse_up} on_key_down={&key_down} on_key_up={&key_up}/>
                <Key label='U' key_color={KeyColor::Black} on_mouse_down={&mouse_down} on_mouse_up={&mouse_up} on_key_down={&key_down} on_key_up={&key_up}/>
                <div class="filler"></div>
                <div id="corner-left" class="filler"></div>
                <div id="corner-right" class="filler"></div>
            </div>
            <div class={white_keys_style}>
                {create_white_keys(props)}
            </div>
            <div class={octave_change_style}>
                <button>{"Z"}</button>
                <button>{"X"}</button>
            </div>
        </>
    }
}

