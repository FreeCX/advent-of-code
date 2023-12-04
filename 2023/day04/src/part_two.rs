use crate::card::Card;

pub fn process(cards: &[Card]) -> usize {
    let mut totals = vec![1; cards.len()];

    for card in cards {
        let end = (card.id + card.intersect().len() as u32).min(cards.len() as u32);
        for i in card.id..end {
            totals[i as usize] += totals[card.id as usize - 1];
        }
    }

    totals.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::part_two::process;

    #[test]
    fn example() {
        let cards = parse::process(include_str!("../data/example"));
        assert_eq!(process(&cards), 30);
    }
}
