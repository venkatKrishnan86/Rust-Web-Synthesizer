use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IconProps {
    pub class: String,
    pub label: String,
    pub img_path: String,
    pub is_active: bool,
    pub mouse_down: Callback<MouseEvent>,
    pub mouse_up: Option<Callback<MouseEvent>>
}

#[styled_component(CustomIcon)]
pub fn custom_icon(props: &IconProps) -> Html{
    match &props.mouse_up {
        Some(callback_val) => {
            html! {
                <button 
                    class = {&props.class} 
                    onmousedown={&props.mouse_down} 
                    onmouseup={callback_val}
                >
                // {props.label.clone()}
                <img class = {&props.class} src={props.img_path.clone()} alt={props.label.clone()} />
                </button>
            }
        },
        None => {
            html! {
                <button 
                    class = {&props.class} 
                    onmousedown={&props.mouse_down}
                >
                // {props.label.clone()}
                <img class={&props.class} src={props.img_path.clone()} alt={props.label.clone()} />
                </button>
            }
        }
    }
}