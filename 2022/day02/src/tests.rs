use crate::parse;
use crate::part_one;
use crate::part_two;

const GAMES: &str = "A Y\nB X\nC Z";

#[test]
fn rock_paper_scissors_rule() {
    use crate::types::State::*;
    use std::cmp::Ordering::*;

    assert_eq!(Rock.cmp(&Rock), Equal);
    assert_eq!(Rock.cmp(&Scissors), Greater);
    assert_eq!(Rock.cmp(&Paper), Less);

    assert_eq!(Paper.cmp(&Paper), Equal);
    assert_eq!(Paper.cmp(&Rock), Greater);
    assert_eq!(Paper.cmp(&Scissors), Less);

    assert_eq!(Scissors.cmp(&Scissors), Equal);
    assert_eq!(Scissors.cmp(&Rock), Less);
    assert_eq!(Scissors.cmp(&Paper), Greater);
}

#[test]
fn test_opposites() {
    use crate::types::State::*;

    // Rock lose Paper
    assert_eq!(Rock.opposite_win(), Paper);
    // Rock win Scissors
    assert_eq!(Rock.opposite_lose(), Scissors);

    assert_eq!(Paper.opposite_win(), Scissors);
    assert_eq!(Paper.opposite_lose(), Rock);

    assert_eq!(Scissors.opposite_win(), Rock);
    assert_eq!(Scissors.opposite_lose(), Paper);
}

#[test]
fn example_one() {
    let rounds = parse::parse(GAMES);
    assert_eq!(part_one::process(&rounds), 15);
}

#[test]
fn example_two() {
    let rounds = parse::parse(GAMES);
    assert_eq!(part_two::process(&rounds), 12);
}

#[test]
fn part_one() {
    let rounds = parse::parse(include_str!("../data/input"));
    assert_eq!(part_one::process(&rounds), 10404);
}

#[test]
fn part_two() {
    let rounds = parse::parse(include_str!("../data/input"));
    assert_eq!(part_two::process(&rounds), 10334);
}
