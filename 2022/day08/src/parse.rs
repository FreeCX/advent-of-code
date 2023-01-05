use crate::types;

pub fn parse(input: &str) -> types::Data {
    let mut result = types::Data::default();
    for line in input.lines() {
        let block: Vec<u8> = line.chars().map(|x| x as u8 - b'0').collect();
        if result.height == 0 {
            result.width = block.len();
        }
        result.data.extend(block);
        result.height += 1;
    }
    result
}
