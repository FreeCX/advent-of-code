use std::cmp::Ordering;

use crate::types::{Round, State};

pub fn outcome_of_round(opponent: &State, you: &State) -> u32 {
    match opponent.cmp(you) {
        Ordering::Less => 6,    // win
        Ordering::Equal => 3,   // draw
        Ordering::Greater => 0, // lose
    }
}

pub fn shape_selected(you: &State) -> u32 {
    match you {
        State::Rock => 1,
        State::Paper => 2,
        State::Scissors => 3,
    }
}

pub fn process(rounds: &[Round]) -> u32 {
    let mut total_score = 0;
    for (opponent, you, _) in rounds {
        total_score += outcome_of_round(opponent, you) + shape_selected(you);
    }
    total_score
}
