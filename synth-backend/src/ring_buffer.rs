use crate::oscillators::MultiOscillator;
use rodio::Source;

/// # Polyphony handler struct
/// Aim of this struct is to avoid the usage of multiple sinks on playing multiple notes, instead handle multiple notes 
/// through a new source, which would be a RingBuffer filled with MultiOScillators
#[derive(Clone)]
pub struct PolyphonyRingBuffer {
    buffer: Vec<MultiOscillator>,
    capacity: usize,
    head: Option<usize>,
    tail: Option<usize>,
    sample_rate: u32
}

impl PolyphonyRingBuffer {
    pub fn new(capacity: usize, sample_rate: u32) -> Self {
        Self {
            buffer: vec![MultiOscillator::new(sample_rate); capacity],
            capacity: capacity,
            head: None,
            tail: None,
            sample_rate: sample_rate
        }
    }

    pub fn reset(&mut self) {
        self.head = None;
        self.tail = None;
    }

    // `put` and `peek` write/read without advancing the indices.
    pub fn put(&mut self, value: MultiOscillator) {
        match self.head {
            None => {
                self.head = Some(0);
                self.tail = Some(0);
                self.buffer[0] = value;
            },
            Some(h) => self.buffer[h] = value
        }
    }

    pub fn peek(&self) -> Option<&MultiOscillator> {
        match self.tail {
            None => None,
            Some(t) => Some(&self.buffer[t])
        }
    }

    pub fn get(&self, offset: usize) -> Option<&MultiOscillator> {
        match self.tail {
            None => None,
            Some(t) => Some(&self.buffer[(t + offset) % self.capacity()])
        }
    }

    pub fn get_mutable(&mut self, offset: usize) -> Option<&mut MultiOscillator> {
        match self.tail {
            None => None,
            Some(t) => Some(&mut self.buffer[(t + offset) % self.capacity])
        }
    }

    pub fn push(&mut self, value: MultiOscillator) {
        match self.head {
            None => {
                self.head = Some(0);
                self.tail = Some(0);
                self.buffer[0] = value;
            },
            Some(h) => {
                if !(self.len() == self.capacity()){
                    self.head = Some((h + 1) % self.capacity());
                    self.buffer[self.head.unwrap()] = value;
                }
            }
        }
    }

    // Changing the pop function to not return anything
    pub fn pop(&mut self) {
        match self.tail {
            None => (),
            Some(t) => {
                if self.head == self.tail {
                    self.head = None;
                    self.tail = None;
                }
                else {
                    self.tail = Some((t + 1) % self.capacity());
                }
            }
        }
    }

    pub fn get_read_index(&self) -> usize {
        self.tail.unwrap_or(0)
    }

    pub fn set_read_index(&mut self, index: usize) {
        self.tail = Some(index % self.capacity())
    }

    pub fn get_write_index(&self) -> usize {
        self.head.unwrap_or(0)
    }

    
    pub fn set_write_index(&mut self, index: usize) {
        self.head = Some(index % self.capacity())
    }

    pub fn len(&self) -> usize {
        // Return number of values currently in the ring buffer.
        match (self.head, self.tail) {
            (Some(h), Some(t)) => {
                if h >= t {
                    h - t + 1
                } else {
                    h + self.capacity() - t + 1
                }
            },
            (_, _) => 0
        }
    }

    pub fn capacity(&self) -> usize {
        // Return the size of the internal buffer.
        self.capacity
    }

    pub fn get_sample(&mut self) -> f32{
        let temp_len = self.len();
        if temp_len > 0 {
            let mut value: f32 = 0.0;
            for index in 0..temp_len {
                let multi_osc = self.get_mutable(index).unwrap();
                value += multi_osc.get_sample();
            }
            value
        } else {
            0.0
        }
    }
}

impl Iterator for PolyphonyRingBuffer {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.get_sample())
    }
}

impl Source for PolyphonyRingBuffer {
    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}