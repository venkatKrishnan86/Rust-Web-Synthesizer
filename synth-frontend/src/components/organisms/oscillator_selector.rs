use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use crate::components::molecules::selector::Selector;

const OSCILLATOR_SELECT_CSS: &str = include_str!("../../UI_components/selectors/oscillator_selector.css");

#[derive(Properties, PartialEq)]
pub struct OscillatorSelectorProperties {
    pub mouse_down: Callback<char>,
    pub mouse_up: Callback<char>,
    pub number: u32
}

#[styled_component(OscillatorSelector)]
pub fn oscillator_selector(props: &OscillatorSelectorProperties) -> Html {
    let overall_css = Style::new(OSCILLATOR_SELECT_CSS).unwrap();
    let mouse_down = props.mouse_down.clone();
    let number = props.number;
    
    html! {
        <>
        <h2>{"Oscillator "}{number}</h2>
        <div class={overall_css}>
            <Selector
                icon_class={"oscillator-icon"} 
                label={'1'} 
                img_path={"UI_components/assets/icons/Sine.png"} 
                is_active={false} 
                on_mouse_down={&mouse_down} 
                on_mouse_up={Callback::from(|_|{})}
            />
            <Selector
                icon_class={"oscillator-icon"} 
                label={'2'} 
                img_path={"UI_components/assets/icons/Square.png"} 
                is_active={false} 
                on_mouse_down={&mouse_down} 
                on_mouse_up={Callback::from(|_|{})}
            />
            <Selector
                icon_class={"oscillator-icon"} 
                label={'3'} 
                img_path={"UI_components/assets/icons/Sawtooth.png"} 
                is_active={false} 
                on_mouse_down={&mouse_down} 
                on_mouse_up={Callback::from(|_|{})}
            />
            <Selector
                icon_class={"oscillator-icon"} 
                label={'4'} 
                img_path={"UI_components/assets/icons/Triangle.png"} 
                is_active={false} 
                on_mouse_down={&mouse_down} 
                on_mouse_up={Callback::from(|_|{})}
            />
        </div>
        </>
    }
}