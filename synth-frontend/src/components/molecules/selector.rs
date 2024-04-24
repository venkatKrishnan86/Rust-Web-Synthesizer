use stylist::yew::styled_component;
use yew::prelude::*;
use crate::components::atoms::icon::CustomIcon;

// #[derive(PartialEq)]
// pub enum IconType {
//     String,
//     Image,
// }

// impl IconType {
//     pub fn to_string(&self) -> String {
//         match self {
//             Self::String => "string".to_owned(),
//             Self::Image => "image".to_owned(),
//         }
//     }
// }

#[derive(Properties, PartialEq)]
pub struct SelectorProps {
    pub icon_class: String,
    pub label: char,
    // pub icon_type: IconType,
    pub is_active: bool,
    pub img_path: String,
    pub on_mouse_down: Callback<char>,
    pub on_mouse_up: Callback<char>,
}

#[styled_component(Selector)]
pub fn selector(props: &SelectorProps) -> Html {
    let mouse_down = props.on_mouse_down.clone();
    let label = props.label.clone();
    let mouse_down = Callback::from(move |_| {
        mouse_down.emit(label);
    });

    let mouse_up = props.on_mouse_up.clone();
    let label = props.label.clone();
    let mouse_up = Callback::from(move |_| {
        mouse_up.emit(label);
    });

    html! {
        <div>
            <CustomIcon 
                class={props.icon_class.clone()} 
                label={props.label.to_string()} 
                is_active={false}
                img_path={props.img_path.clone()}
                mouse_down={mouse_down}
                mouse_up={mouse_up}
            />
        </div>
    }
}