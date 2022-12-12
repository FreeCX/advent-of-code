use crate::parse;
use crate::part_one;
use crate::part_two;

const N: &[i64] = &[35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];
const HEADER_SIZE: usize = 25;

#[test]
fn example_one() {
    assert_eq!(part_one::process(N, 5), 127);
}

#[test]
fn example_one_extra() {
    let mut n: Vec<i64> = (1..=25).collect();
    n.extend_from_slice(&[26, 49, 100, 50]);
    assert_eq!(part_one::process(&n, 25), 100);
}

#[test]
fn example_two() {
    assert_eq!(part_two::process(N, 5), 62);
}

#[test]
fn part_one() {
    let data = parse::parse(include_str!("../data/input"));
    assert_eq!(part_one::process(&data, HEADER_SIZE), 1504371145);
}

#[test]
fn part_two() {
    let data = parse::parse(include_str!("../data/input"));
    assert_eq!(part_two::process(&data, HEADER_SIZE), 183278487);
}
