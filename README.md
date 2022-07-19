# roulette_simulation 0.5.0

This program simulates playing against an unbiased Roulette with a Martingale-like doubling strategy. 

This program was developed as an exercise to help me learn Rust and should NOT be used as a gambling strategy.


This strategy works partially and can win a fair amount of games if you only play for a few rounds. But, as the player plays more rounds, you are [mathematically guaranteed](https://en.wikipedia.org/wiki/Roulette#:~:text=.-,Simplified%20mathematical%20model,-%5Bedit%5D) to lose money.

```
USAGE:
    roulette_simulation [OPTIONS] [ARGS]

ARGS:
    <MAX_LOSS_STREAK>     How many times a player loses in a row to stop doubling? [default: 20]
    <MINIMUM_BET>         Minimum Bet (will get doubled as you lose) [default: 1]
    <PLAYER_BET_COUNT>    How many rounds a single person will play? [default: 1000]
    <GAME_COUNT>          How many people will play the roulette? [default: 1000]
    <TABLE_SIZE>          Table size [default: 37]

OPTIONS:
    -h, --help        Print help information
    -n, --no-green    Don't generate the GREEN piece?
    -V, --version     Print version information
```