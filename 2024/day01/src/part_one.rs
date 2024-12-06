pub fn process(input: &str) -> u32 {
    let count = input.lines().count();
    let mut left = Vec::with_capacity(count);
    let mut right = Vec::with_capacity(count);

    for line in input.lines() {
        let (a, b) = line.trim().split_once("   ").unwrap();
        left.push(a.parse::<u32>().unwrap());
        right.push(b.parse::<u32>().unwrap());
    }

    left.sort();
    right.sort();

    left.into_iter().zip(right).map(|(x, y)| x.abs_diff(y)).sum()
}

#[cfg(test)]
mod tests {
    use crate::part_one::process;

    #[test]
    fn example() {
        assert_eq!(process(include_str!("../data/example01")), 11);
    }
}
