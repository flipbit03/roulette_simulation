use rand::{
    prelude::{SliceRandom, ThreadRng},
    thread_rng,
};

use crate::enums::RouletteColor;

#[derive(Debug)]
pub struct Roulette {
    colors: Vec<RouletteColor>,
    rng: ThreadRng,
}

impl Roulette {
    pub fn new(size: u8) -> Self {
        if size % 2 != 1 {
            panic!("size must be even")
        }
        let mut colors: Vec<RouletteColor> = Vec::new();

        (0..(size - 1) / 2).into_iter().for_each(|n| {
            colors.push(RouletteColor::BLACK);
            colors.push(RouletteColor::RED);
        });
        colors.push(RouletteColor::GREEN);

        Self {
            colors,
            rng: thread_rng(),
        }
    }

    pub fn play(&mut self) -> RouletteColor {
        self.colors.choose(&mut self.rng).unwrap().clone()
    }
}
