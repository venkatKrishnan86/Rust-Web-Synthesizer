use std::collections::HashMap;
use std::ops::Deref;
use synth_backend::utils::{decrease_octave, increase_octave};
use yew::prelude::*;
use stylist::yew::styled_component;
use gloo::console::log;
use web_sys::{AudioContext, OscillatorNode};

use synth_frontend::MIDIKeyboard;
use synth_backend::utils::midi_to_hz;

#[styled_component(App)]
pub fn app() -> Html {
    let polyphony: UseStateHandle<HashMap<u8, OscillatorNode>> = use_state(|| HashMap::new());
    let audio_context = use_state(|| AudioContext::new().expect("Could not create an AudioContext object"));
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
    let cloned_audio_context = audio_context.clone();
    let cloned_poly = polyphony.clone();
    let mouse_down = Callback::from(move |label: char| {
        let key_label = key_map_down.get(&label).unwrap_or(&0);
        log!("Holding key", label.to_string(), ", MIDI Note:", key_label.to_string());
        let cloned_key_map = &mut key_map_down.deref().clone();
        match label {
            'Z' => decrease_octave(cloned_key_map),
            'X' => increase_octave(cloned_key_map),
            _ => {
                let context = cloned_audio_context.deref().clone();
                let mut buffer = cloned_poly.deref().clone();
                let osc = context.create_oscillator().expect("Could not create oscillator");
                // let gain = context.create_gain().expect("Could not create gain");
                // gain.connect_with_audio_node(&context.destination()).expect("Could not connect gain to audio node");
                osc.connect_with_audio_node(&context.destination()).expect("Could not connect oscillator to audio node");
                osc.set_type(web_sys::OscillatorType::Sine);
                osc.frequency().set_value(midi_to_hz(*key_label).ok().unwrap());
                osc.start().expect("Failed to start oscillator");
                buffer.insert(*key_label, osc);
                cloned_poly.set(buffer);
                cloned_audio_context.set(context);
            }
        }
        key_map_setter.set(cloned_key_map.deref().clone());
    });

    let key_map_up = keycode_maps.clone();
    let cloned_poly = polyphony.clone();
    let cloned_audio_context = audio_context.clone();
    let mouse_up = Callback::from(move |label: char| {
        let key_label = key_map_up.get(&label).unwrap_or(&0);
        let mut poly = cloned_poly.deref().clone();
        let context = cloned_audio_context.deref().clone();
        let osc = poly.remove(key_label);
        match osc {
            None => (),
            Some(val) => {
                val.stop().expect("Failed to stop oscillator");
                val.disconnect_with_audio_node(&context.destination()).expect("Could not disconnect from audio node");
            }
        }
        cloned_poly.set(poly);
        log!("Lifted key", label.to_string(), ", MIDI Note:", key_map_up.get(&label).unwrap_or(&0).to_string());
    });

    let key_map_setter = keycode_maps.setter();
    let key_map_down = keycode_maps.clone();
    let cloned_audio_context = audio_context.clone();
    let cloned_poly = polyphony.clone();
    let key_down = Callback::from(move |label: char| {
        let key_label = key_map_down.get(&label).unwrap_or(&0);
        let cloned_key_map = &mut key_map_down.deref().clone();
        match label {
            'Z' => decrease_octave(cloned_key_map),
            'X' => increase_octave(cloned_key_map),
            _ => {
                let context = cloned_audio_context.deref().clone();
                let mut buffer = cloned_poly.deref().clone();
                let osc = context.create_oscillator().expect("Could not create oscillator");
                osc.connect_with_audio_node(&context.destination()).expect("Could not connect to audio node");
                osc.set_type(web_sys::OscillatorType::Sine);
                osc.frequency().set_value(midi_to_hz(*key_label).ok().unwrap());
                osc.start().expect("Failed to start oscillator");
                buffer.insert(*key_label, osc);
                cloned_poly.set(buffer);
                cloned_audio_context.set(context);
            }
        }
        key_map_setter.set(cloned_key_map.deref().clone());
        log!("Holding key", label.to_string(), ", MIDI Note:", key_label.to_string());
    });

    let key_map_up = keycode_maps.clone();
    let cloned_poly = polyphony.clone();
    let cloned_audio_context = audio_context.clone();
    let key_up = Callback::from(move |label: char| {
        let key_label = key_map_up.get(&label).unwrap_or(&0);
        let mut poly = cloned_poly.deref().clone();
        let context = cloned_audio_context.deref().clone();
        let osc = poly.remove(key_label);
        match osc {
            None => (),
            Some(val) => {
                val.stop().expect("Failed to stop oscillator");
                val.disconnect_with_audio_node(&context.destination()).expect("Could not disconnect from audio node");
            }
        }
        cloned_poly.set(poly);
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