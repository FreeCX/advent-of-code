use crate::types::*;

pub fn parse(data: &str) -> Land {
    let mut land = Vec::new();

    for line in data.lines() {
        let mut block = Vec::new();
        for item in line.chars() {
            block.push(match item {
                '.' => Field::Empty,
                '#' => Field::Tree,
                _ => panic!("unknown item"),
            });
        }
        land.push(block);
    }

    let width = land[0].len();
    let height = land.len();

    Land { data: land, width, height }
}
