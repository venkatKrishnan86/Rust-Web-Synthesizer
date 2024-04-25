use stylist::yew::styled_component;
use yew::prelude::*;

/// Properties for the custom icon component.
#[derive(Properties, PartialEq)]
pub struct IconProps {
    /// The CSS class for the icon.
    pub class: String,
    /// The label for the icon.
    pub label: String,
     /// The path to the icon image.
    pub img_path: String,
    /// Whether the icon is active.
    pub is_active: bool,
    /// Callback for mouse down event.
    pub mouse_down: Callback<MouseEvent>,
    /// Optional callback for the mouse up event.
    pub mouse_up: Option<Callback<MouseEvent>>
}

#[styled_component(CustomIcon)]
pub fn custom_icon(props: &IconProps) -> Html{
    match &props.mouse_up {
        Some(callback_val) => {
            html! {
                <div class={&props.class}>
                <button
                    onmousedown={&props.mouse_down} 
                    onmouseup={callback_val}
                >
                // {props.label.clone()}
                <img src={props.img_path.clone()} alt={props.label.clone()} />
                </button>
                </div>
            }
        },
        None => {
            html! {
                <button 
                    onmousedown={&props.mouse_down}
                >
                // {props.label.clone()}
                <img class={&props.class} src={props.img_path.clone()} alt={props.label.clone()} />
                </button>
            }
        }
    }
}