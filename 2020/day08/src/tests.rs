use crate::part_one;
use crate::part_two;

const APP: &str = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";

#[test]
fn example_one() {
    assert_eq!(part_one::process(APP), 5);
}

#[test]
fn example_two() {
    assert_eq!(part_two::process(APP), 8);
}

#[test]
fn part_one() {
    assert_eq!(part_one::process(include_str!("../data/input")), 1586);
}

#[test]
fn part_two() {
    assert_eq!(part_two::process(include_str!("../data/input")), 703);
}
