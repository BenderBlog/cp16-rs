// Copyright 2025 BenderBlog Rodriguez / BI9CKG
// SPDX-License-Identifier: BSD-3-Clause

/// 非常简单的十六位正弦波 PCM 采样生成器
pub struct SineWaveGenerator {
    /// 两个采样之间的相位差
    ///
    /// 计算公式：$\Delta p = \frac{2 \pi f}{s}$
    ///
    /// 其中，$\Delta p$为相位差，$f$为生成频率，$s$为采样率
    phase_per_sample: f64,

    /// 当前相位
    phase: f64,
}

impl SineWaveGenerator {
    /// 生成一个采样生成器
    /// - frequency: 生成频率
    /// - sample_rate：采样率
    pub fn new(frequency: u32, sample_rate: f64) -> Self {
        Self {
            phase_per_sample: 2.0 * core::f64::consts::PI * frequency as f64 / sample_rate,
            phase: 0.0,
        }
    }
}

impl Iterator for SineWaveGenerator {
    type Item = i16;

    /// 迭代下一个采样
    fn next(&mut self) -> Option<i16> {
        let a = f64::sin(self.phase);
        self.phase += self.phase_per_sample;
        Some((a * i16::MAX as f64) as i16)
    }
}
