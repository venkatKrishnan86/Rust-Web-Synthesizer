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

use oscillators::MultiOscillator;
use ring_buffer::RingBuffer;
use rodio::Source;

/// # Polyphony handler struct
/// Aim of this struct is to avoid the usage of multiple sinks on playing multiple notes, instead handle multiple notes 
/// through a new source, which would be a RingBuffer filled with MultiOScillators
pub struct PolyphonyHandler {
    _source: RingBuffer<MultiOscillator>,
    sample_rate: u32
}

impl PolyphonyHandler {
    pub fn new() {
        todo!("Implement");
    }

    pub fn push(&mut self, _multi_osc: MultiOscillator) {
        todo!("Implement");
    }

    pub fn get_sample(&self) {
        todo!("Implement");
    }
}

impl Iterator for PolyphonyHandler {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("Implement");
    }
}

impl Source for PolyphonyHandler {
    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}


mod utils;
mod oscillators;
mod ring_buffer;

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