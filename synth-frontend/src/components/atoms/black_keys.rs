use yew::prelude::*;
use stylist::yew::styled_component;
use super::KeyProps;

#[styled_component(BlackKey)]
pub fn black_key(props: &KeyProps) -> Html {
    html! {
        <div class = "blackkey">
            <button class = "keycodes">{props.label}</button>
        </div>
    }
}