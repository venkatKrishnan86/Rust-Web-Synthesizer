//! ADSR: Attack Decay Sustain Release

#[derive(Clone, Debug)]
pub enum EnvelopeParam {
    AttackMs,
    DecayMs,
    SustainPercentage,
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

    pub fn reset(&mut self) {
        self.current_step = 0;
    }

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