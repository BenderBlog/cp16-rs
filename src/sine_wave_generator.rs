pub struct SineWaveGenerator {
    phase_per_sample: f64,
    phase: f64,
}

impl SineWaveGenerator {
    pub fn new(frequency: u32, sample_rate: f64) -> Self {
        Self {
            phase_per_sample: 2.0 * core::f64::consts::PI * frequency as f64 / sample_rate,
            phase: 0.0,
        }
    }
}

impl Iterator for SineWaveGenerator {
    type Item = i16;
    fn next(&mut self) -> Option<i16> {
        let a = f64::sin(self.phase);
        self.phase += self.phase_per_sample;
        Some((a * i16::MAX as f64) as i16)
    }
}
