use std::io::prelude::*;
use std::io;

fn analyze(string: &str) -> usize {
    let mut it = string.chars();
    let mut counter: usize = 0;
    let mut in_string: usize = 0;
    loop {
        let character = match it.next() {
            Some(value) => value,
            None => break,
        };
        if character == '"' && (counter == 0 || counter == string.len()) {
            continue;
        } else if character == '\\' {
            let ch1 = it.next().unwrap();
            if ch1 == 'x' {
                let p1 = it.next().unwrap();
                let p2 = it.next().unwrap();
            } else {
                let _ = it.next();
            }
        } else {
            in_string += 1;
        }
        counter += 1;
    }
    string.len() - in_string
}

#[test]
fn examples() {
    assert!(12 == (analyze("\"\"") + analyze("\"abc\"") + analyze("\"aaa\\\"aaa\"") + analyze("\"\\x27\"")));
}

fn main() {
    let input = io::stdin();
    let mut result: usize = 0;
    for line in input.lock().lines() {
        match line {
            Ok(value) => {
                result += analyze(&value);
            }
            Err(why) => break,
        };
    }
    println!("result = {}", result);
}
