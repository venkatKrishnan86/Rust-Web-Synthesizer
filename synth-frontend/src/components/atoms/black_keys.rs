use yew::prelude::*;
use stylist::yew::styled_component;
use super::KeyProps;

#[styled_component(BlackKey)]
pub fn black_key(props: &KeyProps) -> Html {
    let mouse_click = props.on_mouse_click.clone();
    let mouse_click = Callback::from(move |_| {
        mouse_click.emit(());
    });
    html! {
        <div class = "blackkey">
            <button onclick={&mouse_click} class = "keycodes">{props.label}</button>
        </div>
    }
}