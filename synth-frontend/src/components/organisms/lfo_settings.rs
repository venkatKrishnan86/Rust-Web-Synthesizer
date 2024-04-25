use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use crate::components::molecules::multi_selector::MultiSelector;
use crate::components::atoms::slider::Slider;

/// CSS for the LFO selector component.
const OSCILLATOR_SELECT_CSS: &str = include_str!("../../UI_components/selectors/oscillator_selector.css");

/// Properties for the `LFOSelector` component.
#[derive(Properties, PartialEq)]
pub struct LFOSelectorProperties {
    /// Callback invoked when the mouse button is pressed down on the component.
    pub mouse_down: Callback<(char, usize)>,
    /// Callback invoked when the mouse button is released on the component.
    pub mouse_up: Callback<(char, usize)>,
    /// Callback invoked when the frequency value changes.
    pub freq_change: Callback<f64>,
    /// The index of the active item in the multi-selector.
    pub active_index: usize,
    /// The index of the active item in the type of the LFO.
    pub active_index_type: usize,
    /// The frequency value.
    pub freq: f64
}

/// The `lfo_selector` component represents an LFO selector with a multi-selector and a frequency slider.
#[styled_component(LFOSelector)]
pub fn lfo_selector(props: &LFOSelectorProperties) -> Html {
    let overall_css = Style::new(OSCILLATOR_SELECT_CSS).unwrap();
    let mouse_down = props.mouse_down.clone();
    let freq_change = props.freq_change.clone();
    let labels = vec![
        ('|', 0),
        ('[', 0),
        (']', 0),
        ('{', 0),
        ('}', 0),
    ];
    let images = vec![
        "https://i.ibb.co/d7W1DrQ/Power.png".to_owned(),
        "https://i.ibb.co/XZWhWv5/Sine.png".to_owned(),
        "https://i.ibb.co/P1wjXPj/Square.png".to_owned(),
        "https://i.ibb.co/NLj5Szb/saw-tooth.png".to_owned(),
        "https://i.ibb.co/thqmPmZ/Triangle.png".to_owned()
    ];


    let labels_type = vec![
        ('<', 0),
        ('>', 0),
    ];
    let images_type = vec![
        "https://i.ibb.co/B4Kf4R0/amp.png".to_owned(),
        "https://i.ibb.co/tKSRKFR/freq.png".to_owned(),
    ];
    html! {
        <div class={overall_css}>
        <MultiSelector
            icon_class={"lfo-icon"} 
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
            min={0.01}
            max={50.0}
            step={Some(0.01)}
        />
        <MultiSelector
            icon_class={"lfo-icon-type"} 
            label={labels_type} 
            img_path={images_type} 
            is_active={false} 
            active_index={props.active_index_type}
            on_mouse_down={&mouse_down} 
            on_mouse_up={Callback::from(|_|{})}
        />
        </div>
    }
}
