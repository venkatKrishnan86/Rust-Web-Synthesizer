use stylist::yew::styled_component;
use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct KnobProps {
    pub label: String,
    pub value: f32,           
    pub min: f32,             
    pub max: f32,             
    pub step: f32,            
    pub on_change: Callback<f32>, 
}

#[styled_component(Knob)]
pub fn knob(props: &KnobProps) -> Html {
    let KnobProps {
        label,
        value,
        min,
        max,
        step,
        on_change,
    } = props.clone();

    let on_input = {
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            let value = input.value_as_number() as f32;
            on_change.emit(value);
        })
    };

    html! {
        <div class="knob">
            <label>{label}</label>
            <input type="range" class="knob-input" min={min.to_string()} max={max.to_string()} step={step.to_string()} value={value.to_string()} {on_input} />
        </div>
    }
}
