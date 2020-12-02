use std::collections::HashSet;
use std::fs;

fn find_one(value: i32, sum_val: i32, list: &HashSet<i32>) -> Option<i32> {
    let to_find = sum_val - value;
    if list.contains(&to_find) {
        Some(to_find)
    } else {
        None
    }
}

fn main() {
    let sum_val = 2020;
    let items: HashSet<i32> = fs::read_to_string("input")
        .unwrap()
        .trim()
        .split("\n")
        .map(|x| x.parse().unwrap())
        .collect();

    let first_items: HashSet<i32> = items.clone().into_iter().filter(|&x| x <= sum_val).collect();
    'first_part: for item in &first_items {
        match find_one(*item, sum_val, &first_items) {
            Some(value) => {
                println!(" first = {}", item * value);
                break 'first_part;
            },
            None => (),
        }
    }

    'second_part: for first in &items {
        let last_parts = sum_val - first;
        let filtered: HashSet<i32> = items.clone().into_iter().filter(|&x| x <= last_parts).collect();
        for second in &filtered {
            match find_one(*second, last_parts, &filtered) {
                Some(value) => {
                    println!("second = {}", first * second * value);
                    break 'second_part;
                },
                None => (),
            }
        }
    }
}
