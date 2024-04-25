use crate::oscillators::{WaveTableOscillator};
use crate::utils::RingBuffer;

#[derive(Clone, Debug)]
pub struct Vibrato {
    sample_rate_hz: f32,
    width_sample: usize,
    lfo: WaveTableOscillator,
    delay_line: RingBuffer<f32>,
}

impl Vibrato {
    pub fn new(sample_rate_hz: f32, width_sec: f32, lfo: WaveTableOscillator) -> Self {
        let width_sample = (width_sec * sample_rate_hz).round() as usize;
        Self {
            sample_rate_hz: sample_rate_hz,
            width_sample: width_sample,
            lfo: lfo,
            delay_line: RingBuffer::new(2 + width_sample * 3),
        }
    }

    pub fn reset(&mut self) {
        self.delay_line.reset();
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let modulator = self.lfo.get_sample();
        let offset = 1.0 + self.width_sample as f32 + self.width_sample as f32 * modulator;
        let _ = self.delay_line.pop();
        self.delay_line.push(input);
        self.delay_line.get_frac(offset)
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.lfo.set_frequency(frequency);
    }

    pub fn set_width(&mut self, width_sec: f32) {
        self.width_sample = (width_sec * self.sample_rate_hz).round() as usize;
    }
}