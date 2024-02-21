use std::time::Duration;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};
use device_query::{DeviceQuery, DeviceState, Keycode};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let device_state = DeviceState::new();
    sink.pause();

    let note_a = 440.0;
    let mut prev_keys_pressed: usize = 0;

    loop{
        let keys: Vec<Keycode> = device_state.get_keys();
        let num_keys_pressed = keys.len();
        if num_keys_pressed != prev_keys_pressed {
            sink.stop();
            sink.pause();
            prev_keys_pressed = num_keys_pressed;
        }
        if num_keys_pressed > 0 {
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
