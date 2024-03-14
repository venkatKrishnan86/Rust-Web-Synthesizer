use yew::prelude::*;
use stylist::yew::styled_component;
use synth_frontend::MIDIKeyboard;

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <MIDIKeyboard />
    }
}