use stylist::yew::styled_component;
use yew::prelude::*;
use crate::components::atoms::button::CustomButton;

#[derive(Properties, PartialEq)]
pub struct AddButtonProps {
    pub on_mouse_down: Callback<char>,
    pub on_mouse_up: Callback<char>,
}

#[styled_component(AddButton)]
pub fn add_button(props: &AddButtonProps) -> Html{
    let mouse_down = props.on_mouse_down.clone();
    let label = '+';
    let mouse_down = Callback::from(move |_| {
        mouse_down.emit(label);
    });

    let mouse_up = props.on_mouse_up.clone();
    let label = '+';
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