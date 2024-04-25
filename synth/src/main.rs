///
/// # Example Usage of Yew
///
/// ```
/// use synth::App;
///
/// fn main() {
///     // Create a new Yew renderer for the `App` and render it.
///     yew::Renderer::<App>::new().render();
/// }
/// ```

use synth::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
