//! Oscillators
//!
//! This module provides implementations of various types of oscillators, including sine, square, bidirectional square,
//! sawtooth, triangle, and white noise generators.
//!
//! # Examples
//!
//! ```
//! use synth_backend::oscillators::{Oscillator, WaveTableOscillator, MultiOscillator};
//!
//! // Create a new sine wave oscillator with a sample rate of 44100 Hz, wave table size of 1024, and frequency of 440 Hz
//! let mut sine_oscillator = WaveTableOscillator::new(44100, 1024, Oscillator::Sine, 0.5, 440.0);
//!
//! // Create a new square wave oscillator with the same parameters
//! let mut square_oscillator = WaveTableOscillator::new(44100, 1024, Oscillator::Square, 0.5, 440.0);
//!
//! // Combine both oscillators into a multi-oscillator
//! let mut multi_oscillator = sine_oscillator + square_oscillator;
//!
//! // Generate samples from the multi-oscillator
//! let sample = multi_oscillator.get_sample();
//! ```
//!
//! # WaveTableOscillator
//!
//! `WaveTableOscillator` generates audio waveforms using pre-calculated wave tables. It supports sine, square,
//! bidirectional square, sawtooth, triangle, and white noise waveforms.
//!
//! # MultiOscillator
//!
//! `MultiOscillator` combines multiple `WaveTableOscillator` instances into a single oscillator that generates
//! samples by summing the output of each individual oscillator.
//!
//! # Note
//!
//! - The `WaveTableOscillator` and `MultiOscillator` structs implement the `Source` trait from the `rodio` crate,
//!   allowing them to be used as audio sources for audio playback.
//!
//! - The `WaveTableOscillator` and `MultiOscillator` structs implement the `Iterator` trait, allowing them to be
//!   used in iterator contexts to generate a stream of audio samples.
//!
//! - The `MultiOscillator` struct supports adding and removing individual oscillators dynamically, as well as setting
//!   frequency and gain for each oscillator separately.
use std::{f32::consts::PI, ops::Add};
use rand::seq::index;
use rand_distr::{Distribution, Uniform};
use rodio::Source;

#[allow(dead_code)]
/// Types of oscillators.
#[derive(Clone, Debug)]
pub enum Oscillator {
    /// Sine wave oscillator.
    Sine,
    /// Square wave oscillator.
    Square,
    /// Bidirectional square wave oscillator.
    BidirectionalSquare,
    /// Sawtooth wave oscillator.
    Saw,
    /// Triangle wave oscillator.
    Triangle,
    /// White noise generator.
    WhiteNoise
}

