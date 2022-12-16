use crate::types::Pair;

pub fn parse(input: &str) -> Vec<Pair> {
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(Pair::from(line));
    }
    result
}
