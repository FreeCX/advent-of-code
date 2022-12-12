use crate::parse;
use crate::part_one;
use crate::part_two;

#[test]
fn example_one() {
    let data = parse::parse("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb");
    let result = part_one::process(&data);
    assert_eq!(result, 11);
}

#[test]
fn example_two() {
    let data = parse::parse("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb");
    let result = part_two::process(&data);
    assert_eq!(result, 6);
}

#[test]
fn part_one() {
    let data = parse::parse(include_str!("../data/input"));
    let result = part_one::process(&data);
    assert_eq!(result, 6633);
}

#[test]
fn part_two() {
    let data = parse::parse(include_str!("../data/input"));
    let result = part_two::process(&data);
    assert_eq!(result, 3202);
}
