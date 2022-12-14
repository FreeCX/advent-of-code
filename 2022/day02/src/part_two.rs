use crate::types::{Round, Choose};
use crate::part_one::{outcome_of_round, shape_selected};

pub fn process(rounds: &[Round]) -> u32 {
    let mut total_score = 0;
    for (opponent, _, choose) in rounds {
        let new_you = match choose {
            Choose::Win => opponent.opposite_win(),
            Choose::Draw => *opponent,
            Choose::Lose => opponent.opposite_lose(),
        };
        total_score += outcome_of_round(opponent, &new_you) + shape_selected(&new_you);
    }
    total_score
}
