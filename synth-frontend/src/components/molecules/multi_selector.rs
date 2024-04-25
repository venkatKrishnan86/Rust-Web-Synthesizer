use stylist::yew::styled_component;
use yew::prelude::*;
use crate::components::molecules::selector::Selector;

/// Properties for the `MultiSelector` component.
#[derive(Properties, PartialEq)]
pub struct MultiSelectorProps {
    /// The CSS class for the icon.
    pub icon_class: String,
     /// The labels displayed alongside each selector.
    pub label: Vec<(char, usize)>,
    /// Indicates whether the multi-selector is active.
    pub is_active: bool,
    /// The index of the active selector.
    pub active_index: usize,
    /// The paths to the images associated with each selector.
    pub img_path: Vec<String>,
    /// Callback invoked when the mouse button is pressed down on a selector.
    pub on_mouse_down: Callback<(char, usize)>,
    /// Callback invoked when the mouse button is released on a selector.
    pub on_mouse_up: Callback<(char, usize)>,
}
/// The `MultiSelector` component represents a collection of selectors, allowing multiple selections.
#[styled_component(MultiSelector)]
pub fn selector(props: &MultiSelectorProps) -> Html {
    html! {
        <>
        {multi_selection(
            props.icon_class.clone(), 
            props.label.clone(),
            props.is_active,
            props.active_index,
            props.img_path.clone(),
            props.on_mouse_down.clone(),
            props.on_mouse_up.clone()
        )}
        </>
    }
}

/// Generates the HTML for multiple selectors based on the provided properties.
pub fn multi_selection(
    icon_class: String,
    labels: Vec<(char, usize)>,
    is_active: bool,
    active_index: usize,
    img_path: Vec<String>,
    on_mouse_down: Callback<(char, usize)>,
    on_mouse_up: Callback<(char, usize)>,
) -> Vec<Html> {
    let mut vector = Vec::new();
    for (idx, (label, img)) in labels.iter().zip(img_path).enumerate() {
        let mouse_down = on_mouse_down.clone();
        let label = label.clone();
        let mouse_down = Callback::from(move |_| {
            mouse_down.emit(label);
        });

        let mouse_up = on_mouse_up.clone();
        let label = label.clone();
        let mouse_up = Callback::from(move |_| {
            mouse_up.emit(label);
        });
        if idx == active_index {
            vector.push(html! {
                <Selector 
                    icon_class={icon_class.clone()+"_active"} 
                    label={label} 
                    is_active={is_active}
                    img_path={img.clone()}
                    on_mouse_down={mouse_down}
                    on_mouse_up={mouse_up}
                />
            })
        } else {
            vector.push(html! {
                <Selector 
                    icon_class={icon_class.clone()} 
                    label={label} 
                    is_active={is_active}
                    img_path={img.clone()}
                    on_mouse_down={mouse_down}
                    on_mouse_up={mouse_up}
                />
            })
        }
    }
    vector
}