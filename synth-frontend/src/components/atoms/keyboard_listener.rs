use yew::prelude::*;
use gloo::events::EventListener;
use web_sys::KeyboardEvent;
use wasm_bindgen::JsCast;

/// Represents the type of keyboard event.
pub enum KeyHitType {
    /// Indicates a key down event.
    Up,
    /// Indicates a key up event.
    Down
}

// Yew component messages
pub enum Msg{
    /// Represents a keyboard event.
    KeyEvent { event: KeyboardEvent, hit_type: KeyHitType},
}

/// Properties for the `KeyboardListener` component.
#[derive(Properties, PartialEq)]
pub struct KeyboardListenerProps {
    /// Callback for handling key down events.
    pub key_down: Callback<KeyboardEvent>,
    /// Callback for handling key up events.
    pub key_up: Callback<KeyboardEvent>
}

/// A Yew component that listens for keyboard events.
pub struct KeyboardListener {
    /// Holds the listener for key down events.
    pub kbd_listener: Option<EventListener>,
    /// Holds the listener for key up events.
    pub kbu_listener: Option<EventListener>,
}


impl Component for KeyboardListener {
    type Message = Msg;
    type Properties = KeyboardListenerProps;

    /// Creates a new `KeyboardListener` component.
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            kbd_listener: None,
            kbu_listener: None
        }
    }

    /// Renders the `KeyboardListener` component.
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1></h1>
        }
    }

    /// Handles messages sent to the `KeyboardListener` component.
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::KeyEvent { event , hit_type: KeyHitType::Down} => {
                let cloned_keydown = ctx.props().key_down.clone();
                cloned_keydown.emit(event);
                true
            },
            Msg::KeyEvent { event , hit_type: KeyHitType::Up} => {
                let cloned_keyup = ctx.props().key_up.clone();
                cloned_keyup.emit(event);
                true
            }
        }
    }

    /// Executes after the `KeyboardListener` component is rendered.
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        // we only need to run the below stuff the first time
        if !first_render {
            return;
        }

        let document = web_sys::window().unwrap().document().unwrap();
        let ct = ctx.link().to_owned();
        let listener = EventListener::new(&document, "keydown", move |event| {
            let event = event
                .dyn_ref::<KeyboardEvent>()
                .unwrap()
                .to_owned();
            ct.send_message(Msg::KeyEvent { event, hit_type: KeyHitType::Down });
        });

        self.kbd_listener.replace(listener);

        let ct = ctx.link().to_owned();
        let listener = EventListener::new(&document, "keyup", move |event| {
            let event = event
                .dyn_ref::<KeyboardEvent>()
                .unwrap()
                .to_owned();
            ct.send_message(Msg::KeyEvent { event, hit_type: KeyHitType::Up });
        });

        self.kbu_listener.replace(listener);
    }
}