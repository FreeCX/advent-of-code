pub fn process(input: &str) -> usize {
    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut total = 0;
    for line in input.lines() {
        let mut items = Vec::new();

        // digits
        for (index, c) in line.chars().enumerate() {
            if let '0'..='9' = c {
                items.push((index, c as usize - 0x30))
            }
        }

        // letters
        for (n, digit) in digits.iter().enumerate() {
            if let Some(index) = line.find(digit) {
                items.push((index, n + 1));
            }
            if let Some(index) = line.rfind(digit) {
                items.push((index, n + 1));
            }
        }

        // result
        let a = items.iter().min_by_key(|(index, _)| index).unwrap();
        let b = items.iter().max_by_key(|(index, _)| index).unwrap();
        total += a.1 * 10 + b.1;
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::part_two::process;

    #[test]
    fn example() {
        assert_eq!(process(include_str!("../data/example02")), 281);
    }
}
