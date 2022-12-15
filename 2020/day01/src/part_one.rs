use std::collections::HashSet;

use crate::prepare::*;

pub fn process(sum_val: i32, input: &HashSet<i32>) -> Option<i32> {
    for item in input {
        if let Some(value) = find_one(*item, sum_val, input) {
            return Some(item * value);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::part_one;
    use crate::prepare;

    #[test]
    fn default() {
        let value = 2020;
        let input = prepare::process(value, include_str!("../data/example"));
        assert_eq!(part_one::process(value, &input), Some(514579));
    }
}
