use crate::document::Document;
use crate::part_one;
use crate::part_two;

#[test]
fn part_one() {
    let doc = Document::from(include_str!("../data/input"));
    assert_eq!(part_one::process(&doc), 227850);
}

#[test]
fn part_two() {
    let doc = Document::from(include_str!("../data/input"));
    assert_eq!(part_two::process(&doc), 42948149);
}
