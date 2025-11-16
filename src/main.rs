use crate::cp_16_generator::CP16Generator;
use hound;

mod cp_16_generator;
mod sine_wave_generator;

fn main() {
    let sample_rate = 8000;
    let chars = "Ｂ３／Ｂ１ＣＲＡ－ＴＥＳＴ１５　如果，我能够飞到了蓝天，我想我能远离尘世的喧嚣。abcdefghijklmnopqrstuvwxyz";
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("cp16-new.wav", spec).unwrap();

    let iter = CP16Generator::new(chars, 1600, 15, sample_rate, true, 1.0).unwrap();

    for i in iter {
        writer.write_sample(i).unwrap();
    }
}
