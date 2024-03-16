use gloo::console::log;
use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use components::atoms::keys::{Key, KeyColor, create_white_keys};

mod components;

const WHITE_KEYS_CSS: &str = include_str!("UI_components/keys/white_keys.css");
const BLACK_KEYS_CSS: &str = include_str!("UI_components/keys/black_keys.css");

#[derive(Properties, PartialEq)]
pub struct MIDIKeyboardProperties {
    pub mouse_click: Callback<()>
}

#[styled_component(MIDIKeyboard)]
pub fn midi_keyboard(props: &MIDIKeyboardProperties) -> Html {
    let white_keys_style = Style::new(WHITE_KEYS_CSS).unwrap();
    let black_keys_style = Style::new(BLACK_KEYS_CSS).unwrap();

    let mouse_click = props.mouse_click.clone();
    let mouse_click = Callback::from(move |_| {
        mouse_click.emit(());
    });
    html! {
        <>
            <div class={black_keys_style}>
                <div id="corner-left" class="filler" ></div>
                <Key label='W' key_color={KeyColor::Black} on_mouse_click={&mouse_click}/>
                <Key label='E' key_color={KeyColor::Black} on_mouse_click={&mouse_click}/>
                <div class="filler"></div>
                <div id="corner-left" class="filler"></div>
                <Key label='T' key_color={KeyColor::Black} on_mouse_click={&mouse_click}/>
                <Key label='Y' key_color={KeyColor::Black} on_mouse_click={&mouse_click}/>
                <Key label='U' key_color={KeyColor::Black} on_mouse_click={&mouse_click}/>
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

