use rand::prelude::{SliceRandom, ThreadRng};

use crate::enums::RouletteColor;

pub trait RouletteChooser {
    fn play(&mut self, colors: &Vec<RouletteColor>) -> RouletteColor;
}

#[derive(Debug, Clone)]
pub struct Roulette<T> {
    colors: Vec<RouletteColor>,
    chooser: T,
}

impl<T> Roulette<T>
where
    T: RouletteChooser,
{
    pub fn new(size: u8, chooser: T) -> Self {
        if size % 2 != 1 {
            panic!("size must be even")
        }
        let mut colors: Vec<RouletteColor> = Vec::new();

        (0..(size - 1) / 2).into_iter().for_each(|_| {
            colors.push(RouletteColor::BLACK);
            colors.push(RouletteColor::RED);
        });
        colors.push(RouletteColor::GREEN);

        Self { colors, chooser }
    }

    pub fn play(&mut self) -> RouletteColor {
        self.chooser.play(&self.colors)
    }
}

impl RouletteChooser for ThreadRng {
    fn play(&mut self, colors: &Vec<RouletteColor>) -> RouletteColor {
        colors.choose(self).unwrap().clone()
    }
}

pub struct BiasedRouletteChoser {
    sequence: Vec<RouletteColor>,
    last_index: usize,
}

impl BiasedRouletteChoser {
    pub fn new(sequence: Vec<RouletteColor>) -> Self {
        if sequence.is_empty() {
            panic!("Needs to have a sequence of at least 1 item");
        }
        Self {
            sequence,
            last_index: 0,
        }
    }
}

impl RouletteChooser for BiasedRouletteChoser {
    fn play(&mut self, _: &Vec<RouletteColor>) -> RouletteColor {
        let r = self.sequence[self.last_index].clone();
        self.last_index = (self.last_index + 1) % self.sequence.len();
        r
    }
}
