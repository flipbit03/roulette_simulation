use core::panic;

use clap::Parser;
use itertools::Itertools;
use rand::thread_rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use roulette_simulation::{
    enums::RouletteColor,
    player::{RoulettePlayer, RoulettePlayerStats},
    roulette::Roulette,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct RouletteSimulationCLIConfig {
    /// How many times a player loses in a row to stop doubling?
    #[clap(default_value_t = 20, value_parser)]
    pub max_loss_streak: usize,

    /// Minimum Bet (will get doubled as you lose)
    #[clap(default_value_t = 1.0, value_parser)]
    pub minimum_bet: f64,

    /// How many rounds a single person will play?
    #[clap(default_value_t = 1000, value_parser)]
    pub player_bet_count: usize,

    /// How many people will play the roulette?
    #[clap(default_value_t = 1000, value_parser)]
    pub game_count: usize,

    /// Table size
    #[clap(default_value_t = 37, value_parser)]
    pub table_size: usize,

    /// Don't generate the GREEN piece?
    #[clap(short, long, action)]
    no_green: bool,
}

fn main() {
    let config = RouletteSimulationCLIConfig::parse();

    let size_is_even = config.table_size % 2 == 0;
    match (size_is_even, config.no_green) {
        (true, true) => (),
        (true, false) => panic!("Table size must be odd if you want the the GREEN piece!"),
        (false, true) => panic!("Table size must be even if you don't want the the GREEN piece!"),
        (false, false) => (),
    };

    let games_played: Vec<RoulettePlayerStats> = (0..config.game_count)
        .into_par_iter()
        .map(|player_number| {
            let r = Roulette::new(config.table_size, config.no_green, thread_rng());
            let mut player_roulette = r.clone();
            let mut p = RoulettePlayer::new(
                format!("Player{}", player_number).to_string(),
                &mut player_roulette,
                config.max_loss_streak,
                config.minimum_bet,
            );
            for bet_number in 0..config.player_bet_count {
                let bet = match bet_number % 2 {
                    0 => RouletteColor::RED,
                    _ => RouletteColor::BLACK,
                };

                p.bet(&bet, None, false);

                if (config.player_bet_count > 100
                    && bet_number % (config.player_bet_count / 100) == 0)
                    || (bet_number == config.player_bet_count - 1)
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
        .collect();

    let sorted_by_balance_games_played = games_played
        .iter()
        .sorted_by(|r1, r2| r1.get_balance().partial_cmp(&r2.get_balance()).unwrap())
        .collect_vec();

    let total_games = sorted_by_balance_games_played.iter().count();
    let won_games = sorted_by_balance_games_played
        .iter()
        .filter(|p| p.won())
        .count();

    println!("========================");
    for (gn, gp) in sorted_by_balance_games_played.iter().enumerate() {
        println!("Player{}: {}", gn, gp);
    }

    println!("========================");
    println!("config = {:?}", &config);
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
