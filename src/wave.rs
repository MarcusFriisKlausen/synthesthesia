/// A struct that stores the amplitude and frequency of a wave.
pub struct Wave {
    // Amplitude
    amp: f32,
    // Frequency
    freq: f32,
}

impl Wave {
    pub fn new(amp: f32, freq: f32) -> Wave {
        return Wave {
            amp,
            freq,
        }
    }

    pub fn get_amp(&self) -> f32 {
        return self.amp;
    }

    pub fn get_freq(&self) -> f32 {
        return self.freq;
    }
}
