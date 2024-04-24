use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use crate::components::molecules::selector::Selector;
use crate::components::atoms::slider::Slider;

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
        icon_class={"filter-icon"} 
        label={('0', 0)} 
        img_path={"UI_components/assets/icons/HighPass.png"} 
        is_active={false} 
        on_mouse_down={&mouse_down} 
        on_mouse_up={Callback::from(|_|{})}
        />
        <Selector
        icon_class={"oscillator-icon"} 
        label={('9', 0)} 
        img_path={"UI_components/assets/icons/BandPass.png"} 
        is_active={false} 
        on_mouse_down={&mouse_down} 
        on_mouse_up={Callback::from(|_|{})}
        />
        <Selector
        icon_class={"oscillator-icon"} 
        label={('8', 0)} 
        img_path={"UI_components/assets/icons/LowPass.png"} 
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
