use std::collections::HashMap;

pub fn process(input: &str) -> u32 {
    let mut left = Vec::new();
    let mut right = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.trim().split_once("   ").unwrap();
        let a = a.parse::<u32>().unwrap();
        let b = b.parse::<u32>().unwrap();
        left.push(a);
        right.entry(b).and_modify(|v| *v += 1).or_insert(1);
    }

    left.into_iter().map(|i| if right.contains_key(&i) { i * right.get(&i).unwrap() } else { 0 }).sum()
}

#[cfg(test)]
mod tests {
    use crate::part_two::process;

    #[test]
    fn example() {
        assert_eq!(process(include_str!("../data/example01")), 31);
    }
}
