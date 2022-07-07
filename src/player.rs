use std::fmt::Display;

use crate::{
    enums::RouletteColor,
    roulette::{Roulette, RouletteChooser},
};

#[derive(Debug)]
pub struct RoulettePlayer<'a, T>
where
    T: RouletteChooser,
{
    roulette: &'a mut Roulette<T>,
    stats: RoulettePlayerStats,
    config: RoulettePlayerConfig,
}

#[derive(Debug, Clone)]
pub struct RoulettePlayerConfig {
    pub max_loss_streak: usize,
    pub minimum_bet: f64,
}

#[derive(Debug, Clone)]
pub struct RoulettePlayerStats {
    pub player_name: String,
    pub played_games: usize,
    pub win_count: usize,
    pub win_amount: f64,
    pub loss_count: usize,
    pub lost_amount: f64,
    pub lost_in_a_row_count: usize,
    pub highest_loss_streak_count: usize,
    pub losing_streak: usize,
    pub biggest_best: f64,
    pub config: RoulettePlayerConfig,
}

impl RoulettePlayerStats {
    pub fn new(config: &RoulettePlayerConfig, player_name: String) -> Self {
        Self {
            player_name,
            played_games: 0,
            win_count: 0,
            win_amount: 0.0,
            loss_count: 0,
            lost_amount: 0.0,
            biggest_best: 0.0,
            lost_in_a_row_count: 0,
            highest_loss_streak_count: 0,
            config: config.clone(),
            losing_streak: 0,
        }
    }
    pub fn get_balance(&self) -> f64 {
        self.win_amount + self.lost_amount
    }

    pub fn won(&self) -> bool {
        self.get_balance() > 0.0
    }
}

impl Display for RoulettePlayerStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
{} {{
  rounds: {} (won: {:06} lost: {:06} lost_{}_in_a_row: {:06})
  highest_loss_streak_count:{} biggest_bet$: {}
  won$:   {:010} 
  lost$:  {:010}
  result: {:010}
}}",
            self.player_name,
            self.played_games,
            self.win_count,
            self.loss_count,
            self.config.max_loss_streak,
            self.lost_in_a_row_count,
            self.highest_loss_streak_count,
            self.biggest_best,
            self.win_amount,
            self.lost_amount,
            self.win_amount + self.lost_amount
        )
    }
}

impl<'a, T> RoulettePlayer<'a, T>
where
    T: RouletteChooser,
{
    pub fn new(
        name: String,
        roulette: &'a mut Roulette<T>,
        max_loss_streak: usize,
        minimum_bet: f64,
    ) -> Self {
        let config = RoulettePlayerConfig {
            max_loss_streak,
            minimum_bet,
        };
        Self {
            stats: RoulettePlayerStats::new(&config, name),
            roulette,
            config,
        }
    }

    pub fn bet(&mut self, bet: RouletteColor, custom_bet_value: Option<f64>, debug: bool) {
        let bet_value = match custom_bet_value {
            Some(set_bet_value) => set_bet_value,
            None => match self.stats.losing_streak > 0 {
                true => (self.config.minimum_bet * 2.0).powf(self.stats.losing_streak as f64),
                false => self.config.minimum_bet,
            },
        };

        if bet_value > self.stats.biggest_best {
            self.stats.biggest_best = bet_value
        }

        let roulette_result = self.roulette.play();

        self.stats.played_games += 1;
        let won = roulette_result == bet;
        let won_str = match won {
            true => {
                self.win(bet_value);
                "WON"
            }
            false => {
                self.lose(bet_value);
                "LOST"
            }
        };

        if debug {
            println!("{} (bet$: {} roulette: ", won_str, bet_value);
        }
    }

    fn win(&mut self, bet_value: f64) {
        self.stats.win_count += 1;
        self.stats.win_amount += bet_value;
        if self.stats.losing_streak > self.stats.highest_loss_streak_count {
            self.stats.highest_loss_streak_count = self.stats.losing_streak
        }
        self.stats.losing_streak = 0;
    }

    fn lose(&mut self, bet_value: f64) {
        self.stats.loss_count += 1;
        self.stats.lost_amount -= bet_value;
        self.stats.losing_streak += 1;

        if self.stats.losing_streak == self.config.max_loss_streak {
            self.stats.lost_in_a_row_count += 1;
            self.stats.losing_streak = 0;
            self.stats.highest_loss_streak_count = self.config.max_loss_streak;
        };
    }

    pub fn get_stats(&self) -> RoulettePlayerStats {
        self.stats.clone()
    }
}
