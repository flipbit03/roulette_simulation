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
    pub fn new(size: usize, no_green: bool, chooser: T) -> Self {
        let size_is_even = size % 2 == 0;
        match (size_is_even, no_green) {
            (true, false) => panic!("Size must be odd if you want the the GREEN piece"),
            (false, true) => panic!("Size must be even if you don't want the the GREEN piece"),
            (true, true) | (false, false) => (),
        };

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
