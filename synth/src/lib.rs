use std::collections::HashMap;
use std::ops::Deref;
use synth_backend::{filters::FilterParam, ring_buffer::IterablePolyphonyHashMap, utils::{decrease_octave, increase_octave}};
use synth_backend::oscillators::{MultiOscillator, Oscillator, WaveTableOscillator};
use synth_backend::envelopes::{EnvelopeParam, Envelope};
use yew::prelude::*;
use stylist::yew::styled_component;
use gloo::console::log;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SampleRate, SizedSample, Stream, StreamConfig};

use synth_frontend::{components::organisms::lfo_settings::LFOSelector, MIDIKeyboard};
use synth_frontend::components::molecules::add_button::AddButton;
use synth_frontend::components::organisms::{oscillator_selector::OscillatorSelector, filter_selector::FilterSelector, envelope_settings::EnvelopeSettings};
use synth_backend::utils::{midi_to_hz, State};
use synth_backend::filters::{Filter, FilterType};
use synth_backend::wrapper::Synth;


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
    log!(sample_rate);
    // let config = use_state(|| supported_config.into());
    let polyphony: UseStateHandle<IterablePolyphonyHashMap> = use_state(|| IterablePolyphonyHashMap::new(sample_rate));
    let stream = use_state(|| State::new(&device, &config, polyphony.deref().clone()));
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

    let lfo_freq = use_state(|| 0.0);
    let am_lfo = WaveTableOscillator::new(sample_rate, 44100, Oscillator::Sine, 1.0, *lfo_freq.deref());

    let osc1 = MultiOscillator::from(WaveTableOscillator::new(sample_rate, 44100, Oscillator::Sine, 0.5, 0.0));
    let oscillator = use_state(|| Synth::new(
        osc1,
        sample_rate,
        Some(filter),
        Some(envelope),
        Some(am_lfo)
    ));

    let cloned_oscillator = oscillator.clone();
    let cloned_attack = attack_ms.clone();
    let attack_change = Callback::from(move |attack: f64| {
        cloned_attack.set(attack as f32);
        let mut oscillator_type = cloned_oscillator.deref().clone();
        oscillator_type.set_envelope_params(EnvelopeParam::AttackMs, attack as f32);
        cloned_oscillator.set(oscillator_type);
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
    let freq_change = Callback::from(move |freq: f64| {
        cloned_freq.set(freq as f32);
        let mut oscillator_type = cloned_oscillator.deref().clone();
        oscillator_type.set_filter_params(FilterParam::FreqHz, freq as f32);
        oscillator_type.set_filter_params(FilterParam::BandwidthHz, freq as f32*0.5);
        cloned_oscillator.set(oscillator_type);
    });

    let cloned_oscillator = oscillator.clone();
    let cloned_freq_lfo = lfo_freq.clone();
    let freq_lfo_change = Callback::from(move |freq: f64| {
        cloned_freq_lfo.set(freq as f32);
        let mut oscillator_type = cloned_oscillator.deref().clone();
        let _ = oscillator_type.set_lfo_frequency(freq as f32);
        cloned_oscillator.set(oscillator_type);
    });

    let active_oscillators = use_state(|| vec![0; oscillator.deref().num_sources()]);
    let active_lfo = use_state(|| 0);

    let key_map_setter = keycode_maps.setter();
    let key_map_down = keycode_maps.clone();
    let cloned_poly = polyphony.clone();
    let cloned_device = device.clone();
    let cloned_config = config.clone();
    let stream_setter = stream.setter();
    let cloned_oscillator = oscillator.clone();
    let cloned_freq = freq.clone();
    let cloned_active_osc = active_oscillators.clone();
    let cloned_active_lfo = active_lfo.clone();
    let mouse_down = Callback::from(move |label: (char, usize)| {
        let key_label = key_map_down.get(&label.0).unwrap_or(&0);
        log!("Holding key", label.0.to_string(), ", MIDI Note:", key_label.to_string());
        let cloned_key_map = &mut key_map_down.deref().clone();
        let mut buffer = cloned_poly.deref().clone();
        let device_temp = cloned_device.deref().clone();
        let config_temp = cloned_config.deref().clone();
        let mut oscillator_type = cloned_oscillator.deref().clone();
        let freq_filter = cloned_freq.deref().clone();
        let bandwidth_hz_filter = freq_filter*0.5;
        let mut active_indices = cloned_active_osc.deref().clone();
        let mut active_lfo_index = cloned_active_lfo.deref().clone();
        match label.0 {
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
                if label.1>0 {
                    oscillator_type.set_oscillator(label.1 - 1, Oscillator::Sine);
                    active_indices[label.1 - 1] = 0;
                    log!("Sine wave selected");
                }
            },
            '2' => {
                if label.1>0 {
                    oscillator_type.set_oscillator(label.1 - 1, Oscillator::BidirectionalSquare);
                    active_indices[label.1 - 1] = 1;
                    log!("Square wave selected");
                }
            },
            '3' => {
                if label.1>0 {
                    oscillator_type.set_oscillator(label.1 - 1, Oscillator::Saw);
                    active_indices[label.1 - 1] = 2;
                    log!("Sawtooth wave selected");
                }
            },
            '4' => {
                if label.1>0 {
                    oscillator_type.set_oscillator(label.1 - 1, Oscillator::Triangle);
                    active_indices[label.1 - 1] = 3;
                    log!("Triangle wave selected");
                }
            },
            '5' => {
                if label.1>0 {
                    oscillator_type.set_oscillator(label.1 - 1, Oscillator::WhiteNoise);
                    active_indices[label.1 - 1] = 4;
                    log!("White Noise wave selected");
                }
            },
            '0' => {
                oscillator_type.set_filter(Some(FilterType::HighPass), freq_filter, bandwidth_hz_filter);
                log!("High pass selected");
            },
            '9' => {
                oscillator_type.set_filter(Some(FilterType::BandPass), freq_filter, bandwidth_hz_filter);
                log!("Band pass selected");
            },
            '8' => {
                oscillator_type.set_filter(Some(FilterType::LowPass), freq_filter, bandwidth_hz_filter);
                log!("Low pass selected");
            },
            '7' => {
                oscillator_type.set_filter(None, freq_filter, bandwidth_hz_filter);
                log!("Filter off");
            },
            '+' => {
                let _ = oscillator_type.push(WaveTableOscillator::new(sample_rate, 44100, Oscillator::Sine, 0.7, 0.0));
                active_indices.push(0);
                log!("Add an oscillator");
            }
            '-' => {
                if oscillator_type.num_sources() > 1 {
                    let _ = oscillator_type.remove(label.1 - 1);
                    active_indices.remove(label.1 - 1);
                }
            },
            '[' => {
                oscillator_type.set_lfo_osc(Oscillator::Sine);
                active_lfo_index = 0;
            },
            ']' => {
                oscillator_type.set_lfo_osc(Oscillator::Square);
                active_lfo_index = 1;
            },
            '{' => {
                oscillator_type.set_lfo_osc(Oscillator::Saw);
                active_lfo_index = 2;
            },
            '}' => {
                oscillator_type.set_lfo_osc(Oscillator::Triangle);
                active_lfo_index = 3;
            },
            _ => {
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
        cloned_oscillator.set(oscillator_type);
        cloned_active_osc.set(active_indices);
        cloned_active_lfo.set(active_lfo_index);
    });

    let key_map_up = keycode_maps.clone();
    let cloned_poly = polyphony.clone();
    let stream_setter = stream.setter();
    let cloned_device = device.clone();
    let cloned_config = config.clone();
    let mouse_up = Callback::from(move |label: (char, usize)| {
        let key_label = key_map_up.get(&label.0).unwrap_or(&0);
        let mut buffer = cloned_poly.deref().clone();
        let _ = buffer.remove(key_label);
        let device_temp = cloned_device.deref().clone();
        let config_temp = cloned_config.deref().clone();
        let new_stream = State::new(&device_temp, &config_temp, buffer.clone());
        new_stream.play();
        stream_setter.set(new_stream);
        cloned_poly.set(buffer);
        log!("Lifted key", label.0.to_string(), ", MIDI Note:", key_map_up.get(&label.0).unwrap_or(&0).to_string());
    });

    let key_map_setter = keycode_maps.setter();
    let key_map_down = keycode_maps.clone();
    let cloned_poly = polyphony.clone();
    let cloned_device = device.clone();
    let cloned_config = config.clone();
    let stream_setter = stream.setter();
    let cloned_oscillator = oscillator.clone();
    let key_down = Callback::from(move |label: char| {
        let key_label = key_map_down.get(&label).unwrap_or(&0);
        let cloned_key_map = &mut key_map_down.deref().clone();
        let mut buffer = cloned_poly.deref().clone();
        let device_temp = cloned_device.deref().clone();
        let config_temp = cloned_config.deref().clone();
        // let mut oscillator_type = cloned_oscillator.deref().clone();
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
    let key_up = Callback::from(move |label: char| {
        let key_label = key_map_up.get(&label).unwrap_or(&0);
        let mut buffer = cloned_poly.deref().clone();
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
    let oscillator_selector_display: Vec<Html> = display_oscillators(
        mouse_down.clone(), 
        mouse_up.clone(), 
        key_up.clone(), 
        key_down.clone(), 
        oscillator.deref(),
        active_oscillators.deref().clone()
    );
    html! {
        <>
            <h1>{"Oscillator"}</h1>
            {oscillator_selector_display}
            <br />
            <AddButton on_mouse_down={mouse_down.clone()} on_mouse_up={mouse_up.clone()} />
            <h1>{"Filter"}</h1>
            <FilterSelector mouse_down={mouse_down.clone()} mouse_up={mouse_up.clone()} freq_change={freq_change} freq={*freq.deref() as f64}/>
            <h1>{"LFO"}</h1>
            <LFOSelector mouse_down={mouse_down.clone()} mouse_up={mouse_up.clone()} freq_change={freq_lfo_change} active_index={active_lfo.deref()} freq={*lfo_freq.deref() as f64}/>
            <h1>{"Envelope"}</h1>
            <EnvelopeSettings attack_change={attack_change} decay_change={decay_change} sustain_change={sustain_change} attack={*attack_ms.deref() as f64} decay={*decay_ms.deref() as f64} sustain={*sustain_percentage.deref() as f64}/>
            <MIDIKeyboard mouse_down={mouse_down.clone()} mouse_up={&mouse_up} key_down={&key_down} key_up={&key_up}/>
            <p>{"Current MIDI Range: "}{&key_map_clone.deref()[&'A']}{" - "}{&key_map_clone.deref()[&'K']}</p>
        </>
    }
}

pub fn display_oscillators(mouse_down: Callback<(char, usize)>, mouse_up: Callback<(char, usize)>, key_down: Callback<char>, key_up: Callback<char> ,oscillator: &Synth, active_indices: Vec<usize>) -> Vec<Html>{
    let mut display = Vec::new();
    for idx in 0..oscillator.num_sources() {
        display.push(html! {
            <OscillatorSelector 
                mouse_down={mouse_down.clone()} 
                mouse_up={mouse_up.clone()} 
                number={idx as usize+1} 
                active_index={active_indices[idx]}
            />
        })
    }
    display
}
