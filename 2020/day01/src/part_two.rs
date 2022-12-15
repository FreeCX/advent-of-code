use std::collections::HashSet;

use crate::prepare::*;

pub fn process(sum_val: i32, input: &HashSet<i32>) -> Option<i32> {
    for first in input {
        let last_parts = sum_val - first;
        let filtered: HashSet<i32> = input.clone().into_iter().filter(|&x| x <= last_parts).collect();
        for second in &filtered {
            if let Some(value) = find_one(*second, last_parts, &filtered) {
                return Some(first * second * value);
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
