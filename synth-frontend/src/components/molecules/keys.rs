use stylist::yew::styled_component;
use yew::prelude::*;
use crate::components::atoms::button::CustomButton;

/// Represents the color of a keyboard key.
#[derive(PartialEq)]
pub enum KeyColor {
    /// Indicates a white key color.
    White,
    /// Indicates a black key color.
    Black
}

impl KeyColor {
    /// Converts the `KeyColor` enum variant to a string representing the CSS class.
    pub fn to_string(&self) -> String {
        match self {
            Self::White => "whitekey".to_owned(),
            Self::Black => "blackkey".to_owned(),
        }
    }
}

/// Properties for the `Key` component.
#[derive(Properties, PartialEq)]
pub struct KeyProps {
    /// The CSS class for the button.
    pub button_class: String,
    /// The label displayed on the key.
    pub label: (char, usize),
    /// The color of the key.
    pub key_color: KeyColor,
    /// Callback invoked when the mouse button is pressed down on the key.
    pub on_mouse_down: Callback<(char, usize)>,
    /// Callback invoked when the mouse button is released on the key.
    pub on_mouse_up: Callback<(char, usize)>,
}

/// The `key` component represents a keyboard key with a customizable label and appearance.
#[styled_component(Key)]
pub fn key(props: &KeyProps) -> Html {
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
        <div class = {&props.key_color.to_string()}>
            <CustomButton 
                class={props.button_class.clone()} 
                label={props.label.0.to_string()}
                mouse_down={mouse_down}
                mouse_up={mouse_up}
            />
        </div>
    }
}