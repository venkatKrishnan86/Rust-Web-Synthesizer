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
    let mut sources = &midi_map.map(|midi| SineWave::new(midi_to_hz(midi).unwrap_or_default()).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite());
    let mut flag_octave_change = false;
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
    let mut flag_keys_pressed = [false; 17];
    let mut flag_some_key_pressed = false;

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
                for (index, key_code) in keycodes.iter().enumerate() {
                    if key_code == key {
                        flag_keys_pressed[index] = true;

                    }
                }
                
                match key {
                    // &Keycode::A =>  {
                    //     sink.append(sources[0].clone());
                    // },
                    // &Keycode::W =>  {
                    //     sink.append(sources[1].clone());
                    // },
                    // &Keycode::S =>  {
                    //     sink.append(sources[2].clone());
                    // },
                    // &Keycode::E =>  {
                    //     sink.append(sources[3].clone());
                    // },
                    // &Keycode::D =>  {
                    //     sink.append(sources[4].clone());
                    // },
                    // &Keycode::F =>  {
                    //     sink.append(sources[5].clone());
                    // },
                    // &Keycode::T =>  {
                    //     sink.append(sources[6].clone());
                    // },
                    // &Keycode::G =>  {
                    //     sink.append(sources[7].clone());
                    // },
                    // &Keycode::Y =>  {
                    //     sink.append(sources[8].clone());
                    // },
                    // &Keycode::H =>  {
                    //     sink.append(sources[9].clone());
                    // },
                    // &Keycode::U =>  {
                    //     sink.append(sources[10].clone());
                    // },
                    // &Keycode::J =>  {
                    //     sink.append(sources[11].clone());
                    // },
                    // &Keycode::K =>  {
                    //     sink.append(sources[12].clone());
                    // },
                    // &Keycode::O =>  {
                    //     sink.append(sources[13].clone());
                    // },
                    // &Keycode::L =>  {
                    //     sink.append(sources[14].clone());
                    // },
                    // &Keycode::P =>  {
                    //     sink.append(sources[15].clone());
                    // },
                    // &Keycode::Semicolon =>  {
                    //     sink.append(sources[16].clone());
                    // },
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
            let active_sources = sources.clone().into_iter().zip(flag_keys_pressed.iter()).filter(|(a,b)| **b);
            for source in active_sources.into_iter() {
                println!("{}",num_keys_pressed);
                sink.append(source.0);
            }
            sink.play();
        } else {
            sink.stop();
            sink.pause();
            flag_octave_change = false;
            flag_keys_pressed = [false; 17];
        }
    }
}
