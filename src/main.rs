use clap::Parser;
use rand::thread_rng;

use crate::{
    enums::RouletteColor,
    player::RoulettePlayer,
    roulette::{BiasedRouletteChoser, Roulette},
};

pub mod enums;
pub mod player;
pub mod roulette;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct RouletteSimulationCLIConfig {
    #[clap(default_value_t = 5, value_parser)]
    pub max_loss_streak: usize,

    #[clap(default_value_t = 1.0, value_parser)]
    pub minimum_bet: f64,

    #[clap(default_value_t = 10_000_000, value_parser)]
    pub bet_count: usize,
}

fn main() {
    let config = RouletteSimulationCLIConfig::parse();

    println!("config = {:?}", &config);

    // let mut r = Roulette::new(
    //     37,
    //     BiasedRouletteChoser::new(vec![
    //         RouletteColor::RED,
    //         RouletteColor::RED,
    //         RouletteColor::RED,
    //         RouletteColor::BLACK,
    //     ]),
    // );
    let mut r = Roulette::new(
        37,
        thread_rng()
    );

    let mut p = RoulettePlayer::new(&mut r, config.max_loss_streak, config.minimum_bet);

    (0..config.bet_count).into_iter().for_each(|bet_number| {
        let bet = match bet_number % 2 {
            0 => RouletteColor::BLACK,
            _ => RouletteColor::BLACK,
        };

        if config.bet_count > 100 && bet_number % (config.bet_count / 100) == 0 {
            println!("{}", &p);
        }

        (&mut p).bet(bet, None, false);
    });

    println!("{}", p);
}
