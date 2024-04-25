//! ADSR: Attack Decay Sustain Release
//!
//! This module defines an ADSR envelope generator.
//!
//! # Examples
//!
//! ```
//! use synth_backend::envelopes::{Envelope, EnvelopeParam};
//!
//! // Create a new ADSR envelope with the following parameters:
//! // - Sample rate: 44100 Hz
//! // - Attack time: 100 ms
//! // - Decay time: 200 ms
//! // - Sustain level: 0.5
//! // - Release time: 300 ms
//! let mut envelope = Envelope::new(44100.0, 100.0, 200.0, 0.5, 300.0);
//!
//! // Get the amplitude of the envelope at a given step in time
//! let amplitude = envelope.get_amplitude();
//!
//! // Reset the envelope to its initial state
//! envelope.reset();
//!
//! // Set the attack time to 50 ms
//! envelope.set_param(EnvelopeParam::AttackMs, 50.0);
//! ```

#[derive(Clone, Debug)]
pub enum EnvelopeParam {
    /// Attack time in milliseconds (ms)
    AttackMs,
    /// Decay time in ms
    DecayMs,
    /// Sustain level as a percentage (0.0 - 1.0)
    SustainPercentage,
    /// Release time in ms
    ReleaseMs,
}

#[derive(Clone, Debug)]
pub struct Envelope {
    current_step: usize,
    sample_rate_hz: f32,
    attack_step: usize,
    decay_step: usize,
    sustain_percentage: f32,
    release_step: usize
}

impl Envelope {
    /// Creates a new ADSR envelope with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `sample_rate_hz` - Sample rate in Hertz.
    /// * `attack_ms` - Attack time in milliseconds.
    /// * `decay_ms` - Decay time in milliseconds.
    /// * `sustain_percentage` - Sustain level as a percentage (0.0 - 1.0).
    /// * `release_ms` - Release time in milliseconds.
    ///
    /// # Returns
    ///
    /// A new `Envelope` instance.
    pub fn new(
        sample_rate_hz: f32,
        attack_ms: f32,
        decay_ms: f32,
        sustain_percentage: f32,
        release_ms: f32
    ) -> Self 
    {
        Self {
            current_step: 0,
            sample_rate_hz: sample_rate_hz,
            attack_step: (attack_ms * sample_rate_hz / 1000.0) as usize,
            decay_step: (decay_ms * sample_rate_hz / 1000.0) as usize,
            sustain_percentage: sustain_percentage,
            release_step: (release_ms * sample_rate_hz / 1000.0) as usize
        }
    }

    /// Returns the current amplitude of the envelope.
    ///
    /// The amplitude is calculated based on the current time step and envelope parameters.
    ///
    /// # Returns
    ///
    /// The current amplitude value.
    pub fn get_amplitude(&mut self) -> f32 {
        self.current_step += 1;
        if self.current_step < self.attack_step {
            self.current_step as f32 / self.attack_step as f32
        } else if self.current_step < self.attack_step + self.decay_step {
            1.0 - (1.0 - self.sustain_percentage) * ((self.current_step - self.attack_step) as f32 / self.decay_step as f32)
        } else {
            self.sustain_percentage
        }
    }

    /// Resets the envelope to its initial state.
    pub fn reset(&mut self) {
        self.current_step = 0;
    }

    /// Sets a parameter of the envelope to the specified value.
    ///
    /// # Arguments
    ///
    /// * `param` - Parameter to set.
    /// * `value` - New value of the parameter.
    pub fn set_param(&mut self, param: EnvelopeParam, value: f32) {
        match param {
            EnvelopeParam::AttackMs => {
                self.attack_step = (value * self.sample_rate_hz / 1000.0) as usize;
            }
            EnvelopeParam::DecayMs => {
                self.decay_step = (value * self.sample_rate_hz / 1000.0) as usize;
            }
            EnvelopeParam::SustainPercentage => {
                self.sustain_percentage = value;
            }
            EnvelopeParam::ReleaseMs => {
                self.release_step = (value * self.sample_rate_hz / 1000.0) as usize;
            }
        }
    }
}