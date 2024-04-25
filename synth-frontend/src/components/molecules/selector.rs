use stylist::yew::styled_component;
use yew::prelude::*;
use crate::components::atoms::icon::CustomIcon;


/// Properties for the `Selector` component.
#[derive(Properties, PartialEq)]
pub struct SelectorProps {
    /// The CSS class for the icon.
    pub icon_class: String,
    /// The label displayed on the selector.
    pub label: (char, usize),
    /// Indicates whether the selector is active.
    // pub icon_type: IconType,
    pub is_active: bool,
    /// The path to the image associated with the selector.
    pub img_path: String,
    /// Callback invoked when the mouse button is pressed down on the selector.
    pub on_mouse_down: Callback<(char, usize)>,
    /// Callback invoked when the mouse button is released on the selector.
    pub on_mouse_up: Callback<(char, usize)>,
}

/// The `selector` component represents a selectable item with an icon.
#[styled_component(Selector)]
pub fn selector(props: &SelectorProps) -> Html {
    let mouse_down = props.on_mouse_down.clone();
    let label = props.label.clone();
    let mouse_down = Callback::from(move |_| {
        mouse_down.emit(label);
    });

    let mouse_up = props.on_mouse_up.clone();
    let label = props.label.clone();
    let mouse_up = Callback::from(move |_| {
        mouse_up.emit(label);
    });

    html! {
        <div>
            <CustomIcon 
                class={props.icon_class.clone()} 
                label={props.label.0.to_string()} 
                is_active={props.is_active}
                img_path={props.img_path.clone()}
                mouse_down={mouse_down}
                mouse_up={mouse_up}
            />
        </div>
    }
}