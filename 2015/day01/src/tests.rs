use crate::*;

#[test]
fn test01() {
    assert!(floor("(())") == (0, None) && floor("()()") == (0, None));
}

#[test]
fn test02() {
    assert!(floor("(((") == (3, None) && floor("(()(()(") == (3, None));
}

#[test]
fn test03() {
    assert!(floor("))(((((") == (3, Some(1)));
}

#[test]
fn test04() {
    assert!(floor("())") == (-1, Some(3)) && floor("))(") == (-1, Some(1)));
}

#[test]
fn test05() {
    assert!(floor(")))") == (-3, Some(1)) && floor(")())())") == (-3, Some(1)));
}
