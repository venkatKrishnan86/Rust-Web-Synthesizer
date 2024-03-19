use std::collections::HashMap;
use std::ops::Deref;
use synth_backend::utils::{decrease_octave, increase_octave};
use yew::prelude::*;
use stylist::yew::styled_component;
use gloo::console::log;
use std::rc::Rc;

use synth_frontend::MIDIKeyboard;

use synth_backend::ring_buffer::PolyphonyRingBuffer;
use synth_backend::oscillators::*;
// use synth_backend::utils::midi_to_hz;

#[styled_component(App)]
pub fn app() -> Html {
    let poly = 16;
    let osc1 = WaveTableOscillator::new(44100, 44100, Oscillator::Sine, 0.8, 0.0);
    let osc2 = WaveTableOscillator::new(44100, 44100, Oscillator::Square, 0.2, 0.0);
    let osc3 = WaveTableOscillator::new(44100, 44100, Oscillator::Saw, 0.5, 0.0);
    let osc4 = WaveTableOscillator::new(44100, 44100, Oscillator::WhiteNoise, 0.2, 0.0);
    let _sound: Rc<MultiOscillator> = Rc::new(osc1 + osc2 + (osc3 + osc4));
    let mut _polyphony: PolyphonyRingBuffer = PolyphonyRingBuffer::new(poly, 44100);

    let keycode_maps = use_state(|| HashMap::from([
        ('A', 60),
        ('W', 61),
        ('S', 62),
        ('E', 63),
        ('D', 64),
        ('F', 65),
        ('T', 66),
        ('G', 67),
        ('Y', 68),
        ('H', 69),
        ('U', 70),
        ('J', 71),
        ('K', 72)
    ]));

    let key_map_setter = keycode_maps.setter();
    let key_map_down = keycode_maps.clone();
    let mouse_down = Callback::from(move |label: char| {
        log!("Holding key", label.to_string(), ", MIDI Note:", key_map_down.get(&label).unwrap_or(&0).to_string());
        let cloned_key_map = &mut key_map_down.deref().clone();
        match label {
            'Z' => decrease_octave(cloned_key_map),
            'X' => increase_octave(cloned_key_map),
            _ => ()
        }
        key_map_setter.set(cloned_key_map.deref().clone());
    });

    let key_map_up = keycode_maps.clone();
    let mouse_up = Callback::from(move |label: char| {
        log!("Lifted key", label.to_string(), ", MIDI Note:", key_map_up.get(&label).unwrap_or(&0).to_string());
    });

    let key_map_setter = keycode_maps.setter();
    let key_map_down = keycode_maps.clone();
    let key_down = Callback::from(move |label: char| {
        let cloned_key_map = &mut key_map_down.deref().clone();
        match label {
            'Z' => decrease_octave(cloned_key_map),
            'X' => increase_octave(cloned_key_map),
            _ => ()
        }
        key_map_setter.set(cloned_key_map.deref().clone());
        log!("Holding key", label.to_string(), ", MIDI Note:", key_map_down.get(&label).unwrap_or(&0).to_string());
    });

    let key_map_up = keycode_maps.clone();
    let key_up = Callback::from(move |label: char| {
        log!("Lifted key", label.to_string(), ", MIDI Note:", key_map_up.get(&label).unwrap_or(&0).to_string());
    });

    let key_map_clone = keycode_maps.clone();
    html! {
        <>
            <MIDIKeyboard mouse_down={mouse_down} mouse_up={&mouse_up} key_down={&key_down} key_up={&key_up}/>
            <p>{"Current MIDI Range: "}{&key_map_clone.deref()[&'A']}{" - "}{&key_map_clone.deref()[&'K']}</p>
        </>
    }
}