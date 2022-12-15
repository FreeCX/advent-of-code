use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Rock,
    Paper,
    Scissors,
}

pub enum Choose {
    Lose,
    Draw,
    Win,
}

pub type Round = (State, State, Choose);

impl From<&str> for State {
    fn from(state: &str) -> State {
        match state {
            "A" | "X" => State::Rock,
            "B" | "Y" => State::Paper,
            "C" | "Z" => State::Scissors,
            other => panic!("unsupported state {}", other),
        }
    }
}

impl From<&str> for Choose {
    fn from(state: &str) -> Choose {
        match state {
            "X" => Choose::Lose,
            "Y" => Choose::Draw,
            "Z" => Choose::Win,
            other => panic!("unsupported state {}", other),
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else {
            use self::State::*;

            match (self, other) {
                (Rock, Paper) => Ordering::Less,
                (Rock, Scissors) => Ordering::Greater,
                (Paper, Rock) => Ordering::Greater,
                (Paper, Scissors) => Ordering::Less,
                (Scissors, Rock) => Ordering::Less,
                (Scissors, Paper) => Ordering::Greater,
                _ => unreachable!(),
            }
        }
    }
}

impl State {
    // return win State
    pub fn opposite_win(&self) -> State {
        match self {
            State::Rock => State::Paper,
            State::Paper => State::Scissors,
            State::Scissors => State::Rock,
        }
    }

    // return lose State
    pub fn opposite_lose(&self) -> State {
        match self {
            State::Rock => State::Scissors,
            State::Paper => State::Rock,
            State::Scissors => State::Paper,
        }
    }
}
