use roulette_simulation::{
    enums::RouletteColor,
    player::RoulettePlayer,
    roulette::{BiasedRouletteChoser, Roulette},
};

#[test]
pub fn play_lose_3_times_and_earn_it_all_back() {
    let biased_roulette_chooser_sequence = vec![
        RouletteColor::BLACK,
        RouletteColor::BLACK,
        RouletteColor::BLACK,
        RouletteColor::RED,
    ];

    let player_bets = [
        RouletteColor::RED,
        RouletteColor::RED,
        RouletteColor::RED,
        RouletteColor::RED,
    ];

    // Create a biased Roulette
    let mut roulette = Roulette::new(
        37,
        false,
        BiasedRouletteChoser::new(biased_roulette_chooser_sequence),
    );

    // Create player that doubles at most 4 times and bets 1 coin at minimum, doubling everytime
    let mut player = RoulettePlayer::new("Martingale".to_string(), &mut roulette, 4, 1.0);

    // Bet 4 times on the RED
    player_bets.iter().for_each(|current_bet| {
        player.bet(current_bet, None, false);
    });

    // Player should have 1 coin by now
    assert_eq!(player.get_stats().get_balance(), 1.0);
}
