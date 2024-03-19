use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub class: String,
    pub label: String,
    pub is_active: bool,
    pub mouse_down: Callback<MouseEvent>,
    pub mouse_up: Option<Callback<MouseEvent>>
}

#[styled_component(CustomButton)]
pub fn custom_button(props: &ButtonProps) -> Html{
    match &props.mouse_up {
        Some(callback_val) => {
            html! {
                <button 
                    class = {&props.class} 
                    onmousedown={&props.mouse_down} 
                    onmouseup={callback_val}
                >{&props.label}</button>
            }
        },
        None => {
            html! {
                <button 
                    class = {&props.class} 
                    onmousedown={&props.mouse_down}
                >{&props.label}</button>
            }
        }
    }
}