/// Convert WavetableOscillator parameters in to a vector and use aligned_allocator to play each sample from the wavetable
#[derive(Clone, Debug)]
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
    /// Creates a new `WaveTableOscillator` with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `sample_rate` - The sample rate in Hz.
    /// * `wave_table_size` - The size of the wave table.
    /// * `oscillator` - The type of oscillator.
    /// * `gain` - The gain of the oscillator (between 0 and 1).
    /// * `frequency` - The frequency of the oscillator in Hz.
    ///
    /// # Returns
    ///
    /// A new `WaveTableOscillator` instance.
    pub fn new(sample_rate: u32, wave_table_size: usize, oscillator: Oscillator, gain: f32, frequency: f32) -> Self {
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
                        wave_table.push(0.4 * gain);
                    } else {
                        wave_table.push(0.0);
                    }
                }
            },
            Oscillator::BidirectionalSquare => {
                for i in 0..wave_table_size {
                    if i < wave_table_size/2 {
                        wave_table.push(0.4 * gain);
                    } else {
                        wave_table.push(-0.4 * gain);
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
            index_increment: frequency * wave_table_size as f32 / sample_rate as f32
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) -> Result<(), String> {
        if frequency <= 0.0 {
            return Err("Frequency must be a positive floating point value!".to_owned());
        }
        self.index_increment = frequency * self.wave_table_size as f32 / self.sample_rate as f32;
        Ok(())
    }

    pub fn set_oscillator(&mut self, oscillator: Oscillator) {
        let mut wave_table: Vec<f32> = Vec::new();
        self.oscillator = oscillator.clone();
        match oscillator {
            Oscillator::Sine => {
                for i in 0..self.wave_table_size {
                    wave_table.push((2.0 * PI * (i as f32)/(self.wave_table_size as f32)).sin() * self.gain);
                }
            },
            Oscillator::Square => {
                for i in 0..self.wave_table_size {
                    if i < self.wave_table_size/2 {
                        wave_table.push(0.99 * self.gain);
                    } else {
                        wave_table.push(0.0);
                    }
                }
            },
            Oscillator::BidirectionalSquare => {
                for i in 0..self.wave_table_size {
                    if i < self.wave_table_size/2 {
                        wave_table.push(0.99 * self.gain);
                    } else {
                        wave_table.push(-0.99 * self.gain);
                    }
                }
            },
            Oscillator::Saw => {
                for i in 1..=self.wave_table_size {
                    wave_table.push((((self.wave_table_size as f32 - i as f32)/(self.wave_table_size as f32) * 2.0) - 1.0) * self.gain);
                }
            },
            Oscillator::Triangle => {
                for i in 0..self.wave_table_size/2 {
                    wave_table.push((((i as f32/self.wave_table_size as f32)*4.0) - 1.0) * self.gain)
                }
                for i in self.wave_table_size/2..self.wave_table_size {
                    wave_table.push(((-(i as f32/self.wave_table_size as f32)*4.0) + 3.0) * self.gain)
                }
            },
            Oscillator::WhiteNoise => ()
        }
        self.wave_table = wave_table;
    }

    #[allow(dead_code)]
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

impl Add for WaveTableOscillator {
    type Output = MultiOscillator;
    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.sample_rate == rhs.sample_rate, "Sample rates must match for addition");
        let mut new_osc = MultiOscillator::new(self.sample_rate);
        let _ = new_osc.push(self);
        let _ = new_osc.push(rhs);
        new_osc
    }
}

/// `MultiOscillator` combines multiple `WaveTableOscillator` instances into a single oscillator.
#[derive(Clone, Debug)]
pub struct MultiOscillator {
    multi_osc: Vec<WaveTableOscillator>,
    sample_rate: u32,
    normalization: f32
}

impl MultiOscillator{
    /// Creates a new `MultiOscillator` with the specified sample rate.
    ///
    /// # Arguments
    ///
    /// * `sample_rate` - The sample rate in Hz.
    ///
    /// # Returns
    ///
    /// A new `MultiOscillator` instance.
    pub fn new(sample_rate: u32) -> Self {
        Self {
            multi_osc: Vec::new(),
            sample_rate: sample_rate,
            normalization: 1.0
        }
    }

    /// Creates a new `MultiOscillator` from a single `WaveTableOscillator`.
    ///
    /// This method creates a new `MultiOscillator` instance with the specified sample rate and adds
    /// the provided `WaveTableOscillator` as its sole source.
    ///
    /// # Arguments
    ///
    /// * `oscillator` - The `WaveTableOscillator` to be added to the `MultiOscillator`.
    ///
    /// # Returns
    ///
    /// A new `MultiOscillator` instance containing the provided `WaveTableOscillator`.
    #[allow(dead_code)]
    pub fn from(oscillator: WaveTableOscillator) -> Self {
        let mut m_osc = MultiOscillator::new(oscillator.sample_rate);
        m_osc.normalization = oscillator.gain;
        m_osc.multi_osc.push(oscillator);
        m_osc
    }

    /// Adds a `WaveTableOscillator` to the `MultiOscillator`.
    ///
    /// This method adds a new `WaveTableOscillator` to the `MultiOscillator`. The sample rate of
    /// the new oscillator must match the sample rate of the `MultiOscillator`.
    ///
    /// # Arguments
    ///
    /// * `oscillator` - The `WaveTableOscillator` to be added to the `MultiOscillator`.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an error message if the sample rates do not match.
    pub fn push(&mut self, oscillator: WaveTableOscillator) -> Result<(), String> {
        if oscillator.sample_rate != self.sample_rate {
            return Err("Sample rate must be the same!".to_owned());
        }
        self.normalization += oscillator.gain;
        self.multi_osc.push(oscillator);
        Ok(())
    }

