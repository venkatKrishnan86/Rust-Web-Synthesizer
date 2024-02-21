use std::time::Duration;
use std::collections::HashMap;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};
use device_query::{DeviceQuery, DeviceState, Keycode};

mod utils;

use utils::midi_to_hz;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let poly = 16;
    let mut sinks: Vec<Sink> = Vec::new();
    for i in 0..poly {
        sinks.push(Sink::try_new(&stream_handle).unwrap());
        sinks[i].pause();
    }
    let device_state = DeviceState::new();

    let mut prev_keys_pressed: usize = 0;
    let mut midi_map: [u8; 17] = [60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76];
    let mut source_maps = HashMap::new();
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
    for (key, midi) in keycodes.iter().zip(midi_map.iter()) {
        source_maps.insert(key, *midi);
    }
    let mut flag_octave_change = false;
    let mut flag_key_pressed = false;

    loop{
        let keys: Vec<Keycode> = device_state.get_keys();
        // println!("{:?}", keys);
        let num_keys_pressed = keys.len();
        if num_keys_pressed != prev_keys_pressed {
            for sink in sinks.iter() {
                sink.clear();
                sink.pause();
            }
            prev_keys_pressed = num_keys_pressed;
        }
        if num_keys_pressed > 0{
            // if !flag_key_pressed{
            for (index, key) in keys.iter().enumerate() {
                if keycodes.contains(key) {
                    let source = SineWave::new(midi_to_hz(source_maps[key]).unwrap_or_default()).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                    sinks[index].append(source);
                }
                match key {
                    &Keycode::Z =>  {
                        if !flag_octave_change {
                            for sink in sinks.iter() {
                                sink.stop();
                                sink.pause();
                            }
                            midi_map = midi_map.map(|value| value-12);
                            println!("{:?}", midi_map);
                            for (key, midi) in keycodes.iter().zip(midi_map.iter()) {
                                source_maps.insert(key, *midi);
                            }
                            flag_octave_change = true;
                        }
                    },
                    &Keycode::X =>  {
                        if !flag_octave_change {
                            for sink in sinks.iter() {
                                sink.stop();
                                sink.pause();
                            }
                            midi_map = midi_map.map(|value| value+12);
                            for (key, midi) in keycodes.iter().zip(midi_map.iter()) {
                                source_maps.insert(key, *midi);
                            }
                            println!("{:?}", midi_map);
                            flag_octave_change = true;
                        }
                    },
                    _ => ()
                }
            }
            for sink in sinks.iter() { sink.play() }
            flag_key_pressed = true;
            // }
        } else {
            for sink in sinks.iter() {
                sink.stop();
                sink.pause();
            }
            flag_octave_change = false;
            flag_key_pressed = false;
        }
    }
}
