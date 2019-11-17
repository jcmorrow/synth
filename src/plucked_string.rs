use rand::prelude::*;

// Implemented from Digital Synthesis of Plucked-String and Drum Timbres
// JSTOR URL: http://www.jstor.org/stable/3680062

pub fn wavetable_entry(wavetable: &mut Vec<f32>, clock: usize) -> f32 {
    // Y[t] = 1/2 * (Y[t-p] + Y[t-p-1])
    // Basically, assign to this step the average of this step and the next
    let sample_clock = (clock % wavetable.len()) as usize;
    let this_step = wavetable[sample_clock];
    let next_step = match wavetable.get(sample_clock + 1) {
        Some(n) => n,
        None => &wavetable[0],
    };
    let avg = (this_step + next_step) / 2.;
    wavetable[sample_clock] = avg;
    avg
}

pub fn wavetable(period: usize, amplitude: f32) -> Vec<f32> {
    // A random fill works because of the way that the wave decays
    let mut rng = rand::thread_rng();
    (0..period)
        .map(|_| if rand::random() { 1. } else { -1. })
        .collect::<Vec<f32>>()
        .iter()
        .map(|_| rng.gen())
        .collect::<Vec<f32>>()
        .iter()
        .map(|t| t * amplitude)
        .collect()
}
