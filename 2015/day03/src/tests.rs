use crate::*;

#[test]
fn one_str() {
    let one_str = "^v";

    assert!(house_counter(one_str) == 2);
    assert!(with_robot(one_str) == 3);
}

#[test]
fn two_str() {
    let two_str = "^>v<";

    assert!(house_counter(two_str) == 4);
    assert!(with_robot(two_str) == 3);
}

#[test]
fn three_str() {
    let thr_str = "^v^v^v^v^v";
    assert!(house_counter(thr_str) == 2);
    assert!(with_robot(thr_str) == 11);
}
