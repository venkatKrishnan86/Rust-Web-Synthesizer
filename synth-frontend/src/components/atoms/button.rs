//! Button component module providing a customizable button for use in web applications.
//!
//! The `ButtonProps` struct represents the properties required to configure the button component, including its class,
//! label, and optional mouse event callbacks.
//!
//! # Examples
//!
//! ```rust
//! use synth_frontend::components::atoms::button::{ButtonProps, custom_button};
//! use yew::prelude::*;
//!
//! // Define properties for the button
//! let button_props = ButtonProps {
//!     class: String::from("my-button"),
//!     label: String::from("Click me"),
//!     mouse_down: Callback::noop(), // Placeholder callback
//!     mouse_up: Some(Callback::noop()), // Placeholder callback
//! };
//!
//! // Create the custom button component
//! let button_component = custom_button(&button_props);
//! ```
//!
//! The `custom_button` function generates a button component based on the provided properties, allowing customization
//! of the button's appearance and behavior.
use stylist::yew::styled_component;
use yew::prelude::*;

/// Properties struct for configuring the button component.
#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    /// The CSS class for styling the button.
    pub class: String,
    /// The label text displayed on the button.
    pub label: String,
    /// The callback triggered when the mouse is pressed down on the button.
    pub mouse_down: Callback<MouseEvent>,
    /// An optional callback triggered when the mouse is released after pressing the button.
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