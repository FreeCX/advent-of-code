use crate::parse;
use crate::part_one;
use crate::part_two;

#[test]
fn example_one() {
    let input = parse::parse(include_str!("../data/example"));
    assert_eq!(part_one::process(&input), 21);
}

#[test]
fn example_two_scenic_score() {
    let input = parse::parse(include_str!("../data/example"));
    assert_eq!(part_two::scenic_score(&input, 2, 1), 4);
    assert_eq!(part_two::scenic_score(&input, 2, 3), 8);
}

#[test]
fn part_one() {
    let input = parse::parse(include_str!("../data/input"));
    assert_eq!(part_one::process(&input), 1679);
}

#[test]
fn part_two() {
    let input = parse::parse(include_str!("../data/input"));
    assert_eq!(part_two::process(&input), 536625);
}
