//! # Browser Based Synthesizer
//!
//! This crate contains the implementation of a synthesizer application using the Yew framework.
//!
//! ## Components
//!
//! - `app`: The main component representing the entire application. It manages state, sets up audio streams,
//!         and orchestrates interactions between other synth and UI components.
//! - `display_oscillators`: A helper function to display oscillator selectors dynamically based on the number
//!                          of oscillators in the synthesizer.
//!
//! ## State Management
//!
//! This application uses Yew's `use_state` for managing component state. Each component manages its own state
//! using hooks provided by Yew.
//!
//! ## Event Handling
//!
//! Events such as mouse clicks and key presses are handled using Yew's event system. Callbacks are used to
//! handle these events and update the component state accordingly.
//!
//! ## Code Structure
//!
//! The code is organized into modules representing different aspects of the application, such as oscillators,
//! filters, envelopes, etc. Components are structured hierarchically, with the main `App` component at the top.
//!
//! ## External Dependencies
//!
//! This application relies on the following external crates:
//!
//! - `synth_backend`: Provides backend functionality for the synthesizer.
//! - `synth_frontend`: Contains frontend components for the synthesizer UI.
//! - `yew`: The web framework used for building the user interface.
//! - `stylist`: Provides styling utilities for Yew components.
//! - `gloo`: Provides utilities for web APIs.
//! - `cpal`: Used for audio input and output.
//!
//! For detailed documentation of each crate, refer to their respective documentation.


#[doc(include = "synth/src/lib.rs")]


use std::collections::HashMap;
use std::{ops::Deref, sync::{Arc, Mutex}};
use synth_backend::{filters::FilterParam, ring_buffer::IterablePolyphonyHashMap, utils::{decrease_octave, increase_octave}};
use synth_backend::oscillators::{MultiOscillator, Oscillator, WaveTableOscillator};
use synth_backend::envelopes::{EnvelopeParam, Envelope};
use synth_backend::lfo::{LFO, LFOType};
use yew::prelude::*;
use stylist::yew::styled_component;
use gloo::console::log;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SampleRate, SizedSample, Stream, StreamConfig};

use synth_frontend::{components::organisms::lfo_settings::LFOSelector, MIDIKeyboard};
use synth_frontend::components::molecules::add_button::AddButton;
use synth_frontend::components::organisms::{oscillator_selector::OscillatorSelector, filter_selector::FilterSelector, envelope_settings::EnvelopeSettings};
use synth_backend::utils::{midi_to_hz, create_stream};
use synth_backend::filters::{Filter, FilterType};
use synth_backend::wrapper::Synth;

const OVERALL_CSS: &str = include_str!("../../synth-frontend/src/UI_components/overall.css");

