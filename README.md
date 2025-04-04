# Elo Algorithm in Rust

This library provides an implementation of the **Elo rating system** in Rust. It is a simple, efficient way to calculate the Elo ratings of two players based on the outcome of a match.

## Usage

Here’s a basic example showing how to use this library:

```rust
use elo;

fn main() {
    let player_a_rating = 1500.0;
    let player_b_rating = 1400.0;
    let k_factor = 32;
    let match_outcome = 1.0; // Player A wins

    let (new_player_a_rating, new_player_b_rating) = elo::elo_rating(player_a_rating, player_b_rating, k_factor, match_outcome);

    println!("Updated Elo ratings:");
    println!("Player A: {}", new_player_a_rating);
    println!("Player B: {}", new_player_b_rating);
}
