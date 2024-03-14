//! ADSR: Attack Decay Sustain Release
//! 
//! ## Parameters
//! 1. `table`: For the envelope table
//! 2. `sample_rate`: Sample Rate
//! 3. `attack`: Value in milliseconds
//! 4. `decay`: Value in milliseconds
//! 5. `sustain`: Value in percentage (0-100)
//! 6. `release`: Value in milliseconds
pub struct Envelope {
    table: Vec<f32>,
    table_size: usize,
    sample_rate: u32,
    attack: u16,
    decay: u16,
    sustain: f32,
    release: u16,
}

impl Envelope {
    pub fn new(
        sample_rate: u32,
        table_size: usize,
        attack: u16, 
        decay: u16,
        sustain: f32,
        release: u16
    ) -> Self 
    {
        assert!(sustain>=0.0 && sustain<=100.0, "Sustain must be a percentage, hence must be between 0 and 100");
        let table = create_table(attack, decay, sustain, release);
        Self {
            table,
            table_size,
            sample_rate,
            attack,
            decay,
            sustain,
            release
        }
    }

    fn create_table(
        attack: u16, 
        decay: u16,
        sustain: f32,
        release: u16
    ) -> Vec<f32>
    {
        todo!("Implement")
    }
}