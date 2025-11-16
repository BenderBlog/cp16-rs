use unifont::{Glyph, get_glyph};

use crate::sine_wave_generator::SineWaveGenerator;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct OutOfMaxFrequencyException;

impl Display for OutOfMaxFrequencyException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "provided start_freq and step will exceed the maximum frequency limit specified by sample_rate"
            .fmt(f)
    }
}

impl Error for OutOfMaxFrequencyException {}

pub struct CP16Generator {
    freqs: Vec<SineWaveGenerator>,
    text_glyph: Vec<&'static Glyph>,
    char_pos: usize,
    start_padding: usize,
    y: usize,
    glyph: &'static Glyph,
    samples_per_line: u32,
    current_sample: u32,
    is_horizontal: bool,
}

impl CP16Generator {
    pub fn new(
        text: &'static str,
        start_freq: u32,
        step: u32,
        sample_rate: u32,
        is_horizontal: bool,
        time_per_font: f64,
    ) -> Result<Self, OutOfMaxFrequencyException> {
        let max_freq = sample_rate >> 1;
        let range = std::iter::successors(Some(start_freq), |&n| Some(n + step))
            .take(16)
            .collect::<Vec<_>>();

        if range.iter().find(|&&x| x > max_freq).is_some() {
            return Err(OutOfMaxFrequencyException);
        }

        let freqs: Vec<SineWaveGenerator> = range
            .into_iter()
            .map(|freq| SineWaveGenerator::new(freq, sample_rate as f64))
            .collect();

        let text_glyph: Vec<&Glyph> = text.chars().filter_map(|x| match get_glyph(x) {
            Some(n) => Some(n),
            None => {
                println!(
                    "Unifont does not contains {}, will be replased with a full length question mark", x
                );
                get_glyph('？')
            }
        }).collect();

        let first = text_glyph[0];

        let samples_per_line = (sample_rate as f64 / 16.0 * time_per_font) as u32;

        Ok(Self {
            freqs,
            text_glyph,
            char_pos: 0,
            glyph: first,
            start_padding: if is_horizontal || first.is_fullwidth() {
                0
            } else {
                4
            },
            y: 0,
            samples_per_line,
            current_sample: 0,
            is_horizontal,
        })
    }
}

impl Iterator for CP16Generator {
    type Item = i16;
    fn next(&mut self) -> Option<i16> {
        if self.y
            >= if self.is_horizontal {
                self.glyph.get_width() + self.start_padding + 2
            } else {
                16 + 2
            }
        {
            self.char_pos += 1;
            if self.char_pos >= self.text_glyph.len() {
                return None;
            }
            self.glyph = self.text_glyph[self.char_pos];
            self.start_padding = if self.is_horizontal || self.glyph.is_fullwidth() {
                0
            } else {
                4
            };
            self.y = 0;
        }

        if self.current_sample >= self.samples_per_line {
            self.y += 1;
            self.current_sample = 0;
        }

        let mut sum_up: i32 = 0;

        let range_end = if self.is_horizontal {
            16
        } else {
            self.glyph.get_width() + self.start_padding
        };

        if self.y
            < (if self.is_horizontal {
                self.glyph.get_width() + self.start_padding
            } else {
                16
            })
        {
            for x in self.start_padding..range_end {
                let pixel = if self.is_horizontal {
                    self.glyph.get_pixel(self.y - self.start_padding, 16 - x)
                } else {
                    self.glyph.get_pixel(x - self.start_padding, self.y)
                };
                let is_blank = if pixel { 1 } else { 0 };
                sum_up = sum_up + (self.freqs[x].next().unwrap() * is_blank) as i32;
            }
        }

        self.current_sample += 1;

        return Some((sum_up / (self.glyph.get_width() as i32 * 2)) as i16);
    }
}
