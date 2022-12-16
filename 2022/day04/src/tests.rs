use crate::parse;
use crate::part_one;
use crate::part_two;
use crate::types::{Assignment, Pair};

#[test]
fn parse_assignment() {
    assert_eq!(Assignment::new(10, 20), Assignment::from("10-20"));
}

#[test]
#[should_panic]
fn parse_assignment_fail() {
    let _ = Assignment::from("1--5");
}

#[test]
fn parse_pair() {
    assert_eq!(Pair::new(Assignment::new(1, 2), Assignment::new(3, 4)), Pair::from("1-2,3-4"));
}

#[test]
#[should_panic]
fn parse_pair_fail() {
    let _ = Pair::from("1,,5");
}

#[test]
fn example_one() {
    let input = parse::parse(include_str!("../data/example"));
    assert_eq!(part_one::process(&input), 2);
}

#[test]
fn example_two() {
    let input = parse::parse(include_str!("../data/example"));
    assert_eq!(part_two::process(&input), 4);
}

#[test]
fn part_one() {
    let input = parse::parse(include_str!("../data/input"));
    assert_eq!(part_one::process(&input), 503);
}

#[test]
fn part_two() {
    let input = parse::parse(include_str!("../data/input"));
    assert_eq!(part_two::process(&input), 827);
}
