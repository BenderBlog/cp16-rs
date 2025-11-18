use clap::Parser;
use cp16::cp_16_generator::CP16Generator;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleRate, StreamConfig};
use std::error::Error;
use std::path::PathBuf;
use std::sync::{Arc, Condvar, Mutex};

#[derive(Parser, Debug)]
#[command(long_about = None)]
struct Args {
    /// Sample Rate
    #[arg(long, default_value_t = 8000)]
    sample_rate: u32,

    /// Start Frequency
    #[arg(long)]
    start_freq: u32,

    /// Frequncy Step
    #[arg(long)]
    step: u32,

    /// Disable horizontal character display on waterfall plot
    #[arg(long, default_value_t = false)]
    disable_vertical: bool,

    /// Display time for a full-width character
    #[arg(long, default_value_t = 1.5)]
    time_per_font: f64,

    #[arg(long)]
    path: Option<PathBuf>,

    /// String to be transfered to CP-16
    text: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("Will output {}", args.text);

    let iter = match CP16Generator::new(
        args.text,
        args.start_freq,
        args.step,
        args.sample_rate,
        !args.disable_vertical,
        args.time_per_font,
    ) {
        Ok(d) => d,
        Err(_) => {
            eprintln!(
                "Provided start freq and step exceed the maximum frequency limit specified by sample rate, program will exit."
            );
            return Ok(());
        }
    };

    match args.path {
        Some(path) => {
            println!("Will write the file to {:?}", path);
            let spec = hound::WavSpec {
                channels: 1,
                sample_rate: args.sample_rate,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            };
            let mut writer = hound::WavWriter::create(path, spec).unwrap();

            for s in iter {
                match writer.write_sample(s) {
                    Ok(_) => {}
                    Err(e) => eprintln!("An error occurred on writing file: {}", e),
                };
            }
            println!("Finished writing.");
            return Ok(());
        }
        None => {
            println!("Path not provided, program will play in the default audio output device.")
        }
    }

    let host = cpal::default_host();
    let device = match host.default_output_device() {
        Some(d) => d,
        None => {
            eprintln!("Default output device not found, program will exit.");
            return Ok(());
        }
    };

    let channels = 1;

    let config = StreamConfig {
        channels,
        sample_rate: SampleRate(args.sample_rate),
        buffer_size: cpal::BufferSize::Default,
    };

    let generator_mutex = Arc::new(Mutex::new(iter));

    let finished_mutex = Arc::new(Mutex::new(false));
    let finished_condvar = Arc::new(Condvar::new());

    let current_generator = generator_mutex.clone();
    let callback_finished_mutex = finished_mutex.clone();
    let callback_finished_condvar = finished_condvar.clone();

    let data_callback = move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
        let mut generator_lock = match current_generator.lock() {
            Ok(g) => g,
            Err(_) => {
                data.fill(0);
                return;
            }
        };

        let mut finished_lock = callback_finished_mutex.lock().unwrap();
        if *finished_lock {
            data.fill(0);
            return;
        }

        let mut samples_generated = 0;
        let mut ended_in_this_call = false;

        for sample in data.iter_mut() {
            if let Some(value) = generator_lock.next() {
                *sample = value;
                samples_generated += 1;
            } else {
                *sample = 0;
                ended_in_this_call = true;
            }
        }

        if ended_in_this_call {
            *finished_lock = true;
            callback_finished_condvar.notify_one();
            for sample in data.iter_mut().skip(samples_generated) {
                *sample = 0;
            }
        }
    };

    let err_callback = |err| eprintln!("An error occurred on stream: {}", err);
    let stream = device.build_output_stream(&config, data_callback, err_callback, None)?;
    stream.play()?;

    println!("Outputting...");
    let mut guard = finished_mutex.lock().unwrap();
    while !*guard {
        guard = finished_condvar.wait(guard).unwrap();
    }
    println!("Finished.");

    Ok(())
}
