use std::{collections::HashMap, ops::Deref};
use oscillators::{MultiOscillator, WaveTableOscillator, Oscillator};
use rodio::{OutputStream, Sink};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::rc::Rc;
use ring_buffer::PolyphonyRingBuffer;

mod utils;
mod oscillators;
mod ring_buffer;

use utils::{midi_to_hz, increase_octave, decrease_octave};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let poly = 16;

    let osc1 = WaveTableOscillator::new(44100, 44100, Oscillator::Sine, 0.8, 0.0);
    let osc2 = WaveTableOscillator::new(44100, 44100, Oscillator::Square, 0.2, 0.0);
    let osc3 = WaveTableOscillator::new(44100, 44100, Oscillator::Saw, 0.5, 0.0);
    let osc4 = WaveTableOscillator::new(44100, 44100, Oscillator::WhiteNoise, 0.2, 0.0);
    let sound: Rc<MultiOscillator> = Rc::new(osc1 + osc2 + (osc3 + osc4));
    let mut polyphony: PolyphonyRingBuffer = PolyphonyRingBuffer::new(poly, 44100);

    let sink: Sink = Sink::try_new(&stream_handle).unwrap();
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

    println!("{:?}", keycode_maps);

    loop{
        let keys: Vec<Keycode> = device_state.get_keys();
        // println!("{:?}", keys);
        let num_keys_pressed = keys.len();
        println!("{:?}", keys);
        if num_keys_pressed > prev_keys_pressed {
            for i in 0..(num_keys_pressed-prev_keys_pressed){
                let current_key = &keys[prev_keys_pressed + i];
                if keycodes.contains(current_key) {
                    sink.stop();
                    sink.clear();
                    let frequency = midi_to_hz(*keycode_maps[current_key]).unwrap_or(1.0);
                    let mut source = sound.deref().clone();
                    let _ = source.global_set_frequency(frequency);
                    polyphony.push(source);
                    sink.append(polyphony.clone());
                    sink.play();
                }
                match current_key {
                    &Keycode::Z => decrease_octave(&mut keycode_maps),
                    &Keycode::X => increase_octave(&mut keycode_maps),
                    _ => ()
                }
            }
        }
        else if num_keys_pressed < prev_keys_pressed {
            sink.stop();
            sink.clear();
            polyphony.pop();
            sink.append(polyphony.clone());
            sink.play();
        }
        prev_keys_pressed = num_keys_pressed;
    }
}
