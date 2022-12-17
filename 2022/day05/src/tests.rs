use crate::parse;
use crate::part_one;
use crate::part_two;
use crate::types::Command;

#[test]
fn parse_command() {
    assert_eq!(Command::from("move 123 from 456 to 789"), Command::new(123, 456, 789));
}

#[test]
fn parse_stacks() {
    let real_stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
    let (parse_stacks, _) = parse::parse(include_str!("../data/example"));
    assert_eq!(parse_stacks, real_stacks);
}

#[test]
fn example_one() {
    let (ship, app) = parse::parse(include_str!("../data/example"));
    assert_eq!(part_one::process(&ship, &app), "CMZ".to_owned());
}

#[test]
fn example_two() {
    let (ship, app) = parse::parse(include_str!("../data/example"));
    assert_eq!(part_two::process(&ship, &app), "MCD".to_owned());
}

#[test]
fn part_one() {
    let (ship, app) = parse::parse(include_str!("../data/input"));
    assert_eq!(part_one::process(&ship, &app), "VPCDMSLWJ".to_owned());
}

#[test]
fn part_two() {
    let (ship, app) = parse::parse(include_str!("../data/input"));
    assert_eq!(part_two::process(&ship, &app), "TPWCGNCCG".to_owned());
}
