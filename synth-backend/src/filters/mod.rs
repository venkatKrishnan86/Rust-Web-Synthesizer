use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub enum FilterParam {
    SampleRateHz,
    CutoffFreqHz,
    CenterFreqHz,
    BandwidthHz,
}

#[derive(Clone, Debug)]
pub enum FilterType {
    LowPass,
    HighPass,
    BandPass,
}

#[derive(Clone, Debug)]
pub struct Filter {
    filter_type: FilterType,
    sample_rate_hz: f32,
    cutoff_freq_hz: f32,
    center_freq_hz: f32, // band-pass filter
    bandwith_hz: f32, // band-pass filter
    c: f32,
    d: f32, // band-pass filter
    xh: f32,
    xh_bp: [f32; 2], // band-pass filter
}

impl Filter {
    pub fn new (
        filter_type: FilterType,
        sample_rate_hz: f32,
        cutoff_freq_hz: f32,
        center_freq_hz: f32,
        bandwith_hz: f32,
    ) -> Self {
        match filter_type {
            FilterType::LowPass | FilterType::HighPass => {
                let c = ((PI * cutoff_freq_hz / sample_rate_hz).tan() - 1.0)
                    / ((PI * cutoff_freq_hz / sample_rate_hz).tan() + 1.0);
                Self {
                    filter_type: filter_type,
                    sample_rate_hz: sample_rate_hz,
                    cutoff_freq_hz: cutoff_freq_hz,
                    center_freq_hz: 0.0,
                    bandwith_hz: 0.0,
                    c: c,
                    d: 0.0,
                    xh: 0.0,
                    xh_bp: [0.0, 0.0],
                }
            }
            FilterType::BandPass => {
                let c = ((PI * bandwith_hz / sample_rate_hz).tan() - 1.0)
                    / ((PI * bandwith_hz / sample_rate_hz).tan() + 1.0);
                let d = -(PI * center_freq_hz / sample_rate_hz).cos();
                Self {
                    filter_type: filter_type,
                    sample_rate_hz: sample_rate_hz,
                    cutoff_freq_hz: 0.0,
                    center_freq_hz: center_freq_hz,
                    bandwith_hz: bandwith_hz,
                    c: c,
                    d: d,
                    xh: 0.0,
                    xh_bp: [0.0, 0.0],
                }
            }
        }

    }

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

    pub fn reset(&mut self) {
        self.xh = 0.0;
        self.xh_bp = [0.0, 0.0];
    }

    pub fn set_param(&mut self, param: FilterParam, value: f32) {
        match param {
            FilterParam::SampleRateHz => self.sample_rate_hz = value,
            FilterParam::CutoffFreqHz => self.cutoff_freq_hz = value,
            FilterParam::CenterFreqHz => self.center_freq_hz = value,
            FilterParam::BandwidthHz => self.bandwith_hz = value,
        }
        match self.filter_type {
            FilterType::LowPass | FilterType::HighPass => {
                self.c = ((PI * self.cutoff_freq_hz / self.sample_rate_hz).tan() - 1.0) 
                    / ((PI * self.cutoff_freq_hz / self.sample_rate_hz).tan() + 1.0);
            }
            FilterType::BandPass => {
                self.c = ((PI * self.bandwith_hz / self.sample_rate_hz).tan() - 1.0) 
                    / ((PI * self.bandwith_hz / self.sample_rate_hz).tan() + 1.0);
                self.d = -(2.0 * PI * self.center_freq_hz / self.sample_rate_hz).cos();
            }
        }
    }
}
