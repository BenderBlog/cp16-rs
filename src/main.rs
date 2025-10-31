use crate::sine_wave_generator::{SineWaveGenerator, SAMPLE_RATE};
use hound;

mod sine_wave_generator;

fn main() {
    let mut freqs: Vec<SineWaveGenerator> = std::iter::successors(
        Some(700),
        |&n| Some(n + 150)
    )
        .take(16)
        .collect::<Vec<_>>()
        .into_iter()
        .map(|num| { SineWaveGenerator::new(num) })
        .collect();

    let font = include_bytes!("../VonwaonBitmap-16px.ttf") as &[u8];
    let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();

    let chars = "这里是BI9CKG发送的测试信号";
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("cp16.wav", spec).unwrap();

    for char in chars.chars() {
        println!("{}", char);
        let (metrics, bitmap) = font.rasterize(char, 16.0);

        // Each char about 1s
        for line_index in 0..metrics.height {
            let start_index = line_index * metrics.width;
            let end_index = start_index + metrics.width;

            let row_slice = &bitmap[start_index..end_index];

            let mut padded_row = Vec::from(row_slice);
            padded_row.resize(16,0);

            //println!("{:?}", row_slice);

            for _ in 0..SAMPLE_RATE / 16 {
                let mut sum_up: i32 = 0;
                for i in 0..16 {
                    let is_blank = if padded_row[i] > 0 { 1 } else { 0 };
                    let from_generator = freqs[i].next().unwrap();
                    sum_up = sum_up + (from_generator * is_blank) as i32;
                }
                writer.write_sample((sum_up / 16) as i16).unwrap();
            }
            // 空行
            for _ in 0..SAMPLE_RATE / 16 {
                writer.write_sample(0).unwrap();
            }

        }
        for _ in 0..SAMPLE_RATE / 8 {
            writer.write_sample(0).unwrap();
        }
    }
}
