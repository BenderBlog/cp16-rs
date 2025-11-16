use cp16::cp_16_generator::CP16Generator;
use hound;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: cp16-cli [sample_rate] <strings>");
        return;
    }

    let sample_rate = match args[1].parse::<u32>() {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Sample rate cannot parse: {}", e);
            return;
        }
    };

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("cp16-new.wav", spec).unwrap();
    let chars = args[2..].iter().cloned().collect::<Vec<String>>().join("");
    let iter = CP16Generator::new(chars, 1600, 15, sample_rate, true, 1.0).unwrap();
    for i in iter {
        writer.write_sample(i).unwrap();
    }
}
