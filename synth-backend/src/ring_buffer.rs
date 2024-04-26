//! Ring buffer module providing data structures for handling polyphonic MIDI maps.
//!
//! The `IterablePolyphonyHashMap` struct allows handling multiple notes through a single source, which is a HashMap
//! containing MIDI keys mapped to synthesizer instances. This struct implements the `Source` trait from the `rodio` crate,
//! making it compatible with audio playback.
//!
//! ## Examples
//!
//! ```
//! use synth_backend::ring_buffer::IterablePolyphonyHashMap;
//! use synth_backend::wrapper::Synth;
//!
//! // Create a new IterablePolyphonyHashMap with a sample rate of 44100 Hz
//! let mut polyphony_map = IterablePolyphonyHashMap::new(44100);
//!
//! // Insert a synthesizer for MIDI key 60
//! let synth = Synth::new_default();
//! polyphony_map.insert(60, synth);
//!
//! // Generate audio samples
//! let sample = polyphony_map.get_sample();
//! ```
//!
//! The `IterablePolyphonyHashMap` struct provides methods for inserting, removing, and retrieving synthesizers based on MIDI keys,
//! as well as generating audio samples from the entire polyphonic map.
use crate::oscillators::MultiOscillator;
use crate::wrapper::Synth;
use rodio::Source;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct RingBuffer<T> {
    buffer: Vec<T>,
    head: usize,
    tail: usize,
}

impl<T: Copy + Default> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        RingBuffer {
            buffer: vec![T::default(); capacity],
            head: 0,
            tail: 0,
        }
    }

    pub fn reset(&mut self) {
        self.buffer.fill(T::default());
        self.head = 0;
        self.tail = 0;
    }

    // `put` and `peek` write/read without advancing the indices.
    pub fn put(&mut self, value: T) {
        self.buffer[self.head] = value
    }

    pub fn peek(&self) -> T {
        self.buffer[self.tail]
    }

    pub fn get(&self, offset: usize) -> T {
        self.buffer[(self.tail + offset) % self.capacity()]
    }

    // `push` and `pop` write/read and advance the indices.
    pub fn push(&mut self, value: T) {
        self.buffer[self.head] = value;
        self.head = (self.head + 1) % self.capacity();
    }

    pub fn pop(&mut self) -> T {
        let value = self.buffer[self.tail];
        self.tail = (self.tail + 1) % self.capacity();
        value
    }

    pub fn get_read_index(&self) -> usize {
        self.tail
    }

    pub fn set_read_index(&mut self, index: usize) {
        self.tail = index % self.capacity()
    }

    pub fn get_write_index(&self) -> usize {
        self.head
    }

    pub fn set_write_index(&mut self, index: usize) {
        self.head = index % self.capacity()
    }

    pub fn len(&self) -> usize {
        // Return number of values currently in the ring buffer.
        if self.head >= self.tail {
            self.head - self.tail
        } else {
            self.head + self.capacity() - self.tail
        }
    }

    pub fn capacity(&self) -> usize {
        // Return the size of the internal buffer.
        self.buffer.len()
    }
}

impl RingBuffer<f32> {
    // Return the value at at an offset from the current read index.
    // To handle fractional offsets, linearly interpolate between adjacent values. 
    pub fn get_frac(&self, offset: f32) -> f32 {
        let index_floor = offset.floor() as usize;
        let index_ceil = offset.ceil() as usize;
        let index_fract = offset.fract();

        self.get(index_floor) * (1.0 - index_fract) + self.get(index_ceil) * index_fract
    }
}


/// Struct representing an iterable polyphonic MIDI map.
#[derive(Clone, Debug)]
pub struct IterablePolyphonyHashMap {
    hashmap: HashMap<u8, Synth>,
    sample_rate: u32
}

impl IterablePolyphonyHashMap {
    /// Creates a new `IterablePolyphonyHashMap` with the given sample rate.
    pub fn new(sample_rate: u32) -> Self{
        Self {
            hashmap: HashMap::new(),
            sample_rate
        }
    }

    /// Creates a new `IterablePolyphonyHashMap` from an existing HashMap.
    pub fn from(hashmap: HashMap<u8, Synth>) -> Self {
        if hashmap.is_empty() {
            panic!("Empty Hashmap! Use new() instead");
        }
        let mut sample_rate = 0;
        for (_, synth) in hashmap.iter() {
            sample_rate = synth.osc.sample_rate();
        }
        Self {
            hashmap,
            sample_rate
        }
    }

    /// Inserts a synthesizer into the MIDI map with the given MIDI key.
    pub fn insert(&mut self, k: u8, v: Synth){
        self.hashmap.insert(k, v);
    }

    /// Removes a synthesizer from the MIDI map based on the given MIDI key.
    pub fn remove(&mut self, k:&u8) -> Option<Synth> {
        self.hashmap.remove(k)
    }

    /// Clears all synthesizers from the MIDI map.
    pub fn clear(&mut self) {
        self.hashmap.clear()
    }

    pub fn len(&self) -> usize {
        self.hashmap.len()
    }

    /// Checks if the MIDI map is empty.
    pub fn is_empty(&self) -> bool {
        self.hashmap.is_empty()
    }

    /// Retrieves a synthesizer from the MIDI map based on the given MIDI key.
    pub fn get(&self, k: &u8) -> Option<&Synth> {
        self.hashmap.get(k)
    }

    /// Generates audio samples from all synthesizers in the MIDI map.
    pub fn get_sample(&mut self) -> f32 {
        let mut sample = 0.0;
        for (_, synth) in self.hashmap.iter_mut() {
            sample += synth.get_sample();
        }
        sample
    }
}

impl Iterator for IterablePolyphonyHashMap {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.get_sample())
    }
}

impl Source for IterablePolyphonyHashMap {
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