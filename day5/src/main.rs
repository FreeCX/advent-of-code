use std::io::prelude::Read;
use std::fs::File;

fn checker<'a>(line: &'a str) -> bool {
    fn rule_two<'a>(line: &'a str) -> bool {
        let mut last_char = line.chars().nth(0).unwrap();
        for curr_char in line.chars().skip(1) {
            if curr_char == last_char {
                return true;
            }
            last_char = curr_char;
        }
        false
    }
    fn rule_three<'a>(line: &'a str) -> bool {
        let items = ["ab", "cd", "pq", "xy"];
        for item in items.iter() {
            if line.contains(item) {
                return false;
            }
        }
        true
    }
    let rule_one = line.chars().filter(|&x| "aeiou".find(x).is_some()).count() >= 3;
    rule_one && rule_two(line) && rule_three(line)
}

#[test]
fn examples {
    assert_eq!(checker("ugknbfddgicrmopn"), true);
    assert_eq!(checker("aaa"), true);
    assert_eq!(checker("jchzalrnumimnmhp"), false);
    assert_eq!(checker("haegwjzuvuyypxyu"), false);
    assert_eq!(checker("dvszwmarrgswjxmb"), false);
}

fn main() {
    let mut f = File::open("input.txt").expect("[error] can't open file!");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("[error] can't read from file!");
    let mut nice_count = 0_usize;
    for line in buffer.lines() {
        if checker(line) {
            nice_count += 1;
        }
    }
    println!("nice strings = {}", nice_count);
}
