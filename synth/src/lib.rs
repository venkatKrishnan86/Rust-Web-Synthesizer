use std::collections::HashMap;
use std::ops::Deref;
use synth_backend::{ring_buffer::IterablePolyphonyHashMap, utils::{decrease_octave, increase_octave}};
use synth_backend::oscillators::{MultiOscillator, Oscillator, WaveTableOscillator};
use yew::prelude::*;
use stylist::yew::styled_component;
use gloo::console::log;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SampleRate, SizedSample, Stream, StreamConfig};

use synth_frontend::{MIDIKeyboard, OscillatorSelector, FilterSelector};
use synth_backend::utils::{midi_to_hz, State};




#[styled_component(App)]
pub fn app() -> Html {
    let host = cpal::default_host();
    let device = use_state(|| host.default_output_device().expect("No default output device found"));
    let mut supported_configs_range = device.supported_output_configs()
    .expect("error while querying configs");
    let supported_config = supported_configs_range.next()
        .expect("no supported config?!")
        .with_max_sample_rate();
    let sample_rate = supported_config.sample_rate().0;
    let supported_configs = device.supported_output_configs().unwrap();
    let config: UseStateHandle<StreamConfig> = use_state(|| {
        match supported_configs
        .filter(|c| c.channels() == 1)
        .max_by(|a, b| a.cmp_default_heuristics(b)) {
            Some(config) => config.with_sample_rate(SampleRate(sample_rate)).into(),
            _ => panic!("No supported configuration found for output device")
        }
    });
    // println!("Sample Rate: {}", sample_rate);
    // let config = use_state(|| supported_config.into());
    let polyphony: UseStateHandle<IterablePolyphonyHashMap> = use_state(|| IterablePolyphonyHashMap::new(sample_rate));
    let stream = use_state(|| State::new(&device, &config, polyphony.deref().clone()));

    // let audio_context = use_state(|| AudioContext::new().expect("Could not create an AudioContext object"));
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

    let osc1 = MultiOscillator::from(WaveTableOscillator::new(sample_rate, 44100, Oscillator::Sine, 0.8, 0.0));
    let osc2 = MultiOscillator::from(WaveTableOscillator::new(sample_rate, 44100, Oscillator::Square, 0.2, 0.0));
    let osc3 = MultiOscillator::from(WaveTableOscillator::new(sample_rate, 44100, Oscillator::Saw, 0.5, 0.0));
    let osc4 = MultiOscillator::from(WaveTableOscillator::new(sample_rate, 44100, Oscillator::WhiteNoise, 0.8, 0.0));
    // let sound = use_state(|| osc1 + osc2 + (osc3 + osc4));
    let oscillator = use_state(|| osc1);
    

    let key_map_setter = keycode_maps.setter();
    let key_map_down = keycode_maps.clone();
    // let cloned_audio_context = audio_context.clone();
    let cloned_poly = polyphony.clone();
    // let cloned_sound = sound.clone();
    let cloned_device = device.clone();
    let cloned_config = config.clone();
    let stream_setter = stream.setter();
    let cloned_oscillator = oscillator.clone();
    let mouse_down = Callback::from(move |label: char| {
        let key_label = key_map_down.get(&label).unwrap_or(&0);
        log!("Holding key", label.to_string(), ", MIDI Note:", key_label.to_string());
        let cloned_key_map = &mut key_map_down.deref().clone();
        let mut buffer = cloned_poly.deref().clone();
        let device_temp = cloned_device.deref().clone();
        let config_temp = cloned_config.deref().clone();
        // let context = cloned_audio_context.deref().clone();
        let mut oscillator_type = cloned_oscillator.deref().clone();
        match label {
            'Z' => {
                decrease_octave(cloned_key_map);
                // for (_, val) in buffer.iter_mut() {
                //     val.stop().expect("Failed to stop oscillator");
                //     val.disconnect_with_audio_node(&context.destination()).expect("Could not disconnect from audio node");
                // }
                buffer.clear();
                let new_stream = State::new(&device_temp, &config_temp, buffer.clone());
                new_stream.pause();
                stream_setter.set(new_stream);
                cloned_poly.set(buffer);
                key_map_setter.set(cloned_key_map.deref().clone());
            },
            'X' => {
                increase_octave(cloned_key_map);
                // for (_, val) in buffer.iter_mut() {
                //     val.stop().expect("Failed to stop oscillator");
                //     val.disconnect_with_audio_node(&context.destination()).expect("Could not disconnect from audio node");
                // }
                buffer.clear();
                let new_stream = State::new(&device_temp, &config_temp, buffer.clone());
                new_stream.pause();
                stream_setter.set(new_stream);
                cloned_poly.set(buffer);
                key_map_setter.set(cloned_key_map.deref().clone());
            },
            '1' => {
                // OSCILLATOR_TYPE = Some(web_sys::OscillatorType::Sine);
                oscillator_type = MultiOscillator::from(WaveTableOscillator::new(sample_rate, 44100, Oscillator::Sine, 0.8, 0.0));
                cloned_oscillator.set(oscillator_type);
                log!("Sine wave selected");
            },
            '2' => {
                oscillator_type = MultiOscillator::from(WaveTableOscillator::new(sample_rate, 44100, Oscillator::Square, 0.8, 0.0));
                cloned_oscillator.set(oscillator_type);
                log!("Square wave selected");
            },
            '3' => {
                oscillator_type = MultiOscillator::from(WaveTableOscillator::new(sample_rate, 44100, Oscillator::Saw, 0.8, 0.0));
                cloned_oscillator.set(oscillator_type);
                log!("Sawtooth wave selected");
            },
            '4' => {
                oscillator_type = MultiOscillator::from(WaveTableOscillator::new(sample_rate, 44100, Oscillator::Triangle, 0.8, 0.0));
                cloned_oscillator.set(oscillator_type);
                log!("Triangle wave selected");
            },

            _ => {
                // let osc = context.create_oscillator().expect("Could not create oscillator");
                // // let gain = context.create_gain().expect("Could not create gain");
                // // gain.connect_with_audio_node(&context.destination()).expect("Could not connect gain to audio node");
                // osc.connect_with_audio_node(&context.destination()).expect("Could not connect oscillator to audio node");
                // osc.set_type(web_sys::OscillatorType::Sawtooth);
                // osc.frequency().set_value(midi_to_hz(*key_label).ok().unwrap());
                let frequency = midi_to_hz(*key_label).unwrap_or(1.0);
                let mut source = cloned_oscillator.deref().clone();
                let _ = source.global_set_frequency(frequency);
                buffer.insert(*key_label, source);
                let new_stream = State::new(&device_temp, &config_temp, buffer.clone());
                new_stream.play();
                stream_setter.set(new_stream);
                cloned_poly.set(buffer);
            }
        }
    });

    let key_map_up = keycode_maps.clone();
    let cloned_poly = polyphony.clone();
    let stream_setter = stream.setter();
    let cloned_device = device.clone();
    let cloned_config = config.clone();
    // let cloned_sound = sound.clone();
    // let cloned_audio_context = audio_context.clone();
    let mouse_up = Callback::from(move |label: char| {
        let key_label = key_map_up.get(&label).unwrap_or(&0);
        let mut buffer = cloned_poly.deref().clone();
        // let context = cloned_audio_context.deref().clone();
        let _ = buffer.remove(key_label);
        // let stream_inside = cloned_stream.deref();
        let device_temp = cloned_device.deref().clone();
        let config_temp = cloned_config.deref().clone();
        let new_stream = State::new(&device_temp, &config_temp, buffer.clone());
        new_stream.play();
        stream_setter.set(new_stream);
        cloned_poly.set(buffer);
        log!("Lifted key", label.to_string(), ", MIDI Note:", key_map_up.get(&label).unwrap_or(&0).to_string());
    });

    let key_map_setter = keycode_maps.setter();
    let key_map_down = keycode_maps.clone();
    // let cloned_audio_context = audio_context.clone();
    let cloned_poly = polyphony.clone();
    // let cloned_sound = sound.clone();
    let cloned_device = device.clone();
    let cloned_config = config.clone();
    let stream_setter = stream.setter();
    let cloned_oscillator = oscillator.clone();
    let key_down = Callback::from(move |label: char| {
        let key_label = key_map_down.get(&label).unwrap_or(&0);
        let cloned_key_map = &mut key_map_down.deref().clone();
        // let context = cloned_audio_context.deref().clone();
        let mut buffer = cloned_poly.deref().clone();
        let device_temp = cloned_device.deref().clone();
        let config_temp = cloned_config.deref().clone();
        let mut oscillator_type = cloned_oscillator.deref().clone();
        match label {
            'Z' => {
                decrease_octave(cloned_key_map);
                buffer.clear();
                let new_stream = State::new(&device_temp, &config_temp, buffer.clone());
                new_stream.pause();
                stream_setter.set(new_stream);
                cloned_poly.set(buffer);
                key_map_setter.set(cloned_key_map.deref().clone());
            },
            'X' => {
                increase_octave(cloned_key_map);
                buffer.clear();
                let new_stream = State::new(&device_temp, &config_temp, buffer.clone());
                new_stream.pause();
                stream_setter.set(new_stream);
                cloned_poly.set(buffer);
                key_map_setter.set(cloned_key_map.deref().clone());
            },
            '1' => {
                // OSCILLATOR_TYPE = Some(web_sys::OscillatorType::Sine);
                oscillator_type = MultiOscillator::from(WaveTableOscillator::new(sample_rate, 44100, Oscillator::Sine, 0.8, 0.0));
                cloned_oscillator.set(oscillator_type);
                log!("Sine wave selected");
            },
            '2' => {
                oscillator_type = MultiOscillator::from(WaveTableOscillator::new(sample_rate, 44100, Oscillator::Square, 0.8, 0.0));
                cloned_oscillator.set(oscillator_type);
                log!("Square wave selected");
            },
            '3' => {
                oscillator_type = MultiOscillator::from(WaveTableOscillator::new(sample_rate, 44100, Oscillator::Saw, 0.8, 0.0));
                cloned_oscillator.set(oscillator_type);
                log!("Sawtooth wave selected");
            },
            '4' => {
                oscillator_type = MultiOscillator::from(WaveTableOscillator::new(sample_rate, 44100, Oscillator::Triangle, 0.8, 0.0));
                cloned_oscillator.set(oscillator_type);
                log!("Triangle wave selected");
            },
            _ => {
                if cloned_key_map.contains_key(&label) {
                    match buffer.get(key_label) {
                        Some(_) => (),
                        None => {
                            let frequency = midi_to_hz(*key_label).unwrap_or(1.0);
                            let mut source = cloned_oscillator.deref().clone();
                            let _ = source.global_set_frequency(frequency);
                            buffer.insert(*key_label, source);
                            let new_stream = State::new(&device_temp, &config_temp, buffer.clone());
                            new_stream.play();
                            stream_setter.set(new_stream);
                            cloned_poly.set(buffer);
                        }
                    }
                }
            }
        }
        log!("Holding key", label.to_string(), ", MIDI Note:", key_label.to_string());
    });

    let key_map_up = keycode_maps.clone();
    let cloned_poly = polyphony.clone();
    let stream_setter = stream.setter();
    let cloned_device = device.clone();
    let cloned_config = config.clone();
    // let cloned_audio_context = audio_context.clone();
    let key_up = Callback::from(move |label: char| {
        let key_label = key_map_up.get(&label).unwrap_or(&0);
        let mut buffer = cloned_poly.deref().clone();
        // let context = cloned_audio_context.deref().clone();
        let _ = buffer.remove(key_label);
        let device_temp = cloned_device.deref().clone();
        let config_temp = cloned_config.deref().clone();
        let new_stream = State::new(&device_temp, &config_temp, buffer.clone());
        new_stream.play();
        stream_setter.set(new_stream);
        cloned_poly.set(buffer);
        log!("Lifted key", label.to_string(), ", MIDI Note:", key_map_up.get(&label).unwrap_or(&0).to_string());
    });

    let key_map_clone = keycode_maps.clone();
    html! {
        <>
            <h1>{"Choose Your Oscillator Type"}</h1>
            <OscillatorSelector mouse_down={mouse_down.clone()} mouse_up={mouse_up.clone()} />
            <h1>{"Choose Your Filter Type"}</h1>
            <FilterSelector mouse_down={mouse_down.clone()} mouse_up={mouse_up.clone()} />
            <MIDIKeyboard mouse_down={mouse_down.clone()} mouse_up={&mouse_up} key_down={&key_down} key_up={&key_up}/>
            <p>{"Current MIDI Range: "}{&key_map_clone.deref()[&'A']}{" - "}{&key_map_clone.deref()[&'K']}</p>
        </>
    }
}