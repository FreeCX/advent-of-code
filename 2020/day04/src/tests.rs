use crate::parse;
use crate::part_one;
use crate::part_two;

#[test]
fn part_one() {
    let ids = parse::parse(include_str!("../data/input"));
    assert_eq!(part_one::process(&ids), 264);
}

#[test]
fn part_two() {
    let ids = parse::parse(include_str!("../data/input"));
    assert_eq!(part_two::process(&ids), 224);
}
