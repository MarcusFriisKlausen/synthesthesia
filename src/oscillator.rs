use std::time::Duration;
use rodio::source::Source;

pub const WAVETABLE_SIZE: usize = 64;
pub const SAMPLE_RATE: u32 = 44100;

/// A struct that holds the sample rate and single cycle waveform. 
/// Has an index and increment size for oscillating over the waveform.
pub struct Oscillator {
    sample_rate: u32,
    waveform: Vec<f32>,
    index: f32,
    increment_size: f32
}

impl Iterator for Oscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.get_sample())
    }
}

impl Source for Oscillator {
    fn current_frame_len(&self) -> Option<usize> {
        return None
    }

    fn channels(&self) -> u16 {
        return 1
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        return None
    }
}

impl Oscillator {
    pub fn new(sample_rate: u32, waveform: Vec<f32>) -> Oscillator {
        return Oscillator {
            sample_rate: sample_rate,
            waveform: waveform,
            index: 0.0,
            increment_size: 0.0,
        }
    }

    /// Sets the frequency of the wave for the waveform.
    pub fn set_frequency(&mut self, frequency: f32) {
        self.increment_size = frequency * self.waveform.len() as f32 / self.sample_rate as f32;
    }

    /// Gets the sample of the waveform at the current index and increments the index.
    fn get_sample(&mut self) -> f32 {
        let sample: f32 = self.lerp();
        self.index += self.increment_size;
        self.index %= self.waveform.len() as f32;
        return sample
    }

    /// Function for linear interpolation.
    /// Returns a weighted sum of the 2 nearest waveform values.
    /// Used for looking up the current sample in [get_sample](Oscillator::get_sample()).
    fn lerp(&self) -> f32 {
        let truncated_index: usize = self.index as usize;
        let next_index: usize = (truncated_index + 1) % self.waveform.len();

        let next_index_weight: f32 = self.index - truncated_index as f32;
        let truncated_index_weight: f32 = 1.0 - next_index_weight;

        return truncated_index_weight * self.waveform[truncated_index] 
                + next_index_weight * self.waveform[next_index]
    }
}
