use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    label: String,
    onhold: Callback<bool>
}

pub fn create_white_keys() -> Vec<Html> {
    let mut keys = Vec::new();
    let keycodes = vec!['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K'];
    for index in 0..8{
        keys.push(html! {
            <div class = "whitekey">
                <button class = "keycodes">{keycodes[index]}</button>
            </div>
        })
    }
    keys
}

pub fn create_black_key(letter: char) -> Html {
    html! {
        <div class = "blackkey">
            <button class = "keycodes">{letter}</button>
        </div>
    }
}