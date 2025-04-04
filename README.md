# Elo Algorithm in Rust

This library provides an implementation of the **Elo rating system** in Rust. It is a simple, efficient way to calculate the Elo ratings of two players based on the outcome of a match.

## Usage

Here’s a basic example showing how to use this library:

```rust
use elo::{probability, elo_rating};

fn main() {
    let player_a_rating = 1500.0;
    let player_b_rating = 1600.0;
    let k_factor = 32; // Adjust K-factor as needed

    // Calculate the probability of player A winning
    let prob_a = probability(player_a_rating, player_b_rating);
    println!("Probability of player A winning: {}", prob_a);

    // Simulate a game where player A wins (outcome = 1.0)
    let outcome = 1.0;
    let (new_a_rating, new_b_rating) = elo_rating(player_a_rating, player_b_rating, k_factor, outcome);

    println!("New player A rating: {}", new_a_rating);
    println!("New player B rating: {}", new_b_rating);

    // Simulate a game where player B wins (outcome = 0.0)
    let outcome_b = 0.0;
    let (new_a_rating_b, new_b_rating_b) = elo_rating(player_a_rating, player_b_rating, k_factor, outcome_b);

    println!("New player A rating when B wins: {}", new_a_rating_b);
    println!("New player B rating when B wins: {}", new_b_rating_b);

    // Simulate a draw (outcome = 0.5)
    let outcome_draw = 0.5;
    let (new_a_rating_draw, new_b_rating_draw) = elo_rating(player_a_rating, player_b_rating, k_factor, outcome_draw);

    println!("New player A rating when Draw: {}", new_a_rating_draw);
    println!("New player B rating when Draw: {}", new_b_rating_draw);
}
