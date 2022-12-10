use crate::*;

#[test]
fn only_one() {
    assert_eq!(compress("1"), "11");
}

#[test]
fn two_ones() {
    assert_eq!(compress("11"), "21");
}

#[test]
fn two_one() {
    assert_eq!(compress("21"), "1211");
}

#[test]
fn four_digit() {
    assert_eq!(compress("1211"), "111221");
}

#[test]
fn six_digit() {
    assert_eq!(compress("111221"), "312211");
}
