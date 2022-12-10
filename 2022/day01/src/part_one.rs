pub fn process(input: &str) -> u32 {
    let mut max_value = 0;
    let mut cur_value = 0;

    for item in input.split('\n') {
        if item.is_empty() {
            if cur_value > max_value {
                max_value = cur_value;
            }
            cur_value = 0;
            continue;
        }

        cur_value += item.parse::<u32>().unwrap();
    }

    max_value
}

#[cfg(test)]
mod tests {
    use crate::part_one::process;

    #[test]
    fn default() {
        assert_eq!(process(include_str!("../data/demo")), 24_000);
    }
}
