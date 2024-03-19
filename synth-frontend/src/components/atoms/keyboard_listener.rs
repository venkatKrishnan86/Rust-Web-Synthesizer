use yew::prelude::*;
use gloo::events::EventListener;
use web_sys::KeyboardEvent;
use wasm_bindgen::JsCast;

pub enum KeyHitType {
    Up,
    Down
}

// Yew component messages
pub enum Msg{
    KeyEvent { event: KeyboardEvent, hit_type: KeyHitType},
}

#[derive(Properties, PartialEq)]
pub struct KeyboardListenerProps {
    pub key_down: Callback<KeyboardEvent>,
    pub key_up: Callback<KeyboardEvent>
}

pub struct KeyboardListener {
    /// Holds the listener once it's stood up. Can't be done before rendering because the document doesn't exist yet
    pub kbd_listener: Option<EventListener>,
    pub kbu_listener: Option<EventListener>,
}


impl Component for KeyboardListener {
    type Message = Msg;
    type Properties = KeyboardListenerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            kbd_listener: None,
            kbu_listener: None
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1></h1>
        }
    }

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