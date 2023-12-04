use crate::card::Card;

pub fn process(input: &str) -> Vec<Card> {
    input.lines().map(Card::from).collect()
}
