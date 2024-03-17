use yew::prelude::*;
use stylist::yew::styled_component;
use synth_frontend::MIDIKeyboard;
use gloo::console::log;

#[styled_component(App)]
pub fn app() -> Html {
    let mouse_down = Callback::from(move |_| {
        log!("Holding key");
    });

    let mouse_up = Callback::from(move |_| {
        log!("Lifted key");
    });
    html! {
        <MIDIKeyboard mouse_down={mouse_down} mouse_up={&mouse_up}/>
    }
}