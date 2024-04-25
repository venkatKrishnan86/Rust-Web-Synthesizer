use yew::prelude::*;
use std::collections::HashMap;
use std::ops::Deref;
use stylist::{yew::styled_component, Style};
use crate::components::atoms::slider::{Slider, IntSlider};
use crate::components::molecules::multi_selector::MultiSelector;
use crate::components::molecules::remove_button::RemoveButton;

/// CSS for the oscillator selector component.
const OSCILLATOR_SELECT_CSS: &str = include_str!("../../UI_components/selectors/oscillator_selector.css");

/// Properties for the `OscillatorSelector` component.
#[derive(Properties, PartialEq)]
pub struct OscillatorSelectorProperties {
    /// Callback invoked when the mouse button is pressed down on the component.
    pub mouse_down: Callback<(char, usize)>,
    /// Callback invoked when the mouse button is released on the component.
    pub mouse_up: Callback<(char, usize)>,
    pub gain_change: Callback<f64>,
    pub detune_change: Callback<i8>,
    pub gain: f64,
    pub detune: i8,
    /// The number of the oscillator.
    pub number: usize,
    /// The index of the active item in the multi-selector.
    pub active_index: usize,
}

/// The `oscillator_selector` component represents an oscillator selector with a multi-selector and a remove button.
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
    let labels = vec![
        ('1', number),
        ('2', number),
        ('3', number),
        ('4', number),
        ('5', number)
    ];
    let images = vec![
        "https://i.ibb.co/XZWhWv5/Sine.png".to_owned(),
        "https://i.ibb.co/P1wjXPj/Square.png".to_owned(),
        "https://i.ibb.co/NLj5Szb/saw-tooth.png".to_owned(),
        "https://i.ibb.co/thqmPmZ/Triangle.png".to_owned(),
        "https://i.ibb.co/VxRNs6g/Noise.png".to_owned()
    ];
    html! {
        <>
        // <h2>{"Oscillator "}{number}</h2>
        <div class={oscillator_select_style}>
            <MultiSelector
                icon_class={"oscillator"} 
                label={labels} 
                img_path={images} 
                is_active={false} 
                active_index={props.active_index}
                on_mouse_down={&mouse_down} 
                on_mouse_up={Callback::from(|_|{})}
            />
            <Slider 
                label={"Gain"}
                value={props.gain}
                onchange={props.gain_change.clone()}
                precision={Some(2)}
                percentage={false}
                min={0.0}
                max={1.0}
                step={Some(0.01)}
            />
            <IntSlider 
                label={"Detune"}
                value={props.detune}
                onchange={props.detune_change.clone()}
                precision={Some(0)}
                percentage={false}
                min={-12}
                max={12}
                step={Some(1)}
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