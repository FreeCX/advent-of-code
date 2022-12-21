use crate::types::LineType;

pub fn parse(input: &str) -> Vec<LineType> {
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(LineType::from(line));
    }
    result
}
