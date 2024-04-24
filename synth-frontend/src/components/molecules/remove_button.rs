use stylist::yew::styled_component;
use yew::prelude::*;
use crate::components::atoms::button::CustomButton;


#[derive(Properties, PartialEq)]
pub struct RemoveButtonProps {
    pub on_mouse_down: Callback<(char, usize)>,
    pub on_mouse_up: Callback<(char, usize)>,
    pub number: usize
}

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
                label={"-"}
                mouse_down={mouse_down}
                mouse_up={mouse_up}
            />
        </div>
    }
}