use crate::parse;
use crate::part_one;
use crate::part_two;

#[test]
fn part_one() {
    let cards = parse::process(include_str!("../data/input"));
    assert_eq!(part_one::process(&cards), 24848);
}

#[test]
fn part_two() {
    let cards = parse::process(include_str!("../data/input"));
    assert_eq!(part_two::process(&cards), 7258152);
}
