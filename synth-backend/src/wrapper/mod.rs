use crate::oscillators::{MultiOscillator, Oscillator, WaveTableOscillator};
use crate::filters::{Filter, FilterType};
use std::ops::Add;

#[derive(Clone, Debug)]
pub struct Synth {
    pub osc: MultiOscillator,
    pub sample_rate: u32,
    pub filter: Option<Filter>, // Make filter an optional field
}

impl Synth {
    pub fn new(osc: MultiOscillator, sample_rate: u32, filter: Option<Filter>) -> Self {
        Self {
            osc,
            sample_rate,
            filter,
        }
    }

    pub fn get_sample(&mut self) -> f32 {
        // Call the get_sample method of MultiOscillator
        let sample = self.osc.get_sample();

        // Check if filter exists
        if let Some(ref mut filter) = self.filter {
            // Apply the filter if it exists
            let filtered_sample = filter.process(sample);
            return filtered_sample;
        }

        // If filter is None, return the sample directly
        sample
    }

    pub fn set_oscillator(&mut self, index: usize, oscillator: Oscillator) {
        self.osc.set_oscillator(index, oscillator);
    }

    pub fn push(&mut self, oscillator: WaveTableOscillator) -> Result<(), String> {
        self.osc.push(oscillator)
    }

    pub fn global_set_frequency(&mut self, frequency: f32) -> Result<(), String> {
        self.osc.global_set_frequency(frequency)
    }

    pub fn num_sources(&self) -> usize {
        self.osc.num_sources()
    }
}
