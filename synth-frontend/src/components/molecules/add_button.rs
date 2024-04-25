use stylist::yew::styled_component;
use yew::prelude::*;
use crate::components::atoms::button::CustomButton;

/// Properties for the `AddButton` component.
#[derive(Properties, PartialEq)]
pub struct AddButtonProps {
    /// Callback invoked when the mouse button is pressed down on the button.
    pub on_mouse_down: Callback<(char, usize)>,
    /// Callback invoked when the mouse button is released on the button.
    pub on_mouse_up: Callback<(char, usize)>,
}

/// The `add_button` component adds a styled button with a `+` label.
#[styled_component(AddButton)]
pub fn add_button(props: &AddButtonProps) -> Html{
    let mouse_down = props.on_mouse_down.clone();
    let label = ('+', 0);
    let mouse_down = Callback::from(move |_| {
        mouse_down.emit(label);
    });

    let mouse_up = props.on_mouse_up.clone();
    let label = ('+', 0);
    let mouse_up = Callback::from(move |_| {
        mouse_up.emit(label);
    });
    html! {
        <div class = {"add_button"}>
            <CustomButton 
                class={"add_button"} 
                label={"+"}
                mouse_down={mouse_down}
                mouse_up={mouse_up}
            />
        </div>
    }
}