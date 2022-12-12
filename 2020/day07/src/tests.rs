use crate::parse;
use crate::part_one;
use crate::part_two;

#[test]
fn example1() {
    let bags = parse::parse(include_str!("../data/example11.txt"));
    assert_eq!(part_one::process(&bags), 4);
}

#[test]
fn example21() {
    let bags = parse::parse(include_str!("../data/example21.txt"));
    assert_eq!(part_two::process(&bags), 32);
}

#[test]
fn example22() {
    let bags = parse::parse(include_str!("../data/example22.txt"));
    assert_eq!(part_two::process(&bags), 126);
}

#[test]
fn part_one() {
    let bags = parse::parse(include_str!("../data/input"));
    assert_eq!(part_one::process(&bags), 139);
}

#[test]
fn part_two() {
    let bags = parse::parse(include_str!("../data/input"));
    assert_eq!(part_two::process(&bags), 58175);
}
