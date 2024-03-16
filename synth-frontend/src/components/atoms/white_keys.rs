use gloo::console::log;
use stylist::yew::styled_component;
use yew::prelude::*;
use super::KeyProps;

#[styled_component(WhiteKey)]
pub fn white_key(props: &KeyProps) -> Html {
    let mouse_click = props.on_mouse_click.clone();
    let mouse_click = Callback::from(move |_| {
        mouse_click.emit(());
    });
    html! {
        <div class = "whitekey">
            <button class = "keycodes" onclick={&mouse_click}>{props.label}</button>
        </div>
    }
}

pub fn create_white_keys() -> Vec<Html> {
    let mut keys = Vec::new();
    let keycodes = vec!['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K'];
    let mouse_click = Callback::from(move |_| {
        log!("Clicked white key");
    });

    for index in 0..8{
        keys.push(html! {
            <WhiteKey label={keycodes[index]} on_mouse_click={&mouse_click}/>
        })
    }
    keys
}