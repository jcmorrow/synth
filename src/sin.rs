pub fn wavetable_entry(wavetable: &mut Vec<f32>, clock: usize) -> f32 {
    let sample_clock = (clock % wavetable.len()) as usize;
    wavetable[sample_clock]
}

pub fn wavetable(period: usize, amplitude: f32) -> Vec<f32> {
    // A random fill works because of the way that the wave decays
    (0..period)
        .map(|sample_clock| {
            (sample_clock as f32 * 440.0 * 2.0 * std::f32::consts::PI / period as f32).sin()
                * amplitude
        })
        .collect::<Vec<f32>>()
}
