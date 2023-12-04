use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    pub id: u32,
    pub win: Vec<u32>,
    pub have: Vec<u32>,
}

impl From<&str> for Card {
    fn from(input: &str) -> Self {
        let (card, other) = input.split_once(':').unwrap();
        let (_, id) = card.split_once(' ').unwrap();
        let id = id.trim().parse().unwrap();

        let (win, have) = other.split_once('|').unwrap();
        let win = win.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let have = have.split_whitespace().map(|x| x.parse().unwrap()).collect();

        Card { id, win, have }
    }
}

impl Card {
    pub fn intersect(&self) -> Vec<u32> {
        HashSet::<u32>::from_iter(self.win.clone())
            .intersection(&HashSet::from_iter(self.have.clone()))
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::card::Card;

    #[test]
    fn example() {
        let parsed = Card::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        let real = Card { id: 1, win: vec![41, 48, 83, 86, 17], have: vec![83, 86, 6, 31, 17, 9, 48, 53] };
        assert_eq!(parsed, real);
    }
}
