use std::collections::HashMap;
use std::hash;
use std::ops::Deref;

use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use components::molecules::keys::{Key, KeyColor};
use components::molecules::selector::Selector;
use components::atoms::keyboard_listener::KeyboardListener;
use components::atoms::button::CustomButton;
use components::atoms::icon::CustomIcon;

pub mod components;

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

    let class_hashmap = use_state(|| HashMap::from([
        ('A', "keycodes"),
        ('W', "keycodes"),
        ('S', "keycodes"),
        ('E', "keycodes"),
        ('D', "keycodes"),
        ('F', "keycodes"),
        ('T', "keycodes"),
        ('G', "keycodes"),
        ('Y', "keycodes"),
        ('H', "keycodes"),
        ('U', "keycodes"),
        ('J', "keycodes"),
        ('K', "keycodes"),
    ]));

    let octave_class_hashmap = use_state(|| HashMap::from([('Z', "octave_change"), ('X', "octave_change")]));

    let mouse_down = props.mouse_down.clone();
    let mouse_up = props.mouse_up.clone();

    let key_down = props.key_down.clone();
    let cloned_class_hashmap = class_hashmap.clone();
    let cloned_octave_class_hashmap = octave_class_hashmap.clone();
    let key_down = Callback::from(move |event: KeyboardEvent| {
        let key_pressed= char::from_u32(event.key_code()).unwrap_or('a');
        let mut hashmap = cloned_class_hashmap.deref().clone();
        if hashmap.contains_key(&key_pressed){
            hashmap.insert(key_pressed, "keycodes_active");
            cloned_class_hashmap.set(hashmap);
        } else {
            hashmap = cloned_octave_class_hashmap.deref().clone();
            if hashmap.contains_key(&key_pressed){
                hashmap.insert(key_pressed, "octave_change_active");
            }
            cloned_octave_class_hashmap.set(hashmap)
        } 
        key_down.emit(key_pressed);
    });
    
    let key_up = props.key_up.clone();
    let cloned_class_hashmap = class_hashmap.clone();
    let cloned_octave_class_hashmap = octave_class_hashmap.clone();
    let key_up = Callback::from(move |event: KeyboardEvent| {
        let key_pressed= char::from_u32(event.key_code()).unwrap_or('a');
        let mut hashmap = cloned_class_hashmap.deref().clone();
        if hashmap.contains_key(&key_pressed){
            hashmap.insert(key_pressed, "keycodes");
            cloned_class_hashmap.set(hashmap);
        } else {
            hashmap = cloned_octave_class_hashmap.deref().clone();
            if hashmap.contains_key(&key_pressed){
                hashmap.insert(key_pressed, "octave_change");
            }
            cloned_octave_class_hashmap.set(hashmap)
        }
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
                <Key button_class={class_hashmap.deref()[&'W']} label='W' key_color={KeyColor::Black} on_mouse_down={&mouse_down} on_mouse_up={&mouse_up}/>
                <Key button_class={class_hashmap.deref()[&'E']} label='E' key_color={KeyColor::Black} on_mouse_down={&mouse_down} on_mouse_up={&mouse_up}/>
                <div class="filler"></div>
                <div id="corner-left" class="filler"></div>
                <Key button_class={class_hashmap.deref()[&'T']} label='T' key_color={KeyColor::Black} on_mouse_down={&mouse_down} on_mouse_up={&mouse_up}/>
                <Key button_class={class_hashmap.deref()[&'Y']} label='Y' key_color={KeyColor::Black} on_mouse_down={&mouse_down} on_mouse_up={&mouse_up}/>
                <Key button_class={class_hashmap.deref()[&'U']} label='U' key_color={KeyColor::Black} on_mouse_down={&mouse_down} on_mouse_up={&mouse_up}/>
                <div class="filler"></div>
                <div id="corner-left" class="filler"></div>
                <div id="corner-right" class="filler"></div>
            </div>
            <div class={white_keys_style}>
                <Key button_class={class_hashmap.deref()[&'A']} label={'A'} key_color={KeyColor::White} on_mouse_down={&mouse_down} on_mouse_up= {&mouse_up} />
                <Key button_class={class_hashmap.deref()[&'S']} label={'S'} key_color={KeyColor::White} on_mouse_down={&mouse_down} on_mouse_up= {&mouse_up} />
                <Key button_class={class_hashmap.deref()[&'D']} label={'D'} key_color={KeyColor::White} on_mouse_down={&mouse_down} on_mouse_up= {&mouse_up} />
                <Key button_class={class_hashmap.deref()[&'F']} label={'F'} key_color={KeyColor::White} on_mouse_down={&mouse_down} on_mouse_up= {&mouse_up} />
                <Key button_class={class_hashmap.deref()[&'G']} label={'G'} key_color={KeyColor::White} on_mouse_down={&mouse_down} on_mouse_up= {&mouse_up} />
                <Key button_class={class_hashmap.deref()[&'H']} label={'H'} key_color={KeyColor::White} on_mouse_down={&mouse_down} on_mouse_up= {&mouse_up} />
                <Key button_class={class_hashmap.deref()[&'J']} label={'J'} key_color={KeyColor::White} on_mouse_down={&mouse_down} on_mouse_up= {&mouse_up} />
                <Key button_class={class_hashmap.deref()[&'K']} label={'K'} key_color={KeyColor::White} on_mouse_down={&mouse_down} on_mouse_up= {&mouse_up} />
            </div>
            <div class={octave_change_style}>
                <CustomButton 
                    class={octave_class_hashmap.deref()[&'Z']}
                    label={"Z"}
                    mouse_down={&octave_down_mouse_down}
                    mouse_up={&None}
                />
                <CustomButton 
                    class={octave_class_hashmap.deref()[&'X']}
                    label={"X"}
                    mouse_down={&octave_up_mouse_down}
                    mouse_up={&None}
                />
            </div>
        </>
    }
}

#[styled_component(VolumeBar)]
pub fn volume_bar(props: &MIDIKeyboardProperties) -> Html {
    html! {
        <div>
        // html for hello world
            <h1>{"Hello World hello"}</h1>
        </div>
    }
}
