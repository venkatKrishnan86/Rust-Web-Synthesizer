#[derive(Clone)]
pub struct RingBuffer<T> {
    buffer: Vec<T>,
    capacity: usize,
    head: Option<usize>,
    tail: Option<usize>,
}

impl<T: Copy + Default> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        RingBuffer {
            buffer: vec![T::default(); capacity],
            capacity: capacity,
            head: None,
            tail: None,
        }
    }

    pub fn reset(&mut self) {
        self.buffer.fill(T::default());
        self.head = None;
        self.tail = None;
    }

    // `put` and `peek` write/read without advancing the indices.
    pub fn put(&mut self, value: T) {
        match self.head {
            None => {
                self.head = Some(0);
                self.tail = Some(0);
                self.buffer[0] = value;
            },
            Some(h) => self.buffer[h] = value
        }
    }

    pub fn peek(&self) -> Option<T> {
        match self.tail {
            None => None,
            Some(t) => Some(self.buffer[t])
        }
    }

    pub fn get(&self, offset: usize) -> T {
        match self.tail {
            None => T::default(),
            Some(t) => self.buffer[(t + offset) % self.capacity()]
        }
    }

    /// Push a value into the RingBuffer.
    ///
    /// This will push the value and also advance the head index to the next position
    ///
    /// Note: If the Ring Buffer is full, it will not do anything!
    ///
    /// ## Arguments
    ///
    /// `value`: The value to be pushed
    ///
    /// ## Returns
    ///
    /// `()`
    ///
    /// ```
    /// use crate::ring_buffer::RingBuffer;
    ///
    /// let mut rb = RingBuffer::<i16>::new(2);
    /// rb.push(0);
    /// rb.push(2);
    /// assert_eq!(vec![0, 2], rb.buffer);
    ///
    /// rb.push(-3);
    /// assert_eq!(vec![-3, 2], rb.buffer);
    /// ```
    pub fn push(&mut self, value: T) {
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

    /// Pop the value from the RingBuffer.
    ///
    /// This will advance the tail index to the next position.
    ///
    /// ## Arguments
    ///
    /// No Arguments
    ///
    /// ## Returns
    ///
    /// `Option<T>`: Return the popped value wrapped in an `Option`. If the Ring Buffer is empty, it will return `None`
    ///
    /// ```
    /// use crate::ring_buffer::RingBuffer;
    ///
    /// let mut rb = RingBuffer::<i16>::new(2);
    /// rb.push(0);
    /// rb.push(2);
    /// assert_eq!(vec![0, 2], rb.buffer);
    /// assert_eq!(2, rb.len());
    /// assert_eq!(Some(0), rb.pop());
    /// assert_eq!(Some(2), rb.pop());
    /// assert_eq!(None, rb.pop());
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        match self.tail {
            None => None,
            Some(t) => {
                let value = self.buffer[t];
                if self.head == self.tail {
                    self.head = None;
                    self.tail = None;
                }
                else {
                    self.tail = Some((t + 1) % self.capacity());
                }
                Some(value)
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

    /// Obtain the number of values used in the RingBuffer
    ///
    /// ```
    /// use crate::ring_buffer::RingBuffer;
    ///
    /// let mut rb = RingBuffer::<f32>::new(5);
    /// rb.push(0.0);
    /// rb.push(2.12);
    /// assert_eq!(2, rb.len());
    /// ```
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

    /// Obtain the maximum capacity of the RingBuffer
    ///
    /// NOTE: It is NOT the same as len(), which returns the current number of elements used up in the RingBuffer
    ///
    /// ```
    /// use crate::ring_buffer::RingBuffer;
    ///
    /// let rb = RingBuffer::<f32>::new(5);
    /// assert_eq!(5, rb.capacity());
    /// ```
    pub fn capacity(&self) -> usize {
        // Return the size of the internal buffer.
        self.capacity
    }
}

impl RingBuffer<f32> {
    // Return the value at at an offset from the current read index.
    // To handle fractional offsets, linearly interpolate between adjacent values. 
    pub fn get_frac(&self, offset: f32) -> f32 {
        assert!(offset>=0.0, "Offset must be greater than or equal to 0");
        if offset >= self.len() as f32 - 1.0 { return f32::default() }
        match self.tail {
            None => f32::default(),
            Some(t) => (1.0-(offset - offset.trunc())) * self.buffer[(t+offset.trunc() as usize)%self.capacity()] 
                + (offset - offset.trunc()) * self.buffer[(t+offset.trunc() as usize + 1)%self.capacity()]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapping() {
        // Test that ring buffer is a ring (wraps after more than `length` elements have entered).
        let capacity = 17;
        let delay = 5;
        let mut ring_buffer: RingBuffer<f32> = RingBuffer::new(capacity);

        for i in 0..delay {
            ring_buffer.push(i as f32);
        }

        for i in delay..capacity + 13 {
            assert_eq!(ring_buffer.len(), delay);
            assert_eq!(ring_buffer.pop(), Some((i - delay) as f32));
            ring_buffer.push(i as f32)
        }
    }

    #[test]
    fn test_api() {
        // Basic test of all API functions.
        let capacity = 3;
        let mut ring_buffer = RingBuffer::new(capacity);
        assert_eq!(ring_buffer.capacity(), capacity);

        ring_buffer.put(3);
        assert_eq!(ring_buffer.peek(), Some(3));

        ring_buffer.set_write_index(0);
        assert_eq!(ring_buffer.get_write_index(), 0);

        ring_buffer.push(17);
        assert_eq!(ring_buffer.get_write_index(), 1);

        assert_eq!(ring_buffer.get_read_index(), 0);
        assert_eq!(ring_buffer.get(1), 17);
        assert_eq!(ring_buffer.pop().unwrap(), 3);
        assert_eq!(ring_buffer.get_read_index(), 1);

        assert_eq!(ring_buffer.len(), 1);
        ring_buffer.push(42);
        assert_eq!(ring_buffer.len(), 2);

        assert_eq!(ring_buffer.get_write_index(), 2);

        // Should be unchanged.
        assert_eq!(ring_buffer.capacity(), capacity);
    }

    #[test]
    fn test_capacity() {
        // Tricky: does `capacity` mean "size of internal buffer" or "number of elements before this is full"?
        let capacity = 3;
        let mut ring_buffer = RingBuffer::new(capacity);
        for i in 0..(capacity - 1) {
            ring_buffer.push(i);
            dbg!(ring_buffer.len());
            assert_eq!(ring_buffer.len(), i+1);
        }
    }

    #[test]
    fn test_len_function() {
        let capacity = 10;
        let mut ring_buffer = RingBuffer::new(capacity);
        for i in 0..capacity {
            ring_buffer.push(i);
            dbg!(ring_buffer.len());
            assert_eq!(ring_buffer.len(), i+1);
        }
    }

    #[test]
    fn test_reset() {
        // Test state after initialization and reset.
        let mut ring_buffer = RingBuffer::new(512);

        // Check initial state.
        assert_eq!(ring_buffer.get_read_index(), 0);
        assert_eq!(ring_buffer.get_write_index(), 0);
        for i in 0..ring_buffer.capacity() {
            assert_eq!(ring_buffer.get(i), 0.0);
        }

        // Fill ring buffer, mess with indices.
        let fill = 123.456;
        for i in 0..ring_buffer.capacity() {
            ring_buffer.push(fill);
            assert_eq!(ring_buffer.get(i), fill);
        }

        ring_buffer.set_write_index(17);
        ring_buffer.set_read_index(42);

        // Check state after reset.
        ring_buffer.reset();
        assert_eq!(ring_buffer.get_read_index(), 0);
        assert_eq!(ring_buffer.get_write_index(), 0);
        for i in 0..ring_buffer.capacity() {
            assert_eq!(ring_buffer.get(i), 0.0);
        }
    }

    #[test]
    fn test_weird_inputs() {
        let capacity = 5;
        let mut ring_buffer = RingBuffer::<f32>::new(capacity);

        ring_buffer.set_write_index(capacity);
        assert_eq!(ring_buffer.get_write_index(), 0);
        ring_buffer.set_write_index(capacity * 2 + 3);
        assert_eq!(ring_buffer.get_write_index(), 3);

        ring_buffer.set_read_index(capacity);
        assert_eq!(ring_buffer.get_read_index(), 0);
        ring_buffer.set_read_index(capacity * 2 + 3);
        assert_eq!(ring_buffer.get_read_index(), 3);

        // NOTE: Negative indices are also weird, but we can't even pass them due to type checking!
    }

    mod pop_tests {
        use super::*;

        #[test]
        fn test_1() {
            let mut rb = RingBuffer::<i16>::new(2);
            rb.push(0);
            rb.push(2);
            assert_eq!(vec![0, 2], rb.buffer);
            assert_eq!(Some(0), rb.pop());
            assert_eq!(Some(2), rb.pop());
            assert_eq!(None, rb.pop());
        }
    }

    mod get_frac_tests {
        use super::*;

        #[test]
        fn test_1() {
            let mut buffer = RingBuffer::<f32>::new(4);
            for i in 1..4 {
                buffer.push(i as f32);
            }
            assert_eq!(1.5, buffer.get_frac(0.5));
        }

        #[test]
        fn test_2() {
            let mut buffer = RingBuffer::<f32>::new(4);
            for i in 1..4 {
                buffer.push(i as f32);
            }
            assert_eq!(2.8, buffer.get_frac(1.8));
        }

        #[test]
        #[should_panic]
        fn test_3() {
            let mut buffer = RingBuffer::<f32>::new(3);
            for i in 1..4 {
                buffer.push(i as f32);
            }
            let _ = buffer.get_frac(-0.2);
        }

        #[test]
        fn test_4() {
            let mut buffer = RingBuffer::<f32>::new(3);
            for i in 1..4 {
                buffer.push(i as f32);
            }
            assert_eq!(0.0, buffer.get_frac(2.1));
        }
    }

    mod push_tests {
        use super::*;

        #[test]
        fn test_1() {
            let mut buffer = RingBuffer::<i16>::new(3);
            for i in 1..5 {
                buffer.push(i);
            }
            
            assert_eq!(vec![1,2,3], buffer.buffer);
        } 
    }
}