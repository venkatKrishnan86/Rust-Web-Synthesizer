use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use crate::components::molecules::multi_selector::MultiSelector;
use crate::components::atoms::slider::Slider;

const OSCILLATOR_SELECT_CSS: &str = include_str!("../../UI_components/selectors/oscillator_selector.css");

#[derive(Properties, PartialEq)]
pub struct LFOSelectorProperties {
    pub mouse_down: Callback<(char, usize)>,
    pub mouse_up: Callback<(char, usize)>,
    pub freq_change: Callback<f64>,
    pub active_index: usize,
    pub freq: f64
}

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
        "https://i.ibb.co/VSSfGGZ/Sawtooth.png".to_owned(),
        "https://i.ibb.co/thqmPmZ/Triangle.png".to_owned()
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
            min={0.0}
            max={50.0}
            step={Some(0.01)}
        />
        </div>
    }
}
