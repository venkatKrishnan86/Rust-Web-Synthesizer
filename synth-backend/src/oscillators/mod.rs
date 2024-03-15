use std::{f32::consts::PI, ops::Add};
use rand_distr::{Distribution, Uniform};
use rodio::Source;

#[allow(dead_code)]
#[derive(Clone)]
pub enum Oscillator {
    Sine,
    Square,
    BidirectionalSquare,
    Saw,
    Triangle,
    WhiteNoise
}

/// Convert WavetableOscillator parameters in to a vector and use aligned_allocator to play each sample from the wavetable
#[derive(Clone)]
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
                        wave_table.push(0.99 * gain);
                    } else {
                        wave_table.push(0.0);
                    }
                }
            },
            Oscillator::BidirectionalSquare => {
                for i in 0..wave_table_size {
                    if i < wave_table_size/2 {
                        wave_table.push(0.99 * gain);
                    } else {
                        wave_table.push(-0.99 * gain);
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

#[derive(Clone)]
pub struct MultiOscillator {
    multi_osc: Vec<WaveTableOscillator>,
    sample_rate: u32,
    normalization: f32
}

impl MultiOscillator{
    pub fn new(sample_rate: u32) -> Self {
        Self {
            multi_osc: Vec::new(),
            sample_rate: sample_rate,
            normalization: 1.0
        }
    }

    #[allow(dead_code)]
    pub fn from(oscillator: WaveTableOscillator) -> Self {
        let mut m_osc = MultiOscillator::new(oscillator.sample_rate);
        m_osc.normalization = oscillator.gain;
        m_osc.multi_osc.push(oscillator);
        m_osc
    }

    pub fn push(&mut self, oscillator: WaveTableOscillator) -> Result<(), String> {
        if oscillator.sample_rate != self.sample_rate {
            return Err("Sample rate must be the same!".to_owned());
        }
        self.normalization += oscillator.gain;
        self.multi_osc.push(oscillator);
        Ok(())
    }

    pub fn set_frequency(&mut self, frequency: f32, source_index: usize) -> Result<(), String> {
        self.multi_osc[source_index].set_frequency(frequency)?;
        Ok(())
    }

    pub fn global_set_frequency(&mut self, frequency: f32) -> Result<(), String> {
        for osc in self.multi_osc.iter_mut(){
            osc.set_frequency(frequency)?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn set_gain(&mut self, gain: f32, source_index: usize) -> Result<(), String> {
        self.multi_osc[source_index].set_gain(gain)?;
        Ok(())
    }

    pub fn num_sources(&self) -> usize {
        self.multi_osc.len()
    }

    pub fn get_sample(&mut self) -> f32 {
        let mut value: f32 = 0.0;
        for osc in self.multi_osc.iter_mut() {
            value += osc.get_sample();
        }
        value/self.normalization
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