    /// Sets the frequency of a source oscillator in the `MultiOscillator`.
    ///
    /// This method sets the frequency of the oscillator at the specified index within the
    /// `MultiOscillator`.
    ///
    /// # Arguments
    ///
    /// * `frequency` - The new frequency in Hz.
    /// * `source_index` - The index of the source oscillator to modify.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an error message if the frequency is invalid.
    pub fn set_frequency(&mut self, frequency: f32, source_index: usize) -> Result<(), String> {
        self.multi_osc[source_index].set_frequency(frequency)?;
        Ok(())
    }

     /// Sets the frequency of all source oscillators in the `MultiOscillator`.
    ///
    /// This method sets the frequency of all oscillators within the `MultiOscillator` to the
    /// specified value.
    ///
    /// # Arguments
    ///
    /// * `frequency` - The new frequency in Hz.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an error message if the frequency is invalid.
    pub fn global_set_frequency(&mut self, frequency: f32) -> Result<(), String> {
        for osc in self.multi_osc.iter_mut(){
            osc.set_frequency(frequency)?;
        }
        Ok(())
    }

    /// Sets the gain of a source oscillator in the `MultiOscillator`.
    ///
    /// This method sets the gain (amplitude) of the oscillator at the specified index within the
    /// `MultiOscillator`.
    ///
    /// # Arguments
    ///
    /// * `gain` - The new gain value, between 0.0 and 1.0.
    /// * `source_index` - The index of the source oscillator to modify.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an error message if the gain is out of range.
    #[allow(dead_code)]
    pub fn set_gain(&mut self, gain: f32, source_index: usize) -> Result<(), String> {
        self.multi_osc[source_index].set_gain(gain)?;
        Ok(())
    }

    /// Sets the oscillator type of a source oscillator in the `MultiOscillator`.
    ///
    /// This method sets the oscillator type (waveform) of the oscillator at the specified index
    /// within the `MultiOscillator`.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the source oscillator to modify.
    /// * `oscillator` - The new oscillator type.
    pub fn set_oscillator(&mut self, index: usize, oscillator: Oscillator) {
        let osc = self.multi_osc.get_mut(index);
        match osc {
            Some(value) => value.set_oscillator(oscillator),
            None => panic!("index out of bounds in oscillator")
        }
    }

    /// Returns the number of source oscillators in the `MultiOscillator`.
    pub fn num_sources(&self) -> usize {
        self.multi_osc.len()
    }

    /// Removes a source oscillator from the `MultiOscillator`.
    ///
    /// This method removes the source oscillator at the specified index from the `MultiOscillator`
    /// and returns it.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the source oscillator to remove.
    ///
    /// # Returns
    ///
    /// The removed `WaveTableOscillator`.
    pub fn remove(&mut self, index: usize) -> WaveTableOscillator {
        self.multi_osc.remove(index)
    }

    /// Generates the next sample from the `MultiOscillator`.
    ///
    /// This method generates the next sample by summing the samples produced by all source
    /// oscillators within the `MultiOscillator`.
    ///
    /// # Returns
    ///
    /// The next audio sample.
    pub fn get_sample(&mut self) -> f32 {
        let mut value: f32 = 0.0;
        for osc in self.multi_osc.iter_mut() {
            value += osc.get_sample();
        }
        // value/self.normalization
        value
    }
}

impl Iterator for MultiOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.get_sample())
    }
}

impl Source for MultiOscillator {
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

impl Add for MultiOscillator {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.sample_rate == rhs.sample_rate, "Sample rates must match for addition");
        let mut new_osc = MultiOscillator::new(self.sample_rate);
        for wave in self.multi_osc {
            let _ = new_osc.push(wave);
        }
        for wave in rhs.multi_osc {
            let _ = new_osc.push(wave);
        }
        new_osc
    }
}

impl Default for MultiOscillator {
    fn default() -> Self {
        Self {
            multi_osc: Vec::new(),
            sample_rate: 44100,
            normalization: 1.0
        }
    }
}