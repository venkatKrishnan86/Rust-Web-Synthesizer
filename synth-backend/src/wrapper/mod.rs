//! Wrapper module providing a high-level interface for creating synthesizers.
//!
//! # Examples
//!
//! ```
//! use synth_backend::wrapper::{Synth, Oscillator, FilterType, EnvelopeParam};
//!
//! // Create a new synthesizer with default settings
//! let synth = Synth::new_default();
//!
//! // Set the oscillator type
//! synth.set_oscillator(0, Oscillator::Sine);
//!
//! // Set the filter type
//! synth.set_filter(Some(FilterType::LowPass), 1000.0, 500.0);
//!
//! // Set envelope parameters
//! synth.set_envelope_params(EnvelopeParam::AttackMs, 10.0);
//!
//! // Generate audio samples
//! let sample = synth.get_sample();
//! ```
//!
//! The `Synth` struct provides methods for configuring and generating audio samples from a synthesizer.
use crate::oscillators::{MultiOscillator, Oscillator, WaveTableOscillator};
use crate::filters::{Filter, FilterParam, FilterType};
use crate::envelopes::{Envelope, EnvelopeParam};
use crate::lfo::{LFOType, LFO};
use std::ops::Add;

const GAIN: f32 = 1.0;
const WIDTH: f32 = 0.010;

/// Synth struct representing a synthesizer.
#[derive(Clone, Debug)]
pub struct Synth {
    pub osc: MultiOscillator,
    pub sample_rate: u32,
    pub filter: Option<Filter>, // Make filter an optional field
    pub envelope: Option<Envelope>,
    pub lfo: Option<LFO>,
    pub lfo_type: LFOType,
}

impl Synth {
    /// Creates a new `Synth` instance with the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `osc` - The multi-oscillator to be used by the synthesizer.
    /// * `sample_rate` - The sample rate of the synthesizer.
    /// * `filter` - An optional filter to be applied to the audio output.
    /// * `envelope` - An optional envelope to shape the audio output.
    /// * `am_lfo` - An optional low-frequency oscillator for amplitude modulation.
    ///
    /// # Returns
    ///
    /// A new `Synth` instance configured with the provided parameters.
    pub fn new(osc: MultiOscillator, sample_rate: u32, filter: Option<Filter>, envelope: Option<Envelope>, lfo: Option<LFO>, lfo_type: LFOType) -> Self {
        Self {
            osc,
            sample_rate,
            filter,
            envelope,
            lfo,
            lfo_type
        }
    }

    /// Generates the next audio sample from the synthesizer.
    ///
    /// This method calculates the next audio sample by processing the output of the multi-oscillator
    /// and applying any configured filter, envelope, and amplitude modulation.
    ///
    /// # Returns
    ///
    /// The next audio sample as a 32-bit floating point value.
    pub fn get_sample(&mut self) -> f32 {
        // Call the get_sample method of MultiOscillator
        let sample = self.osc.get_sample();
        let mut output_sample = sample;

        // Check if filter exists
        if let Some(ref mut filter) = self.filter {
            // Apply the filter if it exists
            output_sample = filter.process(sample);
        }

        if let Some(ref mut envelope) = self.envelope {
            output_sample = output_sample * envelope.get_amplitude();
        }

        if let Some(ref mut lfo) = self.lfo {
            output_sample = lfo.process(output_sample);
        }

        // If filter is None, return the sample directly
        output_sample
    }

    pub fn set_gain(&mut self, index: usize, gain: f32) -> Result<(), String> {
        self.osc.set_gain(gain, index)
    }

    /// Sets the oscillator type at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the oscillator to be set.
    /// * `oscillator` - The type of oscillator to set.
    pub fn set_oscillator(&mut self, index: usize, oscillator: Oscillator) {
        self.osc.set_oscillator(index, oscillator);
    }

    pub fn set_detune_semitones(&mut self, index: usize, detune_semitones: i8) -> Result<(), String> {
        self.osc.set_detune_semitones(detune_semitones, index)
    }

    pub fn set_lfo_type(&mut self, lfo_type: LFOType) {
        match self.lfo {
            None => {
                let lfo_type = self.lfo_type.clone();
                self.lfo.as_mut().unwrap().set_type(lfo_type)
            },
            Some(_) => self.lfo.as_mut().unwrap().set_type(lfo_type)
        }
    }

