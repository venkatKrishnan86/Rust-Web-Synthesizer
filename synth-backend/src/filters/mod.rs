use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub enum FilterParam {
    SampleRateHz,
    CutoffFreqHz,
    CenterFreqHz,
    BandwidthHz,
}

/// Trait representing a filter.
pub trait Filter {
    /// Process a single sample.
    fn process(&mut self, input: f32) -> f32;

    /// Reset the filter to its initial state.
    fn reset(&mut self);

    /// Get the parameters of the filter.
    fn get_param(&self, param: FilterParam) -> f32;

    /// Set the parameters of the filter.
    fn set_param(&mut self, params: FilterParam, value: f32);
}

/// A simple first-order low-pass filter.
pub struct LowPassFilter {
    sample_rate_hz: f32,
    cutoff_freq_hz: f32,
    c: f32,
    xh: f32,
}

impl LowPassFilter {
    /// Create a new low-pass filter with the specified parameters.
    pub fn new(sample_rate_hz: f32, cutoff_freq_hz: f32) -> Self {
        let c = ((PI * cutoff_freq_hz / sample_rate_hz).tan() - 1.0)
            / ((PI * cutoff_freq_hz / sample_rate_hz).tan() + 1.0);
        Self {
            sample_rate_hz: sample_rate_hz,
            cutoff_freq_hz: cutoff_freq_hz,
            c: c,
            xh: 0.0,
        }
    }
}

impl Filter for LowPassFilter {
    fn process(&mut self, input: f32) -> f32 {
        let xh_new = input - self.c * self.xh;
        let ap_y = self.c * xh_new + self.xh;
        self.xh = xh_new;
        0.5 * (input + ap_y)
    }
    
    fn reset(&mut self) {
        self.xh = 0.0;
    }
    
    fn get_param(&self, param: FilterParam) -> f32 {
        match param {
            FilterParam::SampleRateHz => self.sample_rate_hz,
            FilterParam::CutoffFreqHz => self.cutoff_freq_hz,
            _ => panic!("Invalid parameter for lowpass filter!")
        }
    }

    fn set_param(&mut self, params: FilterParam, value: f32) {
        match params {
            FilterParam::SampleRateHz => {self.sample_rate_hz = value},
            FilterParam::CutoffFreqHz => {self.cutoff_freq_hz = value},
            _ => panic!("Invalid parameter for lowpass filter!")
        }
        self.c = ((PI * self.cutoff_freq_hz / self.sample_rate_hz).tan() - 1.0)
            / ((PI * self.cutoff_freq_hz / self.sample_rate_hz).tan() + 1.0);
    }
}

/// A simple first-order low-pass filter.
pub struct HighPassFilter {
    pub sample_rate_hz: f32,
    pub cutoff_freq_hz: f32,
    c: f32,
    xh: f32,
}

impl HighPassFilter {
    /// Create a new low-pass filter with the specified parameters.
    pub fn new(sample_rate_hz: f32, cutoff_freq_hz: f32) -> Self {
        let c = ((PI * cutoff_freq_hz / sample_rate_hz).tan() - 1.0)
            / ((PI * cutoff_freq_hz / sample_rate_hz).tan() + 1.0);
        Self {
            sample_rate_hz: sample_rate_hz,
            cutoff_freq_hz: cutoff_freq_hz,
            c: c,
            xh: 0.0,
        }
    }
}

impl Filter for HighPassFilter {
    fn process(&mut self, input: f32) -> f32 {
        let xh_new = input - self.c * self.xh;
        let ap_y = self.c * xh_new + self.xh;
        self.xh = xh_new;
        0.5 * (input - ap_y)
    }
    
    fn reset(&mut self) {
        self.xh = 0.0;
    }
    
    fn get_param(&self, param: FilterParam) -> f32 {
        match param {
            FilterParam::SampleRateHz => self.sample_rate_hz,
            FilterParam::CutoffFreqHz => self.cutoff_freq_hz,
            _ => panic!("Invalid parameter for lowpass filter!")
        }
    }

    fn set_param(&mut self, params: FilterParam, value: f32) {
        match params {
            FilterParam::SampleRateHz => {self.sample_rate_hz = value},
            FilterParam::CutoffFreqHz => {self.cutoff_freq_hz = value},
            _ => panic!("Invalid parameter for lowpass filter!")
        }
        self.c = ((PI * self.cutoff_freq_hz / self.sample_rate_hz).tan() - 1.0)
            / ((PI * self.cutoff_freq_hz / self.sample_rate_hz).tan() + 1.0);
    }
}

/// A simple first-order band-pass filter.
pub struct BandPassFilter {
    pub sample_rate_hz: f32,
    pub center_freq_hz: f32,
    pub bandwith_hz: f32,
    c: f32,
    d: f32,
    xh: [f32; 2],
}

