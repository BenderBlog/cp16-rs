// Copyright 2025 BenderBlog Rodriguez / BI9CKG
// SPDX-License-Identifier: BSD-3-Clause

use unifont::{Glyph, get_glyph};

use crate::sine_wave_generator::SineWaveGenerator;
use std::error::Error;
use std::fmt::Display;

/// 如果不符合采样定律限制，抛出该错误
#[derive(Debug)]
pub struct OutOfMaxFrequencyException;

impl Display for OutOfMaxFrequencyException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "provided start_freq and step will exceed the maximum frequency limit specified by sample_rate"
            .fmt(f)
    }
}

impl Error for OutOfMaxFrequencyException {}

/// 包括 CP-16 信息的 PCM 数字音频信号生成器
///
/// CP-16 发明者：BA1HAM
///
/// CP-16 介绍：https://www.hellocq.net/forum/read.php?tid=345437
///
/// 本生成器使用了 [unifont](https://crates.io/crates/unifont) 库。该库均为像素字体，
/// 宽 8 像素或者 16 像素，高为 16 像素。字体的左上角设定为 (0, 0)，向左为 x 轴，向下为 y 轴。
///
/// 本迭代器仅支持生成 16 位 PCM 音频。
///
/// 生成步骤大致为：
/// 1. 设置字形在瀑布图上是否需要垂直展示。如果设置了，则字形在瀑布图上会显示成类似竹简的
/// 从上到下排列；如果没有设置，则每个字形会在瀑布图上顺时针旋转 90 度。后者在横向的频谱图、
/// 展示中更可读。
/// 2. 根据是否垂直展示，决定字符上像素的读取方式。如果是垂直方式表示，则外层从 0 迭代 y，
/// 内层从 0 迭代 x。否则为外层从 0 迭代 x，内部从最后一个值迭代 y。由此读出来传入信号生成器
/// 的信息。如果长度为 8 则需要补全前四位，保证显示在中间。
/// 3. 每个信号生成器生成信号，并根据传入信息决定是否参与线性叠加。叠加后信息作为采样输出。
pub struct CP16Generator {
    /// 16 个正弦波 PCM 采样生成器
    freqs: Vec<SineWaveGenerator>,

    /// 从 `unifont` 库读出来的字形信息集合
    text_glyph: Vec<&'static Glyph>,

    /// 目前读取的字符索引
    char_pos: usize,

    /// 字形开始渲染的位置
    start_padding: usize,

    /// 结束迭代的位置
    range_end: usize,

    /// 目前读取字符行的索引
    iter_index: usize,

    /// 当前正在读取的字形
    glyph: &'static Glyph,

    /// 每行需要渲染的采样数
    samples_per_line: u32,

    /// 采样计数器
    current_sample: u32,

    /// 设置字形在瀑布图上是否需要垂直展示
    is_vertical_display_on_waterfall: bool,

    /// 该字符需迭代的行数
    line_count: usize,

    /// 是否反向迭代字符及扫描行
    is_reverse: bool,
}

impl CP16Generator {
    /// 根据字形，当前行数和是否垂直展示读取对应行/列的信息
    fn get_line(&self) -> Vec<bool> {
        let mut line_info = [
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false,
        ];

        for i in self.start_padding..self.range_end {
            let mut y = if self.is_vertical_display_on_waterfall {
                self.iter_index
            } else {
                16 - i
            };
            if self.is_reverse {
                y = 16 - y;
            }

            line_info[i] = if self.is_vertical_display_on_waterfall {
                // 如果是瀑布图模式，则迭代过程中，y 轴不变，迭代 x 轴
                self.glyph.get_pixel(i - self.start_padding, y)
            } else {
                self.glyph
                    .get_pixel(self.iter_index - self.start_padding, y)
            };
        }

        line_info.to_vec()
    }

