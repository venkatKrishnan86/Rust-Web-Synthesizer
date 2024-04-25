use crate::oscillators::{WaveTableOscillator, Oscillator, self};
use crate::utils::RingBuffer;

#[derive(Clone, Debug)]
pub enum LFOType {
    Amplitude,
    Frequency
}

#[derive(Clone, Debug)]
pub struct LFO {
    lfo_type: LFOType,
    sample_rate_hz: f32,
    lfo: WaveTableOscillator,
    width_sample: usize, // frequency
    delay_line: RingBuffer<f32>, // frequency
}

impl LFO {
    pub fn new(lfo_type: LFOType, sample_rate_hz: f32, lfo: WaveTableOscillator, width_sec: f32) -> Self {
        let width_sample = (width_sec * sample_rate_hz).round() as usize;
        Self {
            lfo_type: lfo_type,
            sample_rate_hz: sample_rate_hz,
            lfo: lfo,
            width_sample: width_sample,
            delay_line: RingBuffer::new(2 + width_sample * 3),
        }
    }

    pub fn reset(&mut self) {
        self.delay_line.reset();
    }

    pub fn process(&mut self, input: f32) -> f32 {
        match self.lfo_type {
            LFOType::Amplitude => self.process_amplitude(input),
            LFOType::Frequency => self.process_frequency(input)
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.lfo.set_frequency(frequency);
    }

    pub fn set_type(&mut self, lfo_type: LFOType) {
        self.lfo_type = lfo_type;
        match self.lfo_type {
            LFOType::Amplitude => {},
            LFOType::Frequency => {
                self.delay_line = RingBuffer::new(2 + self.width_sample * 3);
            }
        }
    }

    pub fn set_oscillator(&mut self, oscillator: Oscillator) {
        self.lfo.set_oscillator(oscillator);
    }

    pub fn set_width(&mut self, width_sec: f32) {
        self.width_sample = (width_sec * self.sample_rate_hz).round() as usize;
    }

    fn process_frequency(&mut self, input: f32) -> f32 {
        let modulator = self.lfo.get_sample();
        let offset = 1.0 + self.width_sample as f32 + self.width_sample as f32 * modulator;
        let _ = self.delay_line.pop();
        self.delay_line.push(input);
        self.delay_line.get_frac(offset)
    }

    fn process_amplitude(&mut self, input: f32) -> f32 {
        let a = self.lfo.get_sample();
        match self.lfo.get_oscillator() {
            Oscillator::Square => input * a,
            _ => input * (a + 1.0) / 2.0,
        }
        
    }
}