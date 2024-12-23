use crate::part_one;
use crate::part_two;

#[test]
fn part_one() {
    assert_eq!(part_one::process(include_str!("../data/input")), 2378066);
}

#[test]
fn part_two() {
    assert_eq!(part_two::process(include_str!("../data/input")), 18934359);
}