    /// 新建一个生成器
    /// - text: 需要编码的字符串
    /// - start_freq: 起始频率
    /// - step：频率间隔
    /// - sample_rate：采样率
    /// - is_vertical_display_on_waterfall：设置字形在瀑布图上是否需要垂直展示
    /// - time_per_font：每个字的显示时间，单位是秒
    /// - is_reverse：是否从最后一个字符的最后一行数据开始迭代
    pub fn new(
        text: String,
        start_freq: u32,
        step: u32,
        sample_rate: u32,
        is_vertical_display_on_waterfall: bool,
        time_per_font: f64,
        is_reverse: bool,
    ) -> Result<Self, OutOfMaxFrequencyException> {
        // 第一步：根据起始频率和频率间隔，计算出采样生成器的频率
        let range = std::iter::successors(Some(start_freq), |&n| Some(n + step))
            .take(16)
            .collect::<Vec<_>>();

        // 第二步：验证频率是否符合香农采样定理
        let max_freq = sample_rate >> 1;
        if range.iter().find(|&&x| x > max_freq).is_some() {
            return Err(OutOfMaxFrequencyException);
        }

        // 第三步：根据频率生成采样生成器
        let freqs: Vec<SineWaveGenerator> = range
            .into_iter()
            .map(|freq| SineWaveGenerator::new(freq, sample_rate as f64))
            .collect();

        // 第四步：获得字形，如果没找到就用全角问号代替
        let get_glyph = |x| match get_glyph(x) {
            Some(n) => Some(n),
            None => {
                println!(
                    "Unifont does not contains {}, will be replased with a full length question mark",
                    x
                );
                get_glyph('？')
            }
        };
        let text_glyph: Vec<&Glyph> = if is_reverse {
            text.chars().rev().filter_map(get_glyph).collect()
        } else {
            text.chars().filter_map(get_glyph).collect()
        };

        // 第五步：根据 16 个采样生成器和每个字符显示时间，生成每行需要渲染的采样数
        let samples_per_line = (sample_rate as f64 / 16.0 * time_per_font) as u32;

        // 第六步：获得第一个字形，用于开始循环
        let first_glyph = text_glyph[0];

        // 第七步：获取第一个字形每行开始渲染的位置
        let start_padding = if is_vertical_display_on_waterfall && !first_glyph.is_fullwidth() {
            4
        } else {
            0
        };

        // 第八步：获取第一个字形结束渲染的位置
        let range_end: usize = if is_vertical_display_on_waterfall {
            first_glyph.get_width() + start_padding
        } else {
            16
        };

        // 第九步：获取预计高度
        let line_count = if is_vertical_display_on_waterfall {
            first_glyph.get_width()
        } else {
            16
        };

        // 第十步：获取第一个字形的第一行信息
        let iter_index = 0;

        Ok(Self {
            freqs,
            text_glyph,
            char_pos: 0,
            glyph: first_glyph,
            start_padding,
            range_end,
            iter_index,
            samples_per_line,
            current_sample: 0,
            is_vertical_display_on_waterfall,
            line_count,
            is_reverse,
        })
    }
}

impl Iterator for CP16Generator {
    type Item = i16;

    fn next(&mut self) -> Option<i16> {
        // 第一步：获取该行信息
        let line_info = self.get_line();

        // 第二步：开始累加
        let mut sum_up: i32 = 0;

        // 第三步：如果不是最后的空行，则生成该行信息，否则生成空行
        if self.iter_index < self.line_count {
            for x in self.start_padding..self.range_end {
                if line_info[x] {
                    sum_up = sum_up + (self.freqs[x].next().unwrap()) as i32;
                }
            }
        }

        // 第四步：计数器加一
        self.current_sample += 1;

        // 第五步：如果该行的采样生成完毕，转进到下一行
        if self.current_sample >= self.samples_per_line {
            self.iter_index += 1;
            self.current_sample = 0;
        }

        // 第六步：判断这个字符是否迭代完，即读完了每行信息并生成了一行空隙
        if self.iter_index > self.line_count {
            // 如果迭代完，迭代下一个字符
            self.char_pos += 1;

            // 如果所有字符全部迭代完毕，整体迭代完毕
            if self.char_pos >= self.text_glyph.len() {
                return None;
            }

            // 获取下一个字符
            self.glyph = self.text_glyph[self.char_pos];

            // 更新字形开始渲染的位置
            self.start_padding =
                if self.is_vertical_display_on_waterfall && !self.glyph.is_fullwidth() {
                    4
                } else {
                    0
                };

            // 决定结束的索引
            self.range_end = if self.is_vertical_display_on_waterfall {
                self.glyph.get_width() + self.start_padding
            } else {
                16
            };

            // 该字符需迭代的行数
            self.line_count = if self.is_vertical_display_on_waterfall {
                16
            } else {
                self.glyph.get_width() + self.start_padding
            };

            // 读取字符行的索引更新
            self.iter_index = 0;

            // 采样生成计数器置零
            self.current_sample = 0;
        }

        // 第七步：输出采样信息，这里除数乘以 2 表示音频电平减半
        return Some((sum_up / (self.glyph.get_width() as i32 * 2)) as i16);
    }
}