#[styled_component(App)]
pub fn app() -> Html {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No default output device found");
    let mut supported_configs_range = device.supported_output_configs()
    .expect("error while querying configs");
    let supported_config = supported_configs_range.next()
        .expect("no supported config?!")
        .with_max_sample_rate();
    let sample_rate = supported_config.sample_rate().0;
    let supported_configs = device.supported_output_configs().unwrap();
    let config = 
        match supported_configs
        .filter(|c| c.channels() == 1)
        .max_by(|a, b| a.cmp_default_heuristics(b)) {
            Some(config) => config.with_sample_rate(SampleRate(sample_rate)).into(),
            _ => panic!("No supported configuration found for output device")
        };
    // log!(sample_rate);
    // let config = use_state(|| supported_config.into());
    let polyphony = use_state(|| Arc::new(Mutex::new(IterablePolyphonyHashMap::new(sample_rate))));
    let stream = use_state(|| create_stream(&device, &config, Arc::clone(polyphony.deref())));
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

    let freq: UseStateHandle<f32> = use_state(|| 200.0);
    let filter_type = use_state(|| FilterType::HighPass);
    let bandwidth_hz = 500.0;
    let filter = Filter::new(
        filter_type.deref().clone(), 
        sample_rate as f32, 
        *freq.deref(), 
        bandwidth_hz
    );

    let attack_ms = use_state(|| 0.0);
    let decay_ms = use_state(|| 0.0);
    let sustain_percentage = use_state(|| 1.0);
    let release_ms = 0.0;
    let envelope = Envelope::new(sample_rate as f32, *attack_ms.deref(), *decay_ms.deref(), *sustain_percentage.deref(), release_ms);

    let lfo_freq = use_state(|| 0.01);
    let lfo_type = use_state(|| LFOType::Amplitude);
    // let am_lfo = WaveTableOscillator::new(sample_rate, 44100, Oscillator::Sine, 0.8, *lfo_freq.deref());
    // let mut osillator = WaveTableOscillator::new(sample_rate, 44100, Oscillator::Sine, 1.0, 0.0);
    // let mut lfo = LFO::new(LFOType::Amplitude, sample_rate as f32,  osillator, 0.0015);
    // lfo.set_frequency(5.0);
    // lfo.set_width(0.0015);
    // lfo.set_type(LFOType::Frequency);
    // lfo.set_oscillator(Oscillator::Triangle);

    let gain = use_state(|| vec![0.5]);
    let detune_semitones = use_state(|| vec![0]);
    let osc1 = MultiOscillator::from(WaveTableOscillator::new(sample_rate, 44100, Oscillator::Sine, gain.deref().clone()[0], 0.0));
    let oscillator = use_state(|| Synth::new(
        osc1,
        sample_rate,
        Some(filter),
        Some(envelope),
        None,
        lfo_type.deref().clone()
    ));

    let cloned_oscillator = oscillator.clone();
    let cloned_attack = attack_ms.clone();
    let attack_change = Callback::from(move |attack: f64| {
        cloned_attack.set(attack as f32);
        let mut oscillator_type = cloned_oscillator.deref().clone();
        oscillator_type.set_envelope_params(EnvelopeParam::AttackMs, attack as f32);
        cloned_oscillator.set(oscillator_type);
        // No need to implement changes in the (already playing) notes
    });

    let cloned_oscillator = oscillator.clone();
    let cloned_decay = decay_ms.clone();
    let decay_change = Callback::from(move |decay: f64| {
        cloned_decay.set(decay as f32);
        let mut oscillator_type = cloned_oscillator.deref().clone();
        oscillator_type.set_envelope_params(EnvelopeParam::DecayMs, decay as f32);
        cloned_oscillator.set(oscillator_type);
    });
    

    let cloned_oscillator = oscillator.clone();
    let cloned_sustain = sustain_percentage.clone();
    let sustain_change = Callback::from(move |sustain: f64| {
        cloned_sustain.set(sustain as f32);
        let mut oscillator_type = cloned_oscillator.deref().clone();
        oscillator_type.set_envelope_params(EnvelopeParam::SustainPercentage, sustain as f32);
        cloned_oscillator.set(oscillator_type);
    });
    
    
    let cloned_oscillator = oscillator.clone();
    let cloned_freq = freq.clone();
    let cloned_poly = polyphony.clone();
    let freq_change = Callback::from(move |freq: f64| {
        cloned_freq.set(freq as f32);
        let mut oscillator_type = cloned_oscillator.deref().clone();
        oscillator_type.set_filter_params(FilterParam::FreqHz, freq as f32);
        oscillator_type.set_filter_params(FilterParam::BandwidthHz, freq as f32*0.5);
        cloned_oscillator.set(oscillator_type);
        let buffer = Arc::clone(cloned_poly.deref());
        for (_, synths) in buffer.lock().unwrap().iterate_hashmap_mut() {
            let _ = synths.set_filter_params(FilterParam::FreqHz, freq as f32);
            let _ = synths.set_filter_params(FilterParam::BandwidthHz, freq as f32*0.5);
        }
    });

    let cloned_oscillator = oscillator.clone();
    let cloned_freq_lfo = lfo_freq.clone();
    let cloned_poly = polyphony.clone();
    let freq_lfo_change = Callback::from(move |freq: f64| {
        cloned_freq_lfo.set(freq as f32);
        let mut oscillator_type = cloned_oscillator.deref().clone();
        let _ = oscillator_type.set_lfo_frequency(freq as f32);
        cloned_oscillator.set(oscillator_type);
        let buffer = Arc::clone(cloned_poly.deref());
        for (_, synths) in buffer.lock().unwrap().iterate_hashmap_mut() {
            let _ = synths.set_lfo_frequency(freq as f32);
        }
    });

    let active_oscillators = use_state(|| vec![0; oscillator.deref().num_sources()]);
    let active_lfo = use_state(|| 0);
    let active_lfo_type = use_state(|| 0);
    let active_filter = use_state(|| 0);

    let key_map_setter = keycode_maps.setter();
    let key_map_down = keycode_maps.clone();
    let cloned_poly = polyphony.clone();
    let cloned_oscillator = oscillator.clone();
    let cloned_freq = freq.clone();
    let cloned_active_osc = active_oscillators.clone();
    let cloned_active_lfo = active_lfo.clone();
    let cloned_active_lfo_type = active_lfo_type.clone();
    let cloned_active_filter = active_filter.clone();
    let cloned_freq_lfo = lfo_freq.clone();
    let cloned_type_lfo = lfo_type.clone();
    let cloned_osc_gain = gain.clone();
    let cloned_osc_detune = detune_semitones.clone();
    let mouse_down = Callback::from(move |label: (char, usize)| {
        let key_label = key_map_down.get(&label.0).unwrap_or(&0);
        log!("Holding key", label.0.to_string(), ", MIDI Note:", key_label.to_string());
        let cloned_key_map = &mut key_map_down.deref().clone();
        let buffer = Arc::clone(cloned_poly.deref());
        let mut oscillator_type = cloned_oscillator.deref().clone();
        let freq_filter = cloned_freq.deref().clone();
        let bandwidth_hz_filter = freq_filter*0.5;
        let mut active_indices = cloned_active_osc.deref().clone();
        let mut active_lfo_index = cloned_active_lfo.deref().clone();
        let mut active_lfo_type_index = cloned_active_lfo_type.deref().clone();
        let mut active_filter_index = cloned_active_filter.deref().clone();
        let lfo_freq = cloned_freq_lfo.deref().clone();
        let lfo_type = cloned_type_lfo.deref().clone();
        let mut list_of_gains = cloned_osc_gain.deref().clone();
        let mut list_of_detunes = cloned_osc_detune.deref().clone();
        let new_buffer = Arc::clone(&buffer);
        match label.0 {
            'Z' => {
                if cloned_key_map[&'A'] > 12 {
                    decrease_octave(cloned_key_map);
                    let _ = new_buffer.lock().unwrap().clear();
                    key_map_setter.set(cloned_key_map.deref().clone());
                }
            },
            'X' => {
                if cloned_key_map[&'A'] < 108 {
                    increase_octave(cloned_key_map);
                    let _ = new_buffer.lock().unwrap().clear();
                    key_map_setter.set(cloned_key_map.deref().clone());
                }
            },
            '1' => {
                if label.1>0 {
                    oscillator_type.set_oscillator(label.1 - 1, Oscillator::Sine);
                    for (_, synths) in buffer.lock().unwrap().iterate_hashmap_mut() {
                        let _ = synths.set_oscillator(label.1 - 1, Oscillator::Sine);
                    }
                    active_indices[label.1 - 1] = 0;
                    log!("Sine wave selected");
                }
            },
            '2' => {
                if label.1>0 {
                    oscillator_type.set_oscillator(label.1 - 1, Oscillator::BidirectionalSquare);
                    for (_, synths) in buffer.lock().unwrap().iterate_hashmap_mut() {
                        let _ = synths.set_oscillator(label.1 - 1, Oscillator::BidirectionalSquare);
                    }
                    active_indices[label.1 - 1] = 1;
                    log!("Square wave selected");
                }
            },
            '3' => {
                if label.1>0 {
                    oscillator_type.set_oscillator(label.1 - 1, Oscillator::Saw);
                    for (_, synths) in buffer.lock().unwrap().iterate_hashmap_mut() {
                        let _ = synths.set_oscillator(label.1 - 1, Oscillator::Saw);
                    }
                    active_indices[label.1 - 1] = 2;
                    log!("Sawtooth wave selected");
                }
            },
            '4' => {
                if label.1>0 {
                    oscillator_type.set_oscillator(label.1 - 1, Oscillator::Triangle);
                    for (_, synths) in buffer.lock().unwrap().iterate_hashmap_mut() {
                        let _ = synths.set_oscillator(label.1 - 1, Oscillator::Triangle);
                    }
                    active_indices[label.1 - 1] = 3;
                    log!("Triangle wave selected");
                }
            },
            '5' => {
                if label.1>0 {
                    oscillator_type.set_oscillator(label.1 - 1, Oscillator::WhiteNoise);
                    for (_, synths) in buffer.lock().unwrap().iterate_hashmap_mut() {
                        let _ = synths.set_oscillator(label.1 - 1, Oscillator::WhiteNoise);
                    }
                    active_indices[label.1 - 1] = 4;
                    log!("White Noise wave selected");
                }
            },
            '0' => {
                oscillator_type.set_filter(Some(FilterType::HighPass), freq_filter, bandwidth_hz_filter);
                for (_, synths) in buffer.lock().unwrap().iterate_hashmap_mut() {
                    let _ = synths.set_filter(Some(FilterType::HighPass), freq_filter, bandwidth_hz_filter);
                }
                active_filter_index = 1;
                log!("High pass selected");
            },
            '9' => {
                oscillator_type.set_filter(Some(FilterType::BandPass), freq_filter, bandwidth_hz_filter);
                for (_, synths) in buffer.lock().unwrap().iterate_hashmap_mut() {
                    let _ = synths.set_filter(Some(FilterType::BandPass), freq_filter, bandwidth_hz_filter);
                }
                active_filter_index = 2;
                log!("Band pass selected");
            },
            '8' => {
                oscillator_type.set_filter(Some(FilterType::LowPass), freq_filter, bandwidth_hz_filter);
                for (_, synths) in buffer.lock().unwrap().iterate_hashmap_mut() {
                    let _ = synths.set_filter(Some(FilterType::LowPass), freq_filter, bandwidth_hz_filter);
                }
                active_filter_index = 3;
                log!("Low pass selected");
            },
            '7' => {
                oscillator_type.set_filter(None, freq_filter, bandwidth_hz_filter);
                for (_, synths) in buffer.lock().unwrap().iterate_hashmap_mut() {
                    let _ = synths.set_filter(None, freq_filter, bandwidth_hz_filter);
                }
                active_filter_index = 0;
                log!("Filter off");
            },
            '+' => {
                let _ = oscillator_type.push(WaveTableOscillator::new(sample_rate, 44100, Oscillator::Sine, 0.7, 0.0));
                for (_, synths) in buffer.lock().unwrap().iterate_hashmap_mut() {
                    let _ = synths.push(WaveTableOscillator::new(sample_rate, 44100, Oscillator::Sine, 0.7, 0.0));
                }
                active_indices.push(0);
                list_of_gains.push(0.5);
                list_of_detunes.push(0);
                log!("Add an oscillator");
            }
            '-' => {
                if oscillator_type.num_sources() > 1 {
                    let _ = oscillator_type.remove(label.1 - 1);
                    for (_, synths) in buffer.lock().unwrap().iterate_hashmap_mut() {
                        let _ = synths.remove(label.1 - 1);
                    }
                    list_of_gains.remove(label.1 - 1);
                    list_of_detunes.remove(label.1 - 1);
                    active_indices.remove(label.1 - 1);
                }
            },
            '<' => {
                oscillator_type.set_lfo_type(LFOType::Amplitude);
                for (_, synths) in buffer.lock().unwrap().iterate_hashmap_mut() {
                    let _ = synths.set_lfo_type(LFOType::Amplitude);
                }
                active_lfo_type_index = 0;
            },
            '>' => {
                oscillator_type.set_lfo_type(LFOType::Frequency);
                for (_, synths) in buffer.lock().unwrap().iterate_hashmap_mut() {
                    let _ = synths.set_lfo_type(LFOType::Frequency);
                }
                active_lfo_type_index = 1;
            },
            '|' => {
                oscillator_type.set_lfo_osc(None, lfo_freq, lfo_type.clone());
                for (_, synths) in buffer.lock().unwrap().iterate_hashmap_mut() {
                    let _ = synths.set_lfo_osc(None, lfo_freq, lfo_type.clone());
                }
                active_lfo_index = 0;
            },
            '[' => {
                oscillator_type.set_lfo_osc(Some(Oscillator::Sine), lfo_freq, lfo_type.clone());
                for (_, synths) in buffer.lock().unwrap().iterate_hashmap_mut() {
                    let _ = synths.set_lfo_osc(Some(Oscillator::Sine), lfo_freq, lfo_type.clone());
                }
                active_lfo_index = 1;
            },
            ']' => {
                oscillator_type.set_lfo_osc(Some(Oscillator::Square), lfo_freq, lfo_type.clone());
                for (_, synths) in buffer.lock().unwrap().iterate_hashmap_mut() {
                    let _ = synths.set_lfo_osc(Some(Oscillator::Square), lfo_freq, lfo_type.clone());
                }
                active_lfo_index = 2;
            },
            '{' => {
                oscillator_type.set_lfo_osc(Some(Oscillator::Saw), lfo_freq, lfo_type.clone());
                for (_, synths) in buffer.lock().unwrap().iterate_hashmap_mut() {
                    let _ = synths.set_lfo_osc(Some(Oscillator::Saw), lfo_freq, lfo_type.clone());
                }
                active_lfo_index = 3;
            },
            '}' => {
                oscillator_type.set_lfo_osc(Some(Oscillator::Triangle), lfo_freq, lfo_type.clone());
                for (_, synths) in buffer.lock().unwrap().iterate_hashmap_mut() {
                    let _ = synths.set_lfo_osc(Some(Oscillator::Triangle), lfo_freq, lfo_type.clone());
                }
                active_lfo_index = 4;
            },
            _ => {
                let frequency = midi_to_hz(*key_label).unwrap_or(1.0);
                let mut source = cloned_oscillator.deref().clone();
                let _ = source.global_set_frequency(frequency);
                new_buffer.lock().unwrap().insert(*key_label, source.clone());
            }
        }
        cloned_poly.set(buffer);
        cloned_oscillator.set(oscillator_type);
        cloned_active_osc.set(active_indices);
        cloned_active_lfo.set(active_lfo_index);
        cloned_active_lfo_type.set(active_lfo_type_index);
        cloned_active_filter.set(active_filter_index);
        cloned_osc_gain.set(list_of_gains);
        cloned_osc_detune.set(list_of_detunes);
    });

    let key_map_up = keycode_maps.clone();
    let cloned_poly = polyphony.clone();
    let mouse_up = Callback::from(move |label: (char, usize)| {
        let key_label = key_map_up.get(&label.0).unwrap_or(&0);
        let buffer = cloned_poly.deref().clone();
        let new_buffer = Arc::clone(&buffer);
        let _ = new_buffer.lock().unwrap().remove(key_label);
        cloned_poly.set(buffer);
        log!("Lifted key", label.0.to_string(), ", MIDI Note:", key_map_up.get(&label.0).unwrap_or(&0).to_string());
    });

    let key_map_setter = keycode_maps.setter();
    let key_map_down = keycode_maps.clone();
    let cloned_poly = polyphony.clone();
    let cloned_oscillator = oscillator.clone();
    let key_down = Callback::from(move |label: char| {
        let key_label = key_map_down.get(&label).unwrap_or(&0);
        let cloned_key_map = &mut key_map_down.deref().clone();
        let buffer = Arc::clone(cloned_poly.deref());
        let new_buffer = Arc::clone(&buffer);
        match label {
            'Z' => {
                if cloned_key_map[&'A'] > 12 {
                    decrease_octave(cloned_key_map);
                    let _ = new_buffer.lock().unwrap().clear();
                    key_map_setter.set(cloned_key_map.deref().clone());
                }
            },
            'X' => {
                if cloned_key_map[&'A'] < 108 {
                    increase_octave(cloned_key_map);
                    let _ = new_buffer.lock().unwrap().clear();
                    key_map_setter.set(cloned_key_map.deref().clone());
                }
            },
            _ => {
                if cloned_key_map.contains_key(&label) {
                    let new_buffer = Arc::clone(&buffer);
                    let exists_label = match new_buffer.lock().unwrap().get(key_label) {
                        None => None,
                        Some(_) => Some(true)
                    };
                    // });
                    match exists_label {
                        Some(_) => (),
                        None => {
                            let frequency = midi_to_hz(*key_label).unwrap_or(1.0);
                            let mut source = cloned_oscillator.deref().clone();
                            let _ = source.global_set_frequency(frequency);
                            let _ = new_buffer.lock().unwrap().insert(*key_label, source);
                        }
                    }
                }
            }
        }
        cloned_poly.set(buffer);
        log!("Holding key", label.to_string(), ", MIDI Note:", key_label.to_string());
    });

    let key_map_up = keycode_maps.clone();
    let cloned_poly = polyphony.clone();
    let key_up = Callback::from(move |label: char| {
        let key_label = key_map_up.get(&label).unwrap_or(&0);
        let buffer = Arc::clone(cloned_poly.deref());
        let _ = buffer.lock().unwrap().remove(key_label);
        cloned_poly.set(buffer);
        log!("Lifted key", label.to_string(), ", MIDI Note:", key_map_up.get(&label).unwrap_or(&0).to_string());
    });
    let overall_css = stylist::Style::new(OVERALL_CSS).unwrap();
    let key_map_clone = keycode_maps.clone();
    let oscillator_selector_display: Vec<Html> = display_oscillators(
        mouse_down.clone(), 
        mouse_up.clone(), 
        key_up.clone(), 
        key_down.clone(), 
        oscillator.clone(),
        gain.clone(),
        detune_semitones.clone(),
        polyphony.clone(),
        active_oscillators.deref().clone()
    );
    html! {

        <div class= {overall_css}>
        <div class = "header">
            <img src="https://i.ibb.co/VDXtLD5/mr-krabb.png" alt="Mr. Krabs" border="0"/>
            <img src="https://i.ibb.co/YDBhvC5/Rusty-Krabby-Synthie-4-25-2024-2.png" alt="Rusty Krabby Synthie" border="0"/>
            <img src="https://i.ibb.co/ryZDfXg/sponge-bob-transparent.png" alt="SpongeBob" border="0"/>

        </div>

        <div class="parameters">
            
            <div class="column1">
                <h1>{"Oscillator"}</h1>
                {oscillator_selector_display}
                <br />
                <AddButton on_mouse_down={mouse_down.clone()} on_mouse_up={mouse_up.clone()} />
                
            </div>
            <div class="column2">
            <h1>{"Filter"}</h1>
            <FilterSelector mouse_down={mouse_down.clone()} mouse_up={mouse_up.clone()} freq_change={freq_change} freq={*freq.deref() as f64} active_index={active_filter.deref()}/>
            <h1>{"LFO"}</h1>
            <LFOSelector mouse_down={mouse_down.clone()} mouse_up={mouse_up.clone()} freq_change={freq_lfo_change} active_index={active_lfo.deref()} active_index_type={active_lfo_type.deref()} freq={*lfo_freq.deref() as f64}/>
            <h1>{"Envelope"}</h1>
            <EnvelopeSettings attack_change={attack_change} decay_change={decay_change} sustain_change={sustain_change} attack={*attack_ms.deref() as f64} decay={*decay_ms.deref() as f64} sustain={*sustain_percentage.deref() as f64}/>
                
            </div>

        </div>
        <div class="row">

     
            <MIDIKeyboard mouse_down={mouse_down.clone()} mouse_up={&mouse_up} key_down={&key_down} key_up={&key_up}/>
            <p style="color: white">{"Current MIDI Range: "}{&key_map_clone.deref()[&'A']}{" - "}{&key_map_clone.deref()[&'K']}</p>
        </div>


        </div>
        

    }

}

