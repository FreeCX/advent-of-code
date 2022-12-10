use crate::checker_one;

#[test]
fn test01() {
    assert_eq!(checker_one("ugknbfddgicrmopn"), true);
}

#[test]
fn test02() {
    assert_eq!(checker_one("aaa"), true);
}

#[test]
fn test03() {
    assert_eq!(checker_one("jchzalrnumimnmhp"), false);
}

#[test]
fn test04() {
    assert_eq!(checker_one("haegwjzuvuyypxyu"), false);
}

#[test]
fn test05() {
    assert_eq!(checker_one("dvszwmarrgswjxmb"), false);
}
