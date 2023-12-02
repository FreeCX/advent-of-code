use crate::parse;
use crate::part_one;
use crate::part_two;

#[test]
fn part_one() {
    let games = parse::process(include_str!("../data/input"));
    assert_eq!(part_one::process(&games), 2076);
}

#[test]
fn part_two() {
    let games = parse::process(include_str!("../data/input"));
    assert_eq!(part_two::process(&games), 70950);
}
