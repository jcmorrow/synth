use rand::prelude::*;

pub struct Instrument {
    wavetable: Vec<f32>,
    clock: i32,
}

impl Instrument {
    pub fn new(period: i32) -> Instrument {
        let mut rng = rand::thread_rng();
        Instrument {
            clock: 0,
            wavetable: (0..period / 2)
                .map(|_| if rand::random() { 1. } else { -1. })
                .collect::<Vec<f32>>()
                .iter()
                .map(|_| rng.gen())
                .collect::<Vec<f32>>()
                .iter()
                .map(|t| t * 2.)
                .collect::<Vec<f32>>()
                .iter()
                .cycle()
                .take(period as usize)
                .collect::<Vec<&f32>>()
                .iter()
                .map(|&n| *n)
                .collect(),
        }
    }

    pub fn next_frequency(&mut self) -> f32 {
        // Y[t] = 1/2 * (Y[t-p] + Y[t-p-1])
        // Basically, assign to this step the average of this step and the next
        self.clock += 1;
        let sample_clock = self.clock as usize % self.wavetable.len();
        let this_step = self.wavetable[sample_clock];
        let next_step = match self.wavetable.get(sample_clock + 1) {
            Some(n) => n,
            None => &self.wavetable[0],
        };
        let avg = (this_step + next_step) / -2.;
        self.wavetable[sample_clock] = avg;
        avg
    }
}
