use crate::almanac::Almanac;
use crate::part_one;
use crate::part_two;

#[test]
fn part_one() {
    let almanac = Almanac::from(include_str!("../data/input"));
    assert_eq!(part_one::process(&almanac), 379811651);
}

#[test]
#[ignore] // too many seeds to process
fn part_two() {
    let almanac = Almanac::from(include_str!("../data/input"));
    assert_eq!(part_two::process(&almanac), 27992443);
}
