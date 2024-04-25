//! Filters
//!
//! This module provides digital filter implementations including low-pass, high-pass, and
//! band-pass filters.
//!
//! # Examples
//!
//! ```
//! use synth_backend::filters::{Filter, FilterParam, FilterType};
//!
//! // Create a new low-pass filter with the following parameters:
//! // - Sample rate: 44100 Hz
//! // - Cutoff frequency: 1000 Hz
//! let mut filter = Filter::new(FilterType::LowPass, 44100.0, 1000.0, 0.0);
//!
//! // Process an input sample through the filter
//! let output = filter.process(0.5);
//!
//! // Reset the filter to its initial state
//! filter.reset();
//!
//! // Change the filter type to high-pass
//! filter.change_filter_type(FilterType::HighPass);
//! ```
use std::f32::consts::PI;

/// Parameters that can be set for a filter.
#[derive(Debug, Clone, Copy)]
pub enum FilterParam {
    /// Sample rate in Hertz.
    SampleRateHz,
    /// Cutoff frequency in Hertz.
    FreqHz,
    /// Bandwidth in Hertz (only applicable for band-pass filters).
    BandwidthHz,
}

/// Types of digital filters.
#[derive(Clone, Debug)]
pub enum FilterType {
    /// Low-pass filter.
    LowPass,
    /// High-pass filter.
    HighPass,
    /// Band-pass filter.
    BandPass,
}

/// Represents a digital filter.
#[derive(Clone, Debug)]
pub struct Filter {
    filter_type: FilterType,
    sample_rate_hz: f32,
    freq_hz: f32,
    bandwidth_hz: f32, // band-pass filter
    c: f32,
    d: f32, // band-pass filter
    xh: f32,
    xh_bp: [f32; 2], // band-pass filter
}

impl Filter {
    /// Creates a new digital filter with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `filter_type` - The type of filter (LowPass, HighPass, or BandPass).
    /// * `sample_rate_hz` - The sample rate in Hertz.
    /// * `freq_hz` - The cutoff or center frequency in Hertz.
    /// * `bandwidth_hz` - The bandwidth in Hertz (only applicable for band-pass filters).
    ///
    /// # Returns
    ///
    /// A new `Filter` instance.
    pub fn new (
        filter_type: FilterType,
        sample_rate_hz: f32,
        freq_hz: f32,
        bandwidth_hz: f32,
    ) -> Self {
        match filter_type {
            FilterType::LowPass | FilterType::HighPass => {
                let c = ((PI * freq_hz / sample_rate_hz).tan() - 1.0)
                    / ((PI * freq_hz / sample_rate_hz).tan() + 1.0);
                Self {
                    filter_type: filter_type,
                    sample_rate_hz: sample_rate_hz,
                    freq_hz: freq_hz,
                    bandwidth_hz: bandwidth_hz,
                    c: c,
                    d: 0.0,
                    xh: 0.0,
                    xh_bp: [0.0, 0.0],
                }
            }
            FilterType::BandPass => {
                let c = ((PI * bandwidth_hz / sample_rate_hz).tan() - 1.0)
                    / ((PI * bandwidth_hz / sample_rate_hz).tan() + 1.0);
                let d = -(2.0 * PI * freq_hz / sample_rate_hz).cos();
                Self {
                    filter_type: filter_type,
                    sample_rate_hz: sample_rate_hz,
                    freq_hz: freq_hz,
                    bandwidth_hz: bandwidth_hz,
                    c: c,
                    d: d,
                    xh: 0.0,
                    xh_bp: [0.0, 0.0],
                }
            }
        }

    }

    /// Process an input sample through the filter and returns the output.
    ///
    /// # Arguments
    ///
    /// * `input` - The input sample.
    ///
    /// # Returns
    ///
    /// The output sample after filtering.
    pub fn process(&mut self, input: f32) -> f32 {
        match self.filter_type {
            FilterType::LowPass => self.process_lp(input),
            FilterType::HighPass => self.process_hp(input),
            FilterType::BandPass => self.process_bp(input),
        }
    }

    fn process_lp(&mut self, input: f32) -> f32 {
        let xh_new = input - self.c * self.xh;
        let ap_y = self.c * xh_new + self.xh;
        self.xh = xh_new;
        0.5 * (input + ap_y)
    }

    fn process_hp(&mut self, input: f32) -> f32 {
        let xh_new = input - self.c * self.xh;
        let ap_y = self.c * xh_new + self.xh;
        self.xh = xh_new;
        0.5 * (input - ap_y)
    }

    fn process_bp(&mut self, input: f32) -> f32 {
        let xh_new = input - self.d * (1.0 - self.c) * self.xh_bp[0] + self.c * self.xh_bp[1];
        let ap_y = -self.c * xh_new + self.d * (1.0 - self.c) * self.xh_bp[0] + self.xh_bp[1];
        self.xh_bp[1] = self.xh_bp[0];
        self.xh_bp[0] = xh_new;
        0.5 * (input - ap_y)
    }

    /// Resets the filter to its initial state.
    pub fn reset(&mut self) {
        self.xh = 0.0;
        self.xh_bp = [0.0, 0.0];
    }

    /// Sets a parameter of the filter to the specified value.
    ///
    /// # Arguments
    ///
    /// * `param` - The parameter to set.
    /// * `value` - The new value of the parameter.
    pub fn set_param(&mut self, param: FilterParam, value: f32) {
        match param {
            FilterParam::SampleRateHz => self.sample_rate_hz = value,
            FilterParam::FreqHz => self.freq_hz = value,
            FilterParam::BandwidthHz => self.bandwidth_hz = value,
        }
        match self.filter_type {
            FilterType::LowPass | FilterType::HighPass => {
                self.c = ((PI * self.freq_hz / self.sample_rate_hz).tan() - 1.0) 
                    / ((PI * self.freq_hz / self.sample_rate_hz).tan() + 1.0);
            }
            FilterType::BandPass => {
                self.c = ((PI * self.bandwidth_hz / self.sample_rate_hz).tan() - 1.0) 
                    / ((PI * self.bandwidth_hz / self.sample_rate_hz).tan() + 1.0);
                self.d = -(2.0 * PI * self.freq_hz / self.sample_rate_hz).cos();
            }
        }
    }

    /// Changes the filter type to the specified type.
    ///
    /// # Arguments
    ///
    /// * `filter_type` - The new filter type.
    pub fn change_filter_type(&mut self, filter_type: FilterType) {
        self.filter_type = filter_type;
        match self.filter_type {
            FilterType::LowPass | FilterType::HighPass => {
                self.c = ((PI * self.freq_hz / self.sample_rate_hz).tan() - 1.0) 
                    / ((PI * self.freq_hz / self.sample_rate_hz).tan() + 1.0);
            }
            FilterType::BandPass => {
                self.c = ((PI * self.bandwidth_hz / self.sample_rate_hz).tan() - 1.0) 
                    / ((PI * self.bandwidth_hz / self.sample_rate_hz).tan() + 1.0);
                self.d = -(2.0 * PI * self.freq_hz / self.sample_rate_hz).cos();
            }
        }
    }
}
