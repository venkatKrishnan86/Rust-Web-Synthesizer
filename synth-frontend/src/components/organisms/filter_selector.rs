use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use crate::components::molecules::selector::Selector;
use crate::components::atoms::slider::Slider;
use std::collections::HashMap;

const OSCILLATOR_SELECT_CSS: &str = include_str!("../../UI_components/selectors/oscillator_selector.css");

#[derive(Properties, PartialEq)]
pub struct FilterSelectorProperties {
    pub mouse_down: Callback<(char, usize)>,
    pub mouse_up: Callback<(char, usize)>,
    pub freq_change: Callback<f64>,
    pub freq: f64
}

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

    let cloned_oscillator_class_hashmap = oscillator_class_hashmap.clone();

    html! {
        <div class={overall_css}>
        <Selector
        icon_class={"power-off"} 
        label={('7', 0)} 
        img_path={"UI_components/assets/icons/LowPass.png"} 
        is_active={false} 
        on_mouse_down={&mouse_down} 
        on_mouse_up={Callback::from(|_|{})}
        />
        <Selector 
        icon_class={oscillator_class_hashmap[&'0']} 
        label={('0', 0)} 
        img_path={"https://i.ibb.co/HFCG4h1/HighPass.png"} 
        is_active={false} 
        on_mouse_down={&mouse_down} 
        on_mouse_up={Callback::from(|_|{})}
        />
        <Selector
        icon_class={oscillator_class_hashmap[&'9']} 
        label={('9', 0)} 
        img_path={"https://i.ibb.co/23gfy5Q/BandPass.png"} 
        is_active={false} 
        on_mouse_down={&mouse_down} 
        on_mouse_up={Callback::from(|_|{})}
        />
        <Selector
        icon_class={oscillator_class_hashmap[&'8']} 
        label={('8', 0)}
        img_path={"https://i.ibb.co/SQXCXkf/LowPass.png"} 
        is_active={false} 
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
