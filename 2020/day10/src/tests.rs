use crate::part_one;
use crate::part_two;
use crate::parse;
use crate::types::{Array, Diffs};

const E1: &[u16] = &[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
const E2: &[u16] = &[
    28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
    17, 7, 9, 4, 2, 34, 10, 3,
];

#[test]
fn example_11() {
    let r: Diffs = vec![(1, 7), (3, 5)].into_iter().collect();
    let n: Array = parse::prepare(E1);
    assert_eq!(part_one::process(&n), r);
}

#[test]
fn example_12() {
    let r: Diffs = vec![(1, 22), (3, 10)].into_iter().collect();
    let n = parse::prepare(E2);
    assert_eq!(part_one::process(&n), r);
}

#[test]
fn example_21() {
    let n = parse::prepare(E1);
    assert_eq!(part_two::process(&n), 8);
}

#[test]
fn example_22() {
    let n = parse::prepare(E2);
    assert_eq!(part_two::process(&n), 19208);
}

#[test]
fn part_one() {
    let input = parse::parse(include_str!("../data/input"));
    let result = part_one::fold_one(part_one::process(&input));
    assert_eq!(result, 3034);
}


#[test]
fn part_two() {
    let input = parse::parse(include_str!("../data/input"));
    let result = part_two::process(&input);
    assert_eq!(result, 259172170858496);
}
