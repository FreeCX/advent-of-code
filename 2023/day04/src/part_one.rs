use crate::card::Card;

pub fn process(cards: &[Card]) -> u32 {
    let mut total_points = 0;

    for card in cards {
        let count = card.intersect().len();
        if count > 0 {
            total_points += 2_u32.pow(count as u32 - 1);
        }
    }

    total_points
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::part_one::process;

    #[test]
    fn example() {
        let cards = parse::process(include_str!("../data/example"));
        assert_eq!(process(&cards), 13);
    }
}
