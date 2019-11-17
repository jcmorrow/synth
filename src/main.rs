mod drum;
mod instrument;
mod sin;

extern crate anyhow;
extern crate cpal;
extern crate rand;
extern crate termion;

use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use instrument::Instrument;
use std::io::{stdin, stdout, Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use termion::async_stdin;
use termion::event::Key::*;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() -> Result<(), anyhow::Error> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let format = device.default_output_format().unwrap();
    let sample_rate = format.sample_rate.0 as f32;
    println!("Format: {:?}", format);
    println!("Sample Rate: {:?}", sample_rate);

    let instruments: Vec<Instrument> = Vec::new();
    let ref_to_instruments = Arc::new(Mutex::new(instruments));
    let local_instruments = Arc::clone(&ref_to_instruments);

    // start the player in a different thread
    thread::spawn(move || {
        let local_instruments = Arc::clone(&ref_to_instruments);
        let next_value = || {
            local_instruments
                .lock()
                .unwrap()
                .iter_mut()
                .map(|i| i.next_frequency())
                .collect::<Vec<f32>>()
                .iter()
                .sum()
        };

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
    });

    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let step = |i: i32| -> f32 { (1.059463 as f32).powi(i) };
    loop {
        write!(stdout, "{}", termion::clear::CurrentLine).unwrap();
        if let Some(Ok(next)) = stdin.next() {
            match next as char {
                'a' => {
                    let mut instruments = local_instruments.lock().unwrap();
                    instruments.push(Instrument::new(220));
                }
                's' => {
                    let mut instruments = local_instruments.lock().unwrap();
                    instruments.push(Instrument::new((220. / step(2)) as i32))
                }
                'd' => {
                    let mut instruments = local_instruments.lock().unwrap();
                    instruments.push(Instrument::new((220. / step(4)) as i32))
                }
                'f' => {
                    let mut instruments = local_instruments.lock().unwrap();
                    instruments.push(Instrument::new((220. / step(5)) as i32))
                }
                'g' => {
                    let mut instruments = local_instruments.lock().unwrap();
                    instruments.push(Instrument::new((220. / step(7)) as i32))
                }
                'h' => {
                    let mut instruments = local_instruments.lock().unwrap();
                    instruments.push(Instrument::new((220. / step(9)) as i32))
                }
                'j' => {
                    let mut instruments = local_instruments.lock().unwrap();
                    instruments.push(Instrument::new((220. / step(10)) as i32))
                }
                'q' => break,
                _ => (),
            }
        }
        stdout.flush()?;
    }

    Ok(())
}
