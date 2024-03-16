use yew::prelude::*;
use stylist::yew::styled_component;
use synth_frontend::MIDIKeyboard;
use gloo::console::log;

#[styled_component(App)]
pub fn app() -> Html {
    let mouse_click = Callback::from(move |_| {
        log!("Clicked a key");
    });
    html! {
        <MIDIKeyboard mouse_click={mouse_click}/>
    }
}