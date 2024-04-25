use std::cell::Cell;

use web_sys::HtmlInputElement;
use yew::events::InputEvent;
use yew::{html, Callback, Component, Context, Html, Properties, TargetCast};

/// Thread-local storage for maintaining unique IDs for sliders.
thread_local! {
    static SLIDER_ID: Cell<usize> = Cell::default();
}
/// Generates the next unique slider ID.
fn next_slider_id() -> usize {
    SLIDER_ID.with(|cell| cell.replace(cell.get() + 1))
}

/// Properties for the `Slider` component.
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    /// The label displayed alongside the slider.
    pub label: &'static str,
    /// The current value of the slider.
    pub value: f64,
    /// Callback invoked when the slider value changes.
    pub onchange: Callback<f64>,
    /// The number of decimal places to round the displayed value to.
    #[prop_or_default]
    pub precision: Option<usize>,
    /// Whether to display the slider value as a percentage.
    #[prop_or_default]
    pub percentage: bool,
    /// The minimum value of the slider.
    #[prop_or_default]
    pub min: f64,
    /// The maximum value of the slider.
    pub max: f64,
    /// The step size for the slider.
    #[prop_or_default]
    pub step: Option<f64>,
}

/// The `Slider` component allows users to select a value within a range using a slider input.
pub struct Slider {
    id: usize,
}
impl Component for Slider {
    type Message = ();
    type Properties = Props;

    /// Creates a new `Slider` component instance.
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            id: next_slider_id(),
        }
    }

    /// Updates the `Slider` component.
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        unimplemented!()
    }

    /// Renders the `Slider` component.
    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            label,
            value,
            ref onchange,
            precision,
            percentage,
            min,
            max,
            step,
        } = *ctx.props();

        let precision = precision.unwrap_or_else(|| usize::from(percentage));

        let display_value = if percentage {
            format!("{:.p$}%", 100.0 * value, p = precision)
        } else {
            format!("{value:.precision$}")
        };

        let id = format!("slider-{}", self.id);
        let step = step.unwrap_or_else(|| {
            let p = if percentage { precision + 2 } else { precision };
            10f64.powi(-(p as i32))
        });

        let oninput = onchange.reform(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            input.value_as_number()
        });

        html! {
            <div class="slider">
                <label for={id.clone()} class="slider__label">{ label }</label>
                <input type="range"
                    value={value.to_string()}
                    {id}
                    class="slider__input"
                    min={min.to_string()} max={max.to_string()} step={step.to_string()}
                    {oninput}
                />
                <span class="slider__value">{ display_value }</span>
            </div>
        }
    }
}