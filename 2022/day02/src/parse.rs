use crate::types::{Round, State, Choose};

pub fn parse(input: &str) -> Vec<Round> {
    let mut result = Vec::new();
    for line in input.lines() {
        let mut item = line.split(' ');
        let opponet = State::from(item.next().unwrap());
        let second_column = item.next().unwrap();
        let you = State::from(second_column);
        let choose = Choose::from(second_column);
        result.push((opponet, you, choose));
    }
    result
}
