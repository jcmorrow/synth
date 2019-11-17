mod drum;
mod instrument;
mod sin;

extern crate anyhow;
extern crate cpal;
extern crate rand;

use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use instrument::Instrument;

fn main() -> Result<(), anyhow::Error> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let format = device.default_output_format().unwrap();

    // Begin code that I modified from beep.rs
    let mut clock = 0;
    let sample_rate = format.sample_rate.0 as f32;
    let amplitude = 5.;
    let wavetable_len = 100;

    println!("Format: {:?}", format);
    println!("Sample Rate: {:?}", sample_rate);
    println!("Wavetable Length: {:?}", wavetable_len);

    let mut instruments: Vec<Instrument> = Vec::new();

    instruments.push(Instrument::new(200));
    instruments.push(Instrument::new(100));
    instruments.push(Instrument::new(50));

    let mut next_value = || {
        instruments
            .iter_mut()
            .map(|i| i.next_frequency())
            .collect::<Vec<f32>>()
            .iter()
            .sum()
    };
    // end code that I modified from beep.rs

    let event_loop = host.event_loop();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id.clone()).unwrap();

    event_loop.run(move |id, result| {
        let data = match result {
            Ok(data) => data,
            Err(err) => {
                eprintln!("an error occurred on stream {:?}: {}", id, err);
                return;
            }
        };

        match data {
            cpal::StreamData::Output {
                buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer),
            } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let next: f32 = next_value();
                    let value = ((next * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            }
            cpal::StreamData::Output {
                buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer),
            } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = (next_value() * std::i16::MAX as f32) as i16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            }
            cpal::StreamData::Output {
                buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
            } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = next_value() * 0.5;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            }
            _ => (),
        }
    });
}
