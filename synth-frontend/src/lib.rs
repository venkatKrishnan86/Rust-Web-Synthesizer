use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use components::atoms::{create_black_key, create_white_keys};

mod components;

const WHITE_KEYS_CSS: &str = include_str!("white_keys.css");
const BLACK_KEYS_CSS: &str = include_str!("black_keys.css");

#[styled_component(App)]
pub fn app() -> Html {
    let white_keys_style = Style::new(WHITE_KEYS_CSS).unwrap();
    let black_keys_style = Style::new(BLACK_KEYS_CSS).unwrap();
    html! {
        <>
            <div class={black_keys_style}>
                <div id="corner-left" class="filler" ></div>
                {create_black_key('W')}
                {create_black_key('E')}
                <div class="filler"></div>
                <div id="corner-left" class="filler"></div>
                {create_black_key('T')}
                {create_black_key('Y')}
                {create_black_key('U')}
                <div class="filler"></div>
                <div id="corner-left" class="filler"></div>
                <div id="corner-right" class="filler"></div>
            </div>
            <div class={white_keys_style}>
                {create_white_keys()}
            </div>
        </>
    }
}

