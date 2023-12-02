pub fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let n: Vec<_> = l.chars().filter(|c| ('0'..='9').contains(c)).map(|v| v as u32 - 0x30).collect();
            *n.first().unwrap() * 10 + *n.last().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::part_one::process;

    #[test]
    fn example() {
        assert_eq!(process(include_str!("../data/example01")), 142);
    }
}
