use crate::sine_wave_generator::{SineWaveGenerator, SAMPLE_RATE};
use hound;

mod sine_wave_generator;

fn main() {
    let mut freqs: Vec<SineWaveGenerator> = std::iter::successors(
        Some(300), // 初始值
        |&n| Some(n + 400) // 闭包：传入当前值 n，返回下一个值 n + step
    )
        .take(16) // 取前 16 个数
        .collect::<Vec<_>>()
        .into_iter()
        .map(|num| { SineWaveGenerator::new(num) })
        .collect();

    let output_graph : [[bool; 16]; 10] = [
        [
            true, true, true, false, false, true, false, true, true, true, false, false, false, false, false, false
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false
        ],
        [
            true, false, false, true, false, true, false, true, false, true, false, false, false, false, false, false
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false
        ],
        [
            true, true, true, false, false, true, false, true, true,true, false, false, false, false, false, false
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false
        ],
        [
            true, false, false, true, false, true, false, false, false, true, false, false, false, false, false, false
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false
        ],
        [
            true, true, true, false, false, true, false, true, true,true, false, false, false, false, false, false
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false
        ],
    ];

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("cp16.wav", spec).unwrap();

    // Each line about 1s
    for line in output_graph {
        for _ in 0..SAMPLE_RATE / 8 {
            let mut sum_up: i32 = 0;
            for i in 0..16 {
                let is_blank = if line[i] { 1 } else { 0 };
                let from_generator = freqs[i].next().unwrap();
                sum_up = sum_up + (from_generator * is_blank) as i32;
            }
            writer.write_sample((sum_up / 16) as i16).unwrap();
        }
    }
}
