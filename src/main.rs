use std::time::Duration;
use std::collections::HashMap;
use midi_typing_keyboard::{MultiOscillator, WaveTableOscillator, Oscillator};
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};
use device_query::{DeviceQuery, DeviceState, Keycode};


mod utils;

use utils::{midi_to_hz, increase_octave, decrease_octave};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let poly = 16;

    let osc1 = WaveTableOscillator::new(44100, 44100, Oscillator::Sine, 0.8);
    let osc2 = WaveTableOscillator::new(44100, 44100, Oscillator::Square, 0.2);
    let osc3 = WaveTableOscillator::new(44100, 44100, Oscillator::Saw, 0.5);
    let osc4 = WaveTableOscillator::new(44100, 44100, Oscillator::WhiteNoise, 0.1);
    let sound = osc1 + osc2 + (osc3 + osc4);

    let mut sinks: Vec<Sink> = Vec::new();
    for i in 0..poly {
        sinks.push(Sink::try_new(&stream_handle).unwrap());
        sinks[i].pause();
    }
    let device_state = DeviceState::new();

    let mut prev_keys_pressed: usize = 0;
    let mut midi_map: [u8; 17] = [60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76];
    let mut keycode_maps = HashMap::new();
    let keycodes: [Keycode; 17] = [
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
        Keycode::K,
        Keycode::O,
        Keycode::L,
        Keycode::P,
        Keycode::Semicolon
    ];
    for (key, midi) in keycodes.iter().zip(midi_map.iter_mut()) {
        keycode_maps.insert(key, midi);
    }
    let mut flag_octave_change = false;

    loop{
        let keys: Vec<Keycode> = device_state.get_keys();
        // println!("{:?}", keys);
        let num_keys_pressed = keys.len();
        if num_keys_pressed != prev_keys_pressed {
            for sink in sinks.iter() {
                sink.clear();
            }
            prev_keys_pressed = num_keys_pressed;
        }
        if num_keys_pressed > 0{
            for (index, key) in keys.iter().enumerate() {
                if keycodes.contains(key) {
                    let freq = midi_to_hz(*keycode_maps[key]).unwrap_or(1.0);
                    let mut source = sound.clone();
                    for i in 0..source.num_sources() {
                        let _ = source.set_frequency(freq, i);
                    }
                    sinks[index].append(source);
                }
                if !flag_octave_change {
                    match key {
                        &Keycode::Z =>  {
                            for sink in sinks.iter() {
                                sink.stop();
                                sink.pause();
                            }
                            decrease_octave(&mut keycode_maps);
                            flag_octave_change = true;
                        },
                        &Keycode::X =>  {
                            for sink in sinks.iter() {
                                sink.stop();
                                sink.pause();
                            }
                            increase_octave(&mut keycode_maps);
                            flag_octave_change = true;
                        },
                        _ => ()
                    }
                }
            }
            for sink in sinks.iter() { sink.play() }
        } else {
            for sink in sinks.iter() {
                sink.stop();
                sink.clear();
            }
            flag_octave_change = false;
        }
    }
}
