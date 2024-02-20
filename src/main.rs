// use std::fs::File;
// use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};
use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};

fn main() {
    // let args: Vec<String> = std::env::args().collect();
    // if args.len() != 2 {
    //     eprintln!("Usage: {} <input wave filename>", args[0]);
    //     return
    // }

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let device_state = DeviceState::new();
    sink.pause();

    let note_a = 440.0;

    loop{
        let keys: Vec<Keycode> = device_state.get_keys();
        if keys.len() > 0 {
            for key in keys.iter() {
                match key {
                    &Keycode::A =>  {
                        let source = SineWave::new(note_a * f32::powf(2.0, -9.0/12.0)).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::W =>  {
                        let source = SineWave::new(note_a * f32::powf(2.0, -8.0/12.0)).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::S =>  {
                        let source = SineWave::new(note_a * f32::powf(2.0, -7.0/12.0)).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::E =>  {
                        let source = SineWave::new(note_a * f32::powf(2.0, -6.0/12.0)).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::D =>  {
                        let source = SineWave::new(note_a * f32::powf(2.0, -5.0/12.0)).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::F =>  {
                        let source = SineWave::new(note_a * f32::powf(2.0, -4.0/12.0)).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::T =>  {
                        let source = SineWave::new(note_a * f32::powf(2.0, -3.0/12.0)).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::G =>  {
                        let source = SineWave::new(note_a * f32::powf(2.0, -2.0/12.0)).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::Y =>  {
                        let source = SineWave::new(note_a * f32::powf(2.0, -1.0/12.0)).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::H =>  {
                        let source = SineWave::new(note_a).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::U =>  {
                        let source = SineWave::new(note_a * f32::powf(2.0, 1.0/12.0)).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::J =>  {
                        let source = SineWave::new(note_a * f32::powf(2.0, 2.0/12.0)).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::K =>  {
                        let source = SineWave::new(note_a * f32::powf(2.0, 3.0/12.0)).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::O =>  {
                        let source = SineWave::new(note_a * f32::powf(2.0, 4.0/12.0)).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::L =>  {
                        let source = SineWave::new(note_a * f32::powf(2.0, 5.0/12.0)).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    &Keycode::P =>  {
                        let source = SineWave::new(note_a * f32::powf(2.0, 6.0/12.0)).take_duration(Duration::from_secs(1)).amplify(0.20).repeat_infinite();
                        sink.append(source);
                    },
                    _ => ()
                }
            }
            sink.play();
        } else {
            sink.stop();
            sink.pause();
        }
    }
}
