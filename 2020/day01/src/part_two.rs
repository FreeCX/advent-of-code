use crate::prepare::*;
use std::collections::HashSet;

pub fn process(sum_val: i32, input: &HashSet<i32>) -> Option<i32> {
    for first in input {
        let last_parts = sum_val - first;
        let filtered: HashSet<i32> = input
            .clone()
            .into_iter()
            .filter(|&x| x <= last_parts)
            .collect();
        for second in &filtered {
            match find_one(*second, last_parts, &filtered) {
                Some(value) => {
                    return Some(first * second * value);
                }
                None => (),
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::part_two;
    use crate::prepare;

    #[test]
    fn default() {
        let value = 2020;
        let input = prepare::process(value, include_str!("../data/example"));
        assert_eq!(part_two::process(value, &input), Some(241861950));
    }
}
