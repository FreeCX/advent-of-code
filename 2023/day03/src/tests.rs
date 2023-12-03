use crate::part_one;
use crate::part_two;
use crate::schema::Schema;

#[test]
fn part_one() {
    let schema = Schema::from(include_str!("../data/input"));
    assert_eq!(part_one::process(&schema), 538046);
}

#[test]
fn part_two() {
    let schema = Schema::from(include_str!("../data/input"));
    assert_eq!(part_two::process(&schema), 81709807);
}
