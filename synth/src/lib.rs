use yew::prelude::*;
use stylist::yew::styled_component;
use synth_frontend::MIDIKeyboard;
use gloo::console::log;

#[styled_component(App)]
pub fn app() -> Html {
    let mouse_down = Callback::from(move |ch: char| {
        log!("Holding key", ch.to_string());
    });

    let mouse_up = Callback::from(move |ch: char| {
        log!("Lifted key", ch.to_string());
    });
    html! {
        <MIDIKeyboard mouse_down={mouse_down} mouse_up={&mouse_up}/>
    }
}