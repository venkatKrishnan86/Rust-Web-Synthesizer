# The `main.rs` function

```rust
use std::{collections::HashMap, ops::Deref};
use oscillators::{MultiOscillator, WaveTableOscillator, Oscillator};
use rodio::{OutputStream, Sink};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::rc::Rc;
use ring_buffer::IterablePolyphonyHashMap;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SampleRate, SizedSample, Stream, StreamConfig};

mod utils;
mod oscillators;
mod ring_buffer;

use utils::{midi_to_hz, increase_octave, decrease_octave, State};

fn main() {
    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No default output device found");
    // let supported_configs = device.supported_output_configs().unwrap();
    let mut supported_configs_range = device.supported_output_configs()
    .expect("error while querying configs");
    let supported_config = supported_configs_range.next()
        .expect("no supported config?!")
        .with_max_sample_rate();
    let sample_rate = supported_config.sample_rate().0;
    println!("Sample Rate: {}", sample_rate);
    // let sample_format = supported_config.sample_format();
    let config = supported_config.into();
    // let config = match supported_configs
    //     .filter(|c| c.channels() == 2)
    //     .max_by(|a, b| a.cmp_default_heuristics(b)) {
    //         Some(config) => config.with_sample_rate(SampleRate(SAMPLE_RATE)).into(),
    //         _ => panic!("No supported configuration found for output device")
    //     };

    let osc1 = WaveTableOscillator::new(sample_rate, 44100, Oscillator::Sine, 0.8, 0.0);
    let osc2 = WaveTableOscillator::new(sample_rate, 44100, Oscillator::Square, 0.2, 0.0);
    let osc3 = WaveTableOscillator::new(sample_rate, 44100, Oscillator::Saw, 0.5, 0.0);
    let osc4 = WaveTableOscillator::new(sample_rate, 44100, Oscillator::WhiteNoise, 0.2, 0.0);
    let sound: Rc<MultiOscillator> = Rc::new(osc1 + osc2 + (osc3 + osc4));
    let mut itermap = IterablePolyphonyHashMap::new(sample_rate);

    // let sink: Sink = Sink::try_new(&stream_handle).unwrap();
    let mut stream = State::new(&device, &config, itermap.clone());
    let device_state = DeviceState::new();

    let mut prev_keys_pressed: usize = 0;
    let mut midi_map: [u8; 13] = [60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72];
    let mut keycode_maps = HashMap::new();
    let keycodes: [Keycode; 13] = [
        Keycode::A,
        Keycode::W,
        Keycode::S,
        Keycode::E,
        Keycode::D,
        Keycode::F,
        Keycode::T,
        Keycode::G,
        Keycode::Y,
        Keycode::H,
        Keycode::U,
        Keycode::J,
        Keycode::K
    ];
    for (key, midi) in keycodes.iter().zip(midi_map.iter_mut()) {
        keycode_maps.insert(key, midi);
    }

    loop{
        let keys: Vec<Keycode> = device_state.get_keys();
        let num_keys_pressed = keys.len();
        if num_keys_pressed != prev_keys_pressed {
            itermap.clear();
            // sink.clear();
            stream.pause();
            for key in keys.iter() {
                match key {
                    // &Keycode::Z => decrease_octave(&mut keycode_maps),
                    // &Keycode::X => increase_octave(&mut keycode_maps),
                    _ => {
                        if keycodes.contains(key) {
                            let midi = *keycode_maps[key];
                            let frequency = midi_to_hz(midi).unwrap_or(1.0);
                            let mut source = sound.deref().clone();
                            let _ = source.global_set_frequency(frequency);
                            itermap.insert(midi, source);
                        }
                    }
                }
            }
            // sink.append(itermap.clone());
            // sink.play();
            stream.update_polyphony(itermap.clone());
            stream.play();
        }
        prev_keys_pressed = num_keys_pressed;
    }
}

```