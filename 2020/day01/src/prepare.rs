use std::collections::HashSet;

pub fn find_one(value: i32, sum_val: i32, list: &HashSet<i32>) -> Option<i32> {
    let to_find = sum_val - value;
    if list.contains(&to_find) {
        Some(to_find)
    } else {
        None
    }
}

pub fn process(value: i32, input: &str) -> HashSet<i32> {
    input
        .split('\n')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .filter(|&x| x <= value)
        .collect()
}
