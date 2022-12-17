use crate::part_one;
use crate::part_two;

#[test]
fn example_one() {
    let input = include_str!("../data/example");
    let results = vec![7, 5, 6, 10, 11];
    for (input, result) in input.lines().zip(results) {
        assert_eq!(part_one::process(input.as_bytes()), result);
    }
}

#[test]
fn example_two() {
    let input = include_str!("../data/example");
    let results = vec![19, 23, 23, 29, 26];
    for (input, result) in input.lines().zip(results) {
        assert_eq!(part_two::process(input.as_bytes()), result);
    }
}

#[test]
fn part_one() {
    let input = include_str!("../data/input").as_bytes();
    assert_eq!(part_one::process(input), 1816);
}

#[test]
fn part_two() {
    let input = include_str!("../data/input").as_bytes();
    assert_eq!(part_two::process(input), 2625);
}
