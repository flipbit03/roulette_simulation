use clap::Parser;
use itertools::Itertools;
use rand::thread_rng;

use crate::{enums::RouletteColor, player::RoulettePlayer, roulette::Roulette};

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

    #[clap(default_value_t = 1, value_parser)]
    pub game_count: usize,
}

fn main() {
    let config = RouletteSimulationCLIConfig::parse();

    println!("config = {:?}", &config);

    let r = Roulette::new(37, thread_rng());

    let games_played = (0..config.game_count)
        .into_iter()
        .map(|player_number| {
            let mut player_roulette = r.clone();
            let mut p = RoulettePlayer::new(
                format!("Player{}", player_number).to_string(),
                &mut player_roulette,
                config.max_loss_streak,
                config.minimum_bet,
            );
            for bet_number in 0..config.bet_count {
                let bet = match bet_number % 2 {
                    0 => RouletteColor::RED,
                    _ => RouletteColor::BLACK,
                };

                p.bet(bet, None, false);

                if (config.bet_count > 100 && bet_number % (config.bet_count / 100) == 0)
                    || (bet_number == config.bet_count - 1)
                {
                    let stats = &p.get_stats();
                    println!(
                        "Player{} Bet#: {} Won:{} Lost:{} Balance: {}",
                        player_number,
                        bet_number,
                        stats.win_amount,
                        stats.lost_amount,
                        stats.get_balance()
                    );
                }
            }
            p.get_stats()
        })
        .collect_vec();

    let sorted_by_balance_games_played = games_played
        .iter()
        .sorted_by(|r1, r2| {
            Ord::cmp(&((r1.get_balance() * 1000.0) as i32), &((r2.get_balance() * 1000.0) as i32))
        }).collect_vec();

    let total_games = sorted_by_balance_games_played.iter().count();
    let won_games = sorted_by_balance_games_played
        .iter()
        .filter(|p| p.won()).count();

    println!("========================");
    for (gn, gp) in games_played.iter().enumerate() {
        println!("Player{}: {}", gn, gp);
    }
    println!("========================");
    println!("played {} games", total_games);
    println!(
        "won {} games ({:.0}%)",
        won_games,
        won_games as f64 / total_games as f64 * 100.0
    );

    // for (player_number, player_stats) in games_played.iter().enumerate() {
    //     println!("Player #{}\n{}", player_number, player_stats);
    // }
}