pub fn display_oscillators(
    mouse_down: Callback<(char, usize)>, 
    mouse_up: Callback<(char, usize)>, 
    key_down: Callback<char>, 
    key_up: Callback<char>,
    oscillator: UseStateHandle<Synth>, 
    gain: UseStateHandle<Vec<f32>>,
    detune_semitones: UseStateHandle<Vec<i8>>,
    polyphony: UseStateHandle<Arc<Mutex<IterablePolyphonyHashMap>>>,
    active_indices: Vec<usize>
) -> Vec<Html>{
    let mut display = Vec::new();
    for idx in 0..oscillator.num_sources() {
        let cloned_oscillator: UseStateHandle<Synth> = oscillator.clone();
        let cloned_gain = gain.clone();
        let idx_gain = gain.deref()[idx];
        let cloned_gain_set = gain.setter();
        let cloned_poly = polyphony.clone();
        let gain_change = Callback::from(move |gain1: f64| {
            let mut gain_vec = cloned_gain.deref().clone();
            gain_vec[idx] = gain1 as f32;
            cloned_gain_set.set(gain_vec);
            let mut oscillator_type = cloned_oscillator.deref().clone();
            let _ = oscillator_type.set_gain(idx, gain1 as f32);
            cloned_oscillator.set(oscillator_type);
            let buffer = Arc::clone(cloned_poly.deref());
            for (_, synths) in buffer.lock().unwrap().iterate_hashmap_mut() {
                let _ = synths.set_gain(idx, gain1 as f32);
            }
        });
        let cloned_oscillator: UseStateHandle<Synth> = oscillator.clone();
        let cloned_detune = detune_semitones.clone();
        let idx_detune = detune_semitones.deref()[idx];
        let cloned_detune_set = detune_semitones.setter();
        // let cloned_poly = polyphony.clone();
        let detune_change = Callback::from(move |detune: i8| {
            let mut detune_vec = cloned_detune.deref().clone();
            detune_vec[idx] = detune;
            cloned_detune_set.set(detune_vec);
            let mut oscillator_type = cloned_oscillator.deref().clone();
            let _ = oscillator_type.set_detune_semitones(idx, detune);
            cloned_oscillator.set(oscillator_type);

            // Since it is an IntSlider, no need to create dynamic changes

            // let buffer = Arc::clone(cloned_poly.deref());
            // for (_, synths) in buffer.lock().unwrap().iterate_hashmap_mut() {
            //     let _ = synths.set_detune_semitones(idx, detune);
            // }
        });
        display.push(html! {
            <OscillatorSelector 
                mouse_down={mouse_down.clone()} 
                mouse_up={mouse_up.clone()} 
                gain_change={gain_change}
                detune_change={detune_change}
                gain={idx_gain as f64}
                detune={idx_detune}
                number={idx as usize+1} 
                active_index={active_indices[idx]}
            />
        })
    }
    display
}
