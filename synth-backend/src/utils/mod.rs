use std::collections::HashMap;
use cpal::{traits::StreamTrait, Stream};
use crate::ring_buffer::IterablePolyphonyHashMap;
use cpal::traits::DeviceTrait;

#[allow(dead_code)]
pub fn midi_to_hz(midi: u8) -> Result<f32, String> {
    if midi>=128 {
        return Err("MIDI must range between 0-128".to_owned());
    }
    Ok(f32::powf(2.0, ((midi as f32 - 69 as f32))/12.0) * 440.0)
}

#[allow(dead_code)]
pub fn midi_cents_to_hz(midi: u8, cents_dev: i8) -> Result<f32, String> {
    if cents_dev < -50 && cents_dev > 50 {
        return Err("Cents deviation must be between -50 and 50".to_owned());
    }
    if midi>=128 {
        return Err("MIDI must range between 0-128".to_owned());
    }
    Ok(f32::powf(2.0, ((midi-69) as f32 + cents_dev as f32/100.0)/12.0) * 440.0)
}

#[allow(dead_code)]
pub fn is_close_f32(a: f32, b: f32) -> bool {
    (a - b).abs() <= 0.01
}

pub fn increase_octave(midi_map: &mut HashMap<char, u8>) {
    for (_, midi) in midi_map {
        *midi+=12;
    }
}

pub fn decrease_octave(midi_map: &mut HashMap<char, u8>) {
    for (_, midi) in midi_map {
        *midi-=12;
    }
}

pub struct State<'a> {
    stream: Stream,
    device: &'a cpal::Device,
    config: &'a cpal::StreamConfig,
    polyphony: IterablePolyphonyHashMap
}

impl<'a> State<'a> {
    pub fn new(device: &'a cpal::Device, config: &'a cpal::StreamConfig, polyphony: IterablePolyphonyHashMap) -> Self {
        let stream = State::create_stream(device, config, polyphony.clone());
        return State { stream , device, config, polyphony};
    }

    fn create_stream(device: &cpal::Device, config: &cpal::StreamConfig, polyphony: IterablePolyphonyHashMap) -> Stream {
        let channels: usize = config.channels as usize;
        // let err_fn = |err| console::error_1(&format!("A stream error ocurred: {}", err).into());
        let err_fn = |err| eprintln!("{err}");
        let mut poly = polyphony;
        let mut next_value = {
            move || {
                poly.get_sample()
            }
        };
        device
            .build_output_stream(
                config, 
                move |data: &mut [f32], _| State::write_data(data, channels, &mut next_value),
                err_fn,
                None,
            )
            .unwrap()
    }

    pub fn update_polyphony(&mut self, polyphony: IterablePolyphonyHashMap) {
        self.polyphony = polyphony;
        self.stream = State::create_stream(self.device, self.config, self.polyphony.clone());
    }

    pub fn pause(&self) {
        self.stream.pause().unwrap()
    }

    pub fn play(&self){
        self.stream.play().unwrap()
    }

    fn write_data(output: &mut [f32], channels: usize, next_sample: &mut dyn FnMut() -> f32){
        for frame in output.chunks_mut(channels) {
            let value = next_sample();
            println!("{value}");
            for sample in frame.iter_mut() {
                *sample = value;
            }
        }
    }
    }

// fn hz_to_midi(hz: ) -> f32 {
//     f32::powf(2.0, (midi-69) as f32/12.0) * 440.0
// }