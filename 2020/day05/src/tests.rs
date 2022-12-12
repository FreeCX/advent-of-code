use crate::part_one;
use crate::part_two;
use crate::seat::Seat;

#[test]
fn ticket_one() {
    let seat = Seat::identify("BFFFBBFRRR");

    assert_eq!(seat.row, 70);
    assert_eq!(seat.col, 7);
    assert_eq!(seat.id(), 567);
}

#[test]
fn ticket_two() {
    let seat = Seat::identify("FFFBBBFRRR");

    assert_eq!(seat.row, 14);
    assert_eq!(seat.col, 7);
    assert_eq!(seat.id(), 119);
}

#[test]
fn ticket_three() {
    let seat = Seat::identify("BBFFBBFRLL");

    assert_eq!(seat.row, 102);
    assert_eq!(seat.col, 4);
    assert_eq!(seat.id(), 820);
}

#[test]
fn part_one() {
    let tickets: Vec<_> = include_str!("../data/input").lines().collect();
    assert_eq!(part_one::process(&tickets), 801);
}

#[test]
fn part_two() {
    let tickets: Vec<_> = include_str!("../data/input").lines().collect();
    assert_eq!(part_two::process(&tickets), 597);
}
