use crate::*;

#[test]
fn test1() {
    assert!(paper_one(2, 3, 4) == 58);
}

#[test]
fn test2() {
    assert!(paper_one(1, 1, 10) == 43);
}

#[test]
fn test3() {
    assert!(paper_two(2, 3, 4) == 34);
}

#[test]
fn test4() {
    assert!(paper_two(1, 1, 10) == 14);
}