impl BandPassFilter {
    /// Create a new band-pass filter with the specified parameters.
    pub fn new(sample_rate_hz: f32, center_freq_hz: f32, bandwith_hz: f32) -> Self {
        let c = ((PI * bandwith_hz / sample_rate_hz).tan() - 1.0) 
            / ((PI * bandwith_hz / sample_rate_hz).tan() + 1.0);
        let d = -(PI * center_freq_hz / sample_rate_hz).cos();
        Self {
            sample_rate_hz: sample_rate_hz,
            center_freq_hz: center_freq_hz,
            bandwith_hz: bandwith_hz,
            c: c,
            d: d,
            xh: [0.0, 0.0],
        }
    }
}

impl Filter for BandPassFilter {
    fn process(&mut self, input: f32) -> f32 {
        let xh_new = input - self.d * (1.0 - self.c) * self.xh[0] + self.c * self.xh[1];
        let ap_y = -self.c * xh_new + self.d * (1.0 - self.c) * self.xh[0] + self.xh[1];
        self.xh = [xh_new, self.xh[0]];
        0.5 * (input - ap_y)
    }

    fn reset(&mut self) {
        self.xh = [0.0, 0.0];
    }
    
    fn get_param(&self, param: FilterParam) -> f32 {
        match param {
            FilterParam::SampleRateHz => self.sample_rate_hz,
            FilterParam::CenterFreqHz => self.center_freq_hz,
            FilterParam::BandwidthHz => self.bandwith_hz,
            _ => panic!("Invalid parameter for bandpass filter!")
        }
    }

    fn set_param(&mut self, params: FilterParam, value: f32) {
        match params {
            FilterParam::SampleRateHz => {self.sample_rate_hz = value},
            FilterParam::CenterFreqHz => {self.center_freq_hz = value},
            FilterParam::BandwidthHz => {self.bandwith_hz = value},
            _ => panic!("Invalid parameter for bandpass filter!")
        }
        self.c = ((PI * self.bandwith_hz / self.sample_rate_hz).tan() - 1.0) 
            / ((PI * self.bandwith_hz / self.sample_rate_hz).tan() + 1.0);
        self.d = -(PI * self.center_freq_hz / self.sample_rate_hz).cos();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_get_set_params() {
        let sample_rate_hz = 48000.0;
        let cutoff_freq_hz = 12000.0;
        let mut  filter = LowPassFilter::new(sample_rate_hz, cutoff_freq_hz);
        assert_eq!(filter.get_param(FilterParam::SampleRateHz), sample_rate_hz);
        assert_eq!(filter.get_param(FilterParam::CutoffFreqHz), cutoff_freq_hz);

        filter.set_param(FilterParam::CutoffFreqHz, 1000.0);
        assert_eq!(filter.get_param(FilterParam::CutoffFreqHz), 1000.0);
    }

    #[test]
    fn test_c_value() {
        let sample_rate_hz = 48000.0;
        let cutoff_freq_hz = 12000.0;
        let filter = LowPassFilter::new(sample_rate_hz, cutoff_freq_hz);    
        assert!(filter.c.abs() < f32::EPSILON);

        let filter = HighPassFilter::new(sample_rate_hz, cutoff_freq_hz);        
        assert!(filter.c.abs() < f32::EPSILON);
    }

    #[test]
    fn test_low_pass() {
        let sample_rate_hz = 48000.0;
        let cutoff_freq_hz = 12000.0;
        let mut filter = LowPassFilter::new(sample_rate_hz, cutoff_freq_hz);
    
        // create a sinuoidal signal at 1 kHz
        let mut input = vec![0.0; 48000];
        for i in 0..48000 {
            input[i] = (2.0 * PI * 880.0 * i as f32 / sample_rate_hz).sin();
        }

        // apply the filter
        let mut output = vec![0.0; 48000];
        for i in 0..48000 {
            output[i] = filter.process(input[i]);
        }

        // check the output
        for i in 1..48000 {
            assert!(output[i] - (input[i] + input[i-1]) / 2.0 < f32::EPSILON);
        }
    }

    #[test]
    fn test_high_pass() {
        let sample_rate_hz = 48000.0;
        let cutoff_freq_hz = 12000.0;
        let mut filter = HighPassFilter::new(sample_rate_hz, cutoff_freq_hz);
    
        // create a sinuoidal signal at 1 kHz
        let mut input = vec![0.0; 48000];
        for i in 0..48000 {
            input[i] = (2.0 * PI * 880.0 * i as f32 / sample_rate_hz).sin();
        }

        // apply the filter
        let mut output = vec![0.0; 48000];
        for i in 0..48000 {
            output[i] = filter.process(input[i]);
        }

        // check the output
        for i in 1..48000 {
            assert!(output[i] - (input[i] - input[i-1]) / 2.0 < f32::EPSILON);
        }
    }

}

