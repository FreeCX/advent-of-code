use crate::parse;
use crate::part_one;
use crate::part_two;

#[test]
fn example_one() {
    let input = parse::parse(include_str!("../data/example"));
    assert_eq!(part_one::process(&input), 95437);
}

#[test]
fn exampe_two() {
    let input = parse::parse(include_str!("../data/example"));
    assert_eq!(part_two::process(&input), 24933642);
}

#[test]
fn part_one() {
    let input = parse::parse(include_str!("../data/input"));
    assert_eq!(part_one::process(&input), 1084134);
}

#[test]
fn part_two() {
    let input = parse::parse(include_str!("../data/input"));
    assert_eq!(part_two::process(&input), 6183184);
}
