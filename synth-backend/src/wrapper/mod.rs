use crate::oscillators::MultiOscillator;
use crate::filters::{Filter, FilterType};

#[derive(Clone, Debug)]
pub struct Synth {
    pub osc: MultiOscillator,
    pub filter: Option<Filter>, // Make filter an optional field
}

impl Synth {
    pub fn new(osc: MultiOscillator, filter: Option<Filter>) -> Self {
        Self {
            osc,
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
}
