use stylist::yew::styled_component;
use yew::prelude::*;
use crate::components::atoms::knob::Knob;  

#[derive(Properties, PartialEq)]
pub struct ADSRControlsProps {
    pub on_attack_change: Callback<f32>,
    pub on_decay_change: Callback<f32>,
    pub on_sustain_change: Callback<f32>,
    pub on_release_change: Callback<f32>,
}

#[styled_component(ADSRControls)]
pub fn adsr_controls(props: &ADSRControlsProps) -> Html {
    html! {
        <div class="adsr-controls">
            <Knob label="Attack" on_change={props.on_attack_change.clone()} />
            <Knob label="Decay" on_change={props.on_decay_change.clone()} />
            <Knob label="Sustain" on_change={props.on_sustain_change.clone()} />
            <Knob label="Release" on_change={props.on_release_change.clone()} />
        </div>
    }
}
