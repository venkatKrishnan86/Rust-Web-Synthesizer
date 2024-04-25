use yew::prelude::*;
use std::collections::HashMap;
use std::ops::Deref;
use stylist::{yew::styled_component, Style};
use crate::components::molecules::selector::Selector;
use crate::components::molecules::remove_button::RemoveButton;

use crate::components::atoms::keyboard_listener::KeyboardListener;

const OSCILLATOR_SELECT_CSS: &str = include_str!("../../UI_components/selectors/oscillator_selector.css");

#[derive(Properties, PartialEq)]
pub struct OscillatorSelectorProperties {
    pub mouse_down: Callback<(char, usize)>,
    pub mouse_up: Callback<(char, usize)>,
    pub number: usize
}

#[styled_component(OscillatorSelector)]
pub fn oscillator_selector(props: &OscillatorSelectorProperties) -> Html {
    let mouse_down = props.mouse_down.clone();
    let oscillator_select_style = Style::new(OSCILLATOR_SELECT_CSS).unwrap();
    let oscillator_class_hashmap = use_state(|| HashMap::from([
        ('1', "oscillator"),
        ('2', "oscillator"),
        ('3', "oscillator"),
        ('4', "oscillator"),
        ('5', "oscillator"),
        ('.', "container")
    ]));

    let cloned_oscillator_class_hashmap = oscillator_class_hashmap.clone();
    let number = props.number;

    let mouse_down = props.mouse_down.clone();
    // let mouse_down = Callback::from(move |event: MouseEvent| {
    //     if event.key() == "1" {
    //         let key_pressed = '1';
    //         let mut hashmap = cloned_oscillator_class_hashmap.deref().clone();
    //         let current_state = hashmap.get(&key_pressed).unwrap_or(&"oscillator"); // Using a reference to a literal
    //         let new_state = if *current_state == "oscillator" { "oscillator_active" } else { "oscillator" };
    //         hashmap.insert(key_pressed, new_state);
    //         cloned_oscillator_class_hashmap.set(hashmap);
    //         key_down.emit(key_pressed);
    //     }
    // });

    // let key_down = Callback::from(move |event: KeyboardEvent| {
    //     if event.key() == "1" {
    //         let key_pressed = '1';
    //         let mut hashmap = cloned_oscillator_class_hashmap.deref().clone();
    //         let current_state = hashmap.get(&key_pressed).unwrap_or(&"oscillator"); // Using a reference to a literal
    //         let new_state = if *current_state == "oscillator" { "oscillator_active" } else { "oscillator" };
    //         hashmap.insert(key_pressed, new_state);
    //         cloned_oscillator_class_hashmap.set(hashmap);
    //         key_down.emit(key_pressed);
    //     }
    // });
    
    // let key_up = Callback::from(move |event: KeyboardEvent| {
    // });
    
    html! {
        <>
        <h2>{"Oscillator "}{number}</h2>
        <div class={oscillator_select_style}>
            <Selector
                icon_class={oscillator_class_hashmap.deref()[&'1']} 
                label={('1', number)} 
                img_path={"https://i.ibb.co/XZWhWv5/Sine.png"} 
                is_active={false} 
                on_mouse_down={&mouse_down} 
                on_mouse_up={Callback::from(|_|{})}
            />
            <Selector
                icon_class={oscillator_class_hashmap.deref()[&'2']} 
                label={('2', number)} 
                img_path={"https://i.ibb.co/P1wjXPj/Square.png"} 
                is_active={false} 
                on_mouse_down={&mouse_down} 
                on_mouse_up={Callback::from(|_|{})}
            />
            <Selector
                icon_class={oscillator_class_hashmap.deref()[&'3']} 
                label={('3', number)} 
                img_path={"https://i.ibb.co/VSSfGGZ/Sawtooth.png"} 
                is_active={false} 
                on_mouse_down={&mouse_down} 
                on_mouse_up={Callback::from(|_|{})}
            />
            <Selector
                icon_class={oscillator_class_hashmap.deref()[&'4']} 
                label={('4', number)} 
                img_path={"https://i.ibb.co/thqmPmZ/Triangle.png"} 
                is_active={false} 
                on_mouse_down={&mouse_down} 
                on_mouse_up={Callback::from(|_|{})}
            />
            <Selector
                icon_class={oscillator_class_hashmap.deref()[&'5']} 
                label={('5', number)} 
                img_path={"https://i.ibb.co/VxRNs6g/Noise.png"} 
                is_active={false} 
                on_mouse_down={&mouse_down} 
                on_mouse_up={Callback::from(|_|{})}
            />
            <RemoveButton 
                on_mouse_down={&mouse_down} 
                on_mouse_up={Callback::from(|_|{})}
                number = {number}
            />
        </div>
        </>
    }
}