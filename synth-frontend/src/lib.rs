use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use components::molecules::keys::{Key, KeyColor};
use components::atoms::keyboard_listener::KeyboardListener;
use components::atoms::button::CustomButton;

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
    let key_down = Callback::from(move |event: KeyboardEvent| {
        let key_pressed= char::from_u32(event.key_code()).unwrap_or('a');
        key_down.emit(key_pressed);
    });
    let key_up = props.key_up.clone();
    let key_up = Callback::from(move |event: KeyboardEvent| {
        let key_pressed= char::from_u32(event.key_code()).unwrap_or('a');
        key_up.emit(key_pressed);
    });

    let octave_down_mouse_down = props.mouse_down.clone();
    let octave_down_mouse_down = Callback::from(move |_| {
        octave_down_mouse_down.emit('Z')
    });
    let octave_up_mouse_down = props.mouse_down.clone();
    let octave_up_mouse_down = Callback::from(move |_| {
        octave_up_mouse_down.emit('X')
    });
    html! {
        <>
            <KeyboardListener key_down={&key_down} key_up={&key_up}/>
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
                <Key label={'A'} key_color={KeyColor::White} on_mouse_down={&mouse_down} on_mouse_up= {&mouse_up} />
                <Key label={'S'} key_color={KeyColor::White} on_mouse_down={&mouse_down} on_mouse_up= {&mouse_up} />
                <Key label={'D'} key_color={KeyColor::White} on_mouse_down={&mouse_down} on_mouse_up= {&mouse_up} />
                <Key label={'F'} key_color={KeyColor::White} on_mouse_down={&mouse_down} on_mouse_up= {&mouse_up} />
                <Key label={'G'} key_color={KeyColor::White} on_mouse_down={&mouse_down} on_mouse_up= {&mouse_up} />
                <Key label={'H'} key_color={KeyColor::White} on_mouse_down={&mouse_down} on_mouse_up= {&mouse_up} />
                <Key label={'J'} key_color={KeyColor::White} on_mouse_down={&mouse_down} on_mouse_up= {&mouse_up} />
                <Key label={'K'} key_color={KeyColor::White} on_mouse_down={&mouse_down} on_mouse_up= {&mouse_up} />
            </div>
            <div class={octave_change_style}>
                <CustomButton 
                    class={"octave_change"}
                    label={"Z"} 
                    is_active={false}
                    mouse_down={&octave_down_mouse_down}
                    mouse_up={&None}
                />
                <CustomButton 
                    class={"octave_change"}
                    label={"X"} 
                    is_active={false}
                    mouse_down={&octave_up_mouse_down}
                    mouse_up={&None}
                />
            </div>
        </>
    }
}

