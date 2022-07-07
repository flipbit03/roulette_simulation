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
    played_games: usize,
    win_count: usize,
    win_amount: f64,
    loss_count: usize,
    lost_amount: f64,
    losing_streak: usize,
    lost_in_a_row_count: usize,
    biggest_best: f64,
    // config
    max_loss_streak: usize,
    minimum_bet: f64,
}

impl<'a, T> Display for RoulettePlayer<'a, T>
where
    T: RouletteChooser,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
Player {{
  rounds: {}
  won: {:06} lost: {:06} lost_{}_in_a_row: {:06} biggest_bet$: {}
  won$:   {:010} 
  lost$:  {:010}
  result: {:010}
}}",
            self.played_games,
            self.win_count,
            self.loss_count,
            self.max_loss_streak,
            self.lost_in_a_row_count,
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
    pub fn new(roulette: &'a mut Roulette<T>, max_loss_streak: usize, minimum_bet: f64) -> Self {
        Self {
            roulette,
            played_games: 0,
            win_count: 0,
            win_amount: 0.0,
            loss_count: 0,
            lost_amount: 0.0,
            losing_streak: 0,
            lost_in_a_row_count: 0,
            biggest_best: 0.0,
            // config:
            max_loss_streak,
            minimum_bet,
        }
    }

    pub fn bet(&mut self, bet: RouletteColor, custom_bet_value: Option<f64>, debug: bool) {
        let bet_value = match custom_bet_value {
            Some(set_bet_value) => set_bet_value,
            None => match self.losing_streak > 0 {
                true => (self.minimum_bet * 2.0).powf(self.losing_streak as f64),
                false => self.minimum_bet,
            },
        };

        if bet_value > self.biggest_best {
            self.biggest_best = bet_value
        }

        let roulette_result = self.roulette.play();

        self.played_games += 1;
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
        self.win_count += 1;
        self.win_amount += bet_value;
        self.losing_streak = 0;
    }

    fn lose(&mut self, bet_value: f64) {
        self.loss_count += 1;
        self.lost_amount -= bet_value;
        self.losing_streak += 1;

        if self.losing_streak == self.max_loss_streak {
            self.lost_in_a_row_count += 1;
            self.losing_streak = 0
        };
    }
}
