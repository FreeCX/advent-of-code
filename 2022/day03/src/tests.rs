use crate::parse;
use crate::part_one;
use crate::part_two;

#[test]
fn example_one() {
    let data = parse::parse(include_str!("../data/example"));
    assert_eq!(part_one::process(&data), 157);
}

#[test]
fn example_two() {
    let data = parse::parse(include_str!("../data/example"));
    assert_eq!(part_two::process(&data), 70);
}

#[test]
fn part_one() {
    let data = parse::parse(include_str!("../data/input"));
    assert_eq!(part_one::process(&data), 8105);
}

#[test]
fn part_two() {
    let data = parse::parse(include_str!("../data/input"));
    assert_eq!(part_two::process(&data), 2363);
}
