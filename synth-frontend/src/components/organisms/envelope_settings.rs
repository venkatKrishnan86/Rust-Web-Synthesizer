use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use crate::components::atoms::slider::Slider;

const OSCILLATOR_SELECT_CSS: &str = include_str!("../../UI_components/selectors/oscillator_selector.css");
const ENVELOPE_SETTINGS: &str = include_str!("../../UI_components/selectors/slider_envelope.css");

#[derive(Properties, PartialEq)]
pub struct EnvelopeProperties {
    pub attack_change: Callback<f64>,
    pub decay_change: Callback<f64>,
    pub sustain_change: Callback<f64>,
    pub attack: f64,
    pub decay: f64,
    pub sustain: f64
}

#[styled_component(EnvelopeSettings)]
pub fn filter_selector(props: &EnvelopeProperties) -> Html {
    let overall_css = Style::new(ENVELOPE_SETTINGS).unwrap();
    let attack_change = props.attack_change.clone();
    let decay_change = props.decay_change.clone();
    let sustain_change = props.sustain_change.clone();

    html! {
        <div class={overall_css}>
        <Slider 
            label={"Attack"}
            value={props.attack}
            onchange={attack_change}
            precision={Some(1)}
            percentage={false}
            min={0.0}
            max={1000.0}
            step={Some(0.1)}
        />

        <Slider 
            label={"Decay"}
            value={props.decay}
            onchange={decay_change}
            precision={Some(1)}
            percentage={false}
            min={0.0}
            max={1000.0}
            step={Some(0.1)}
        />

        <Slider 
            label={"Sustain"}
            value={props.sustain}
            onchange={sustain_change}
            precision={Some(1)}
            percentage={true}
            min={0.0}
            max={1.0}
            step={Some(0.001)}
        />
        </div>
    }
}