    /// Removes the oscillator at the specified index and returns it.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the oscillator to be removed.
    ///
    /// # Returns
    ///
    /// The removed oscillator.
    pub fn remove(&mut self, index: usize) -> WaveTableOscillator {
        self.osc.remove(index)
    }

    /// Pushes a new oscillator into the synthesizer's multi-oscillator.
    ///
    /// # Arguments
    ///
    /// * `oscillator` - The oscillator to be added.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    pub fn push(&mut self, oscillator: WaveTableOscillator) -> Result<(), String> {
        self.osc.push(oscillator)
    }

    /// Sets the frequency of all oscillators in the multi-oscillator to the specified value.
    ///
    /// # Arguments
    ///
    /// * `frequency` - The new frequency value.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    pub fn global_set_frequency(&mut self, frequency: f32) -> Result<(), String> {
        self.osc.global_set_frequency(frequency)
    }

    /// Returns the number of sources (oscillators) in the multi-oscillator.
    ///
    /// # Returns
    ///
    /// The number of sources.
    pub fn num_sources(&self) -> usize {
        self.osc.num_sources()
    }

    /// Sets the filter type, frequency, and bandwidth of the synthesizer's filter.
    ///
    /// # Arguments
    ///
    /// * `filter` - The type of filter to set.
    /// * `freq_filter` - The frequency of the filter.
    /// * `bandwidth_hz_filter` - The bandwidth of the filter in Hertz.
    pub fn set_filter(&mut self, filter: Option<FilterType>, freq_filter: f32, bandwidth_hz_filter: f32) {
        match filter {
            None => self.filter = None,
            Some(filter_type) => match self.filter {
                None => self.filter = Some(Filter::new(filter_type, self.sample_rate as f32, freq_filter, bandwidth_hz_filter)),
                Some(_) => self.filter.as_mut().unwrap().change_filter_type(filter_type)
            }
        }
    }

     /// Sets the parameter value of the synthesizer's filter.
    ///
    /// # Arguments
    ///
    /// * `filterparam` - The filter parameter to set.
    /// * `value` - The value to set the parameter.
    pub fn set_filter_params(&mut self, filterparam: FilterParam, value: f32) {
        match self.filter {
            None => (),
            Some(_) => self.filter.as_mut().unwrap().set_param(filterparam, value)
        }
    }

    /// Sets the parameter value of the synthesizer's envelope.
    ///
    /// # Arguments
    ///
    /// * `envelope_param` - The envelope parameter to set.
    /// * `value` - The value to set the parameter to.
    pub fn set_envelope_params(&mut self, envelope_param: EnvelopeParam, value: f32) {
        match self.envelope {
            None => (),
            Some(_) => self.envelope.as_mut().unwrap().set_param(envelope_param, value)
        }
    }

    /// Sets the frequency of the amplitude modulation low-frequency oscillator (LFO).
    ///
    /// # Arguments
    ///
    /// * `frequency` - The new frequency value.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    pub fn set_lfo_frequency(&mut self, frequency: f32) {
        match self.lfo {
            None => (),
            Some(_) => self.lfo.as_mut().unwrap().set_frequency(frequency)
        }
    }

    /// Sets the type of the amplitude modulation low-frequency oscillator (LFO) and its frequency.
    ///
    /// # Arguments
    ///
    /// * `oscillator` - The type of oscillator to set.
    /// * `frequency` - The frequency of the oscillator.
    pub fn set_lfo_osc(&mut self, oscillator: Option<Oscillator>, frequency: f32, lfo_type: LFOType){
        match oscillator {
            None => self.lfo = None,
            Some(osc) => match self.lfo {
                None => self.lfo = Some(LFO::new(
                    lfo_type,
                    self.sample_rate as f32,
                    WaveTableOscillator::new(self.sample_rate, self.sample_rate as usize, osc, GAIN, frequency),
                    WIDTH
                )),
                Some(_) => self.lfo.as_mut().unwrap().set_oscillator(osc)
            }
        }
        
    }

    pub fn get_lfo_osc(&mut self) -> Option<Oscillator> {
        match self.lfo {
            None => None,
            Some(_) => Some(self.lfo.as_mut().unwrap().get_oscillator())
        }
    }
}
