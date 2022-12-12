use crate::parse;
use crate::part_one;
use crate::part_two;

#[test]
fn part_one() {
    let land = parse::parse(include_str!("../data/input"));
    assert_eq!(part_one::process((3, 1), &land), 289);
}

#[test]
fn part_two() {
    let land = parse::parse(include_str!("../data/input"));
    assert_eq!(part_two::process(&land), 5522401584);
}
