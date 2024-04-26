//! Utility functions and types used in the backend of the synthesizer.
use std::{collections::HashMap, ops::{Deref, DerefMut}};
use cpal::{traits::{DeviceTrait, StreamTrait}, Data, FromSample, OutputCallbackInfo, SampleFormat, SizedSample, Stream, StreamConfig};
use crate::ring_buffer::IterablePolyphonyHashMap;
use web_sys::console;
use gloo::console::log;

use std::sync::{Arc, Mutex};
/// Converts MIDI note number to frequency in Hz.
///
/// # Arguments
///
/// * `midi` - The MIDI note number, ranging between 0 and 127.
///
/// # Returns
///
/// The corresponding frequency in Hz.
///
/// # Errors
///
/// Returns an error if the MIDI note is out of range.
#[allow(dead_code)]
pub fn midi_to_hz(midi: u8) -> Result<f32, String> {
    if midi>=128 {
        return Err("MIDI must range between 0-128".to_owned());
    }
    Ok(f32::powf(2.0, ((midi as f32 - 69 as f32))/12.0) * 440.0)
}

pub fn hz_to_midi(frequency: f32) -> Result<u8, String>  {
    if frequency<0.0 {
        return Err("MIDI must range between 0-128".to_owned());
    }
    Ok((69.0 + 12.0 * (frequency / 440.0).log2()).round() as u8)
}

/// Converts MIDI note number and cents deviation to frequency in Hz.
///
/// # Arguments
///
/// * `midi` - The MIDI note number, ranging between 0 and 127.
/// * `cents_dev` - The deviation in cents, ranging between -50 and 50.
///
/// # Returns
///
/// The corresponding frequency in Hz.
///
/// # Errors
///
/// Returns an error if the MIDI note or cents deviation is out of range.
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

/// Checks if two f32 values are approximately equal within a tolerance.
///
/// # Arguments
///
/// * `a` - The first f32 value.
/// * `b` - The second f32 value.
///
/// # Returns
///
/// `true` if the values are approximately equal, `false` otherwise.
#[allow(dead_code)]
pub fn is_close_f32(a: f32, b: f32) -> bool {
    (a - b).abs() <= 0.01
}

/// Increases the octave of each MIDI note in the provided map by 12 semitones.
///
/// # Arguments
///
/// * `midi_map` - A HashMap containing characters as keys and MIDI note numbers as values.
pub fn increase_octave(midi_map: &mut HashMap<char, u8>) {
    for (_, midi) in midi_map {
        *midi+=12;
    }
}

/// Decreases the octave of each MIDI note in the provided map by 12 semitones.
///
/// # Arguments
///
/// * `midi_map` - A HashMap containing characters as keys and MIDI note numbers as values.
pub fn decrease_octave(midi_map: &mut HashMap<char, u8>) {
    for (_, midi) in midi_map {
        *midi-=12;
    }
}

/// Creates a single stream which runs once
pub fn create_stream(device: &cpal::Device, config: &cpal::StreamConfig, polyphony: Arc<Mutex<IterablePolyphonyHashMap>>) -> Stream {
    let channels: usize = config.channels as usize;
    let err_fn = |err| console::error_1(&format!("A stream error ocurred: {}", err).into());
    // let err_fn = |err| eprintln!("{err}");
    let mut next_value = {
        let poly = Arc::clone(&polyphony);
        move || {
            let sample = poly.lock().unwrap().get_sample();
            sample
        }
    };

    let buffer_size = 1024;
    // let sample_format = cpal::SampleFormat::F32;

    let stream_config = cpal::StreamConfig {
        channels: config.channels,
        sample_rate: config.sample_rate,
        buffer_size: cpal::BufferSize::Fixed(buffer_size),
    };

    // log!(channels);

    let stream = device
        .build_output_stream_raw (
            &stream_config,
            cpal::SampleFormat::F32,
            move |data: &mut Data, _info: &OutputCallbackInfo| {
                write_data(data, channels, &mut next_value);
            }, 
            err_fn,
            None,
        )
        .unwrap();
    stream.play().unwrap();
    stream
}

/// Writes audio data to the output buffer.
fn write_data(output: &mut Data, channels: usize, next_sample: &mut dyn FnMut() -> f32){
    for frame in output.as_slice_mut() {
        // log!(frame.len());
        for sample in frame.iter_mut() {
            *sample = next_sample();
        }
    }
}