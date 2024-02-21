use std::time::Duration;
use std::collections::HashMap;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source, Repeat};
use device_query::{DeviceQuery, DeviceState, Keycode};

mod utils;

use utils::midi_to_hz;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let device_state = DeviceState::new();
    sink.pause();

    let mut prev_keys_pressed: usize = 0;
    let mut midi_map: [u8; 17] = [60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76];
    let mut flag_octave_change = false;

    loop{
        let keys: Vec<Keycode> = device_state.get_keys();
        let num_keys_pressed = keys.len();
        if num_keys_pressed != prev_keys_pressed {
            sink.clear();
            sink.pause();
            prev_keys_pressed = num_keys_pressed;
        }
        if num_keys_pressed > 0 {
            for key in keys.iter() {
                match key {
                    &Keycode::A =>  {
                        let source = SineWave::new(midi_to_hz(midi_map[0]).unwrap_or_default()).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::W =>  {
                        let source = SineWave::new(midi_to_hz(midi_map[1]).unwrap_or_default()).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::S =>  {
                        let source = SineWave::new(midi_to_hz(midi_map[2]).unwrap_or_default()).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::E =>  {
                        let source = SineWave::new(midi_to_hz(midi_map[3]).unwrap_or_default()).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::D =>  {
                        let source = SineWave::new(midi_to_hz(midi_map[4]).unwrap_or_default()).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::F =>  {
                        let source = SineWave::new(midi_to_hz(midi_map[5]).unwrap_or_default()).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::T =>  {
                        let source = SineWave::new(midi_to_hz(midi_map[6]).unwrap_or_default()).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::G =>  {
                        let source = SineWave::new(midi_to_hz(midi_map[7]).unwrap_or_default()).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::Y =>  {
                        let source = SineWave::new(midi_to_hz(midi_map[8]).unwrap_or_default()).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::H =>  {
                        let source = SineWave::new(midi_to_hz(midi_map[9]).unwrap_or_default()).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::U =>  {
                        let source = SineWave::new(midi_to_hz(midi_map[10]).unwrap_or_default()).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::J =>  {
                        let source = SineWave::new(midi_to_hz(midi_map[11]).unwrap_or_default()).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::K =>  {
                        let source = SineWave::new(midi_to_hz(midi_map[12]).unwrap_or_default()).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::O =>  {
                        let source = SineWave::new(midi_to_hz(midi_map[13]).unwrap_or_default()).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::L =>  {
                        let source = SineWave::new(midi_to_hz(midi_map[14]).unwrap_or_default()).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::P =>  {
                        let source = SineWave::new(midi_to_hz(midi_map[15]).unwrap_or_default()).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::Semicolon =>  {
                        let source = SineWave::new(midi_to_hz(midi_map[16]).unwrap_or_default()).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::Z =>  {
                        if !flag_octave_change {
                            sink.stop();
                            sink.pause();
                            midi_map = midi_map.map(|value| value-12);
                            println!("{:?}", midi_map);
                            flag_octave_change = true;
                        }
                    },
                    &Keycode::X =>  {
                        if !flag_octave_change {
                            sink.stop();
                            sink.pause();
                            midi_map = midi_map.map(|value| value+12);
                            println!("{:?}", midi_map);
                            flag_octave_change = true;
                        }
                    },
                    _ => ()
                }
            }
            sink.play();
        } else {
            sink.stop();
            sink.pause();
            flag_octave_change = false;
        }
    }
}
