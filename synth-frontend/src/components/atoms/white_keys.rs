use stylist::yew::styled_component;
use yew::prelude::*;
use super::KeyProps;

#[styled_component(WhiteKey)]
pub fn white_key(props: &KeyProps) -> Html {
    html! {
        <div class = "whitekey">
            <button class = "keycodes">{props.label}</button>
        </div>
    }
}

pub fn create_white_keys() -> Vec<Html> {
    let mut keys = Vec::new();
    let keycodes = vec!['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K'];
    for index in 0..8{
        keys.push(html! {
            <WhiteKey label={keycodes[index]}/>
        })
    }
    keys
}