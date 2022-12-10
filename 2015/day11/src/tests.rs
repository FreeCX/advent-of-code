use crate::*;

#[test]
fn one_k_iter() {
    let mut start: Vec<u8> = vec!['a' as u8; 4];
    for _ in 0..1024 {
        start = increment(start);
    }
    assert_eq!("abnk", String::from_utf8(start).unwrap());
}

#[test]
fn next_pass_one() {
    assert_eq!(next_password("abcdefgh"), "abcdffaa");
}

#[test]
fn next_pass_two() {
    assert_eq!(next_password("ghijklmn"), "ghjaabcc");
}
