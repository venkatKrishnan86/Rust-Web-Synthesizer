//! # Polyphonic MIDI map
//! 
//! I aim to create a polyphonic sine MIDI map.
//! ## Key takeaways
//! - `Source`: Any struct containing the audio array information, and must implement the `Source` trait
//! - `Sink`: One track, which feeds to the output. The appending happens in the time domain. Each source appended plays one after the other
//!     - One track
//!     - Feeds directly to the output stream_handle
//!     - The appending happens in the time domain
//!     - Each source appended plays one after the other
//!     - For multiple sounds to play together, one must use multiple sinks

mod utils;

use std::f32::consts::PI;
use rand_distr::{Distribution, Uniform};
use rodio::Source;

pub enum Oscillator {
    Sine,
    Square,
    BidirectionalSquare,
    Saw,
    Triangle,
    WhiteNoise
}

/// Convert WavetableOscillator parameters in to a vector and use aligned_allocator to play each sample from the wavetable
pub struct WaveTableOscillator {
    sample_rate: u32,
    oscillator: Oscillator,
    wave_table_size: usize,
    wave_table: Vec<f32>,
    gain: f32,
    index: f32,
    index_increment: f32
}

impl WaveTableOscillator {
    pub fn new(sample_rate: u32, wave_table_size: usize, oscillator: Oscillator, gain: f32) -> Self {
        assert!(gain>=0.0 && gain<=1.0, "Gain must be between 0 and 1");
        let mut wave_table: Vec<f32> = Vec::new();
        match oscillator {
            Oscillator::Sine => {
                for i in 0..wave_table_size {
                    wave_table.push((2.0 * PI * (i as f32)/(wave_table_size as f32)).sin() * gain);
                }
            },
            Oscillator::Square => {
                for i in 0..wave_table_size {
                    if i < wave_table_size/2 {
                        wave_table.push(0.99 * gain);
                    } else {
                        wave_table.push(0.0);
                    }
                }
            },
            Oscillator::BidirectionalSquare => {
                for i in 0..wave_table_size {
                    if i < wave_table_size/2 {
                        wave_table.push(0.99 * gain);
                    } else {
                        wave_table.push(-0.99 * gain);
                    }
                }
            },
            Oscillator::Saw => {
                for i in 1..=wave_table_size {
                    wave_table.push((((wave_table_size as f32 - i as f32)/(wave_table_size as f32) * 2.0) - 1.0) * gain);
                }
            },
            Oscillator::Triangle => {
                for i in 0..wave_table_size/2 {
                    wave_table.push((((i as f32/wave_table_size as f32)*4.0) - 1.0) * gain)
                }
                for i in wave_table_size/2..wave_table_size {
                    wave_table.push(((-(i as f32/wave_table_size as f32)*4.0) + 3.0) * gain)
                }
            },
            Oscillator::WhiteNoise => ()
        }
        Self {
            sample_rate,
            oscillator,
            gain,
            wave_table_size,
            wave_table,
            index: 0.0,
            index_increment: 0.0
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) -> Result<(), String> {
        if frequency <= 0.0 {
            return Err("Frequency must be a positive floating point value!".to_owned());
        }
        self.index_increment = frequency * self.wave_table_size as f32 / self.sample_rate as f32;
        Ok(())
    }

    pub fn set_gain(&mut self, gain: f32) -> Result<(), String> {
        if gain < 0.0 || gain > 1.0 {
            return Err("Gain must be between 0.0 and 1.0!".to_owned());
        }
        self.gain = gain;
        Ok(())
    }

    pub fn get_sample(&mut self) -> f32 {
        match self.oscillator {
            Oscillator::WhiteNoise => {
                let mut rng = rand::thread_rng();
                let unif = Uniform::new(-1.0, 1.0);
                unif.sample(&mut rng) * self.gain
            },
            _ => {
                let index_1 = self.index.trunc() as usize;
                let frac = self.index - index_1 as f32;
                self.index = (self.index + self.index_increment) % self.wave_table_size as f32;
                WaveTableOscillator::lerp(self.wave_table[index_1], self.wave_table[(index_1 + 1)%self.wave_table_size], frac)
            }
        }
    }

    fn lerp(sample1: f32, sample2: f32, frac: f32) -> f32{
        (1.0-frac)*sample1 + frac*sample2
    }
}

impl Iterator for WaveTableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.get_sample())
    }
}

impl Source for WaveTableOscillator {
    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn current_frame_len(&self) -> Option<usize> {
        None // Means infinite playback
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None // Means infinite playback
    }
}

#[cfg(test)]
mod tests {
    use rand::random;
    use utils::{midi_cents_to_hz, midi_to_hz, is_close_f32};
    use super::*;
    mod midi_to_hz_tests {
        use super::*;

        #[test]
        fn test_1() {
            assert!(is_close_f32(midi_to_hz(69).unwrap(), 440.0));
        }

        #[test]
        fn test_2_octave_above() {
            let value = random::<f32>();
            let random_midi: u8 = ((value * 115.0)).trunc() as u8;
            let random_freq = midi_to_hz(random_midi).unwrap_or_default();
            let octave_above = random_midi + 12;
            assert!(is_close_f32(midi_to_hz(octave_above).unwrap_or_default(), random_freq*2.0), "Octave above midi value {octave_above}, does not match random_freq {random_freq} * 2");
        }

        #[test]
        fn test_3_octave_below() {
            let value = random::<f32>();
            let random_midi: u8 = ((value * 115.0) + 12.0).trunc() as u8;
            let random_freq = midi_to_hz(random_midi).unwrap_or_default();
            let octave_below = random_midi - 12;
            assert!(is_close_f32(midi_to_hz(octave_below).unwrap_or_default(), random_freq/2.0), "Octave below midi value {octave_below}, does not match random_freq {random_freq} / 2");
        }

        #[test]
        #[should_panic]
        fn test_4_midi_upper_bound(){
            let value = midi_to_hz(129);
            match value {
                Ok(_) => (),
                Err(err) => panic!("{}", err)
            }
        }
    }

    mod midi_cents_to_hz_tests {
        use super::*;

        #[test]
        fn test_1() {
            println!("{}", midi_cents_to_hz(69, 0).unwrap_or_default());
            assert!(is_close_f32(midi_cents_to_hz(69, 0).unwrap_or_default(), 440.0));
        }

        #[test]
        #[should_panic]
        fn test_2_lower_bound_on_cents() {
            let value = midi_cents_to_hz(20, -51);
            match value {
                Ok(_) => (),
                Err(err) => panic!("{}", err)
            }
        }

        #[test]
        #[should_panic]
        fn test_3_upper_bound_on_cents() {
            let value = midi_cents_to_hz(20, 51);
            match value {
                Ok(_) => (),
                Err(err) => panic!("{}", err)
            }
        }

        #[test]
        #[should_panic]
        fn test_4_midi_upper_bound() {
            let value = midi_cents_to_hz(129, 0);
            match value {
                Ok(_) => (),
                Err(err) => panic!("{}", err)
            }
        }


    }
}