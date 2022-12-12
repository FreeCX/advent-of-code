use crate::part_one;
use crate::part_two;
use crate::prepare;

const VALUE: i32 = 2020;

#[test]
fn part_one() {
    let items = prepare::process(VALUE, include_str!("../data/input"));
    let part_one = part_one::process(VALUE, &items);
    assert_eq!(part_one, Some(918339));
}

#[test]
fn part_two() {
    let items = prepare::process(VALUE, include_str!("../data/input"));
    let part_two = part_two::process(VALUE, &items);
    assert_eq!(part_two, Some(23869440));
}
