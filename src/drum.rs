use rand::prelude::*;

// Implemented from Digital Synthesis of Plucked-String and Drum Timbres
// JSTOR URL: http://www.jstor.org/stable/3680062

pub fn wavetable_entry(wavetable: &mut Vec<f32>, clock: usize) -> f32 {
    let sample_clock = (clock % wavetable.len()) as usize;
    let this_step = wavetable[sample_clock];
    let next_step = match wavetable.get(sample_clock + 1) {
        Some(n) => n,
        None => &wavetable[0],
    };
    let mut avg = (this_step + next_step) / 2.;

    let threshold = 0.5;
    let mut random_generator = rand::thread_rng();
    let random_number: f32 = random_generator.gen();
    let negative: bool = random_number > threshold;
    if negative {
        avg *= -1.;
    }

    wavetable[sample_clock] = avg;
    avg
}

pub fn wavetable(period: usize, amplitude: f32) -> Vec<f32> {
    (0..period).map(|_| amplitude).collect()
}
