use stylist::yew::styled_component;
use yew::prelude::*;
use crate::components::atoms::button::CustomButton;

/// Properties for the `RemoveButton` component.
#[derive(Properties, PartialEq)]
pub struct RemoveButtonProps {
    /// Callback invoked when the mouse button is pressed down on the button.
    pub on_mouse_down: Callback<(char, usize)>,
    /// Callback invoked when the mouse button is released on the button.
    pub on_mouse_up: Callback<(char, usize)>,
    /// The number associated with the remove button.
    pub number: usize
}

/// The `remove_button` component displays a styled button with a `x` label for removing items.
#[styled_component(RemoveButton)]
pub fn remove_button(props: &RemoveButtonProps) -> Html{
    let mouse_down = props.on_mouse_down.clone();
    let number = props.number;
    let label = ('-', number);
    let mouse_down = Callback::from(move |_| {
        mouse_down.emit(label);
    });

    let mouse_up = props.on_mouse_up.clone();
    let label = ('-', number);
    let mouse_up = Callback::from(move |_| {
        mouse_up.emit(label);
    });
    html! {
        <div class = {"remove_button"}>
            <CustomButton 
                class={"remove_button"} 
                label={"x"}
                mouse_down={mouse_down}
                mouse_up={mouse_up}
            />
        </div>
    }
}