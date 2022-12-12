use crate::parse;
use crate::part_one;
use crate::part_two;

#[test]
fn part_one() {
    let passwords = include_str!("../data/input");
    let mut counter = 0;

    for item in passwords.lines() {
        let rule = parse::parse(item);
        if part_one::rule_one_check(&rule) {
            counter += 1;
        }
    }

    assert_eq!(counter, 625);
}

#[test]
fn part_two() {
    let passwords = include_str!("../data/input");
    let mut counter = 0;

    for item in passwords.lines() {
        let rule = parse::parse(item);
        if part_two::rule_two_check(&rule) {
            counter += 1;
        }
    }

    assert_eq!(counter, 391);
}
