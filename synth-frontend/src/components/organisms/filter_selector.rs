use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use crate::components::molecules::multi_selector::MultiSelector;
use crate::components::atoms::slider::Slider;
use std::collections::HashMap;

/// CSS for the oscillator selector component.
const OSCILLATOR_SELECT_CSS: &str = include_str!("../../UI_components/selectors/oscillator_selector.css");

/// Properties for the `FilterSelector` component.
#[derive(Properties, PartialEq)]
pub struct FilterSelectorProperties {
     /// Callback invoked when the mouse button is pressed down on the component.
    pub mouse_down: Callback<(char, usize)>,
    /// Callback invoked when the mouse button is released on the component.
    pub mouse_up: Callback<(char, usize)>,
    /// Callback invoked when the frequency value changes.
    pub freq_change: Callback<f64>,
    /// The frequency value.
    pub freq: f64,
    /// The index of the active item in the multi-selector.
    pub active_index: usize
}

/// The `filter_selector` component represents a filter selector with a multi-selector and a frequency slider.
#[styled_component(FilterSelector)]
pub fn filter_selector(props: &FilterSelectorProperties) -> Html {
    let overall_css = Style::new(OSCILLATOR_SELECT_CSS).unwrap();
    let mouse_down = props.mouse_down.clone();
    let freq_change = props.freq_change.clone();
    let oscillator_class_hashmap = use_state(|| HashMap::from([
        ('0', "oscillator"),
        ('9', "oscillator"),
        ('8', "oscillator"),
        ('.', "container")
    ]));

    let labels = vec![
        ('7', 0),
        ('0', 0),
        ('9', 0),
        ('8', 0),
    ];
    let images = vec![
        "".to_owned(),
        "https://i.ibb.co/HFCG4h1/HighPass.png".to_owned(),
        "https://i.ibb.co/23gfy5Q/BandPass.png".to_owned(),
        "https://i.ibb.co/SQXCXkf/LowPass.png".to_owned(),
    ];

    let cloned_oscillator_class_hashmap = oscillator_class_hashmap.clone();

    html! {
        <div class={overall_css}>
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
            label={"Frequency"}
            value={props.freq}
            onchange={freq_change}
            precision={Some(1)}
            percentage={false}
            min={20.0}
            max={20000.0}
            step={Some(10.0)}
        />
        </div>
    }
}
