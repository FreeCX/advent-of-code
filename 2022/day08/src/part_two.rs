use crate::types;

pub fn scenic_score(input: &types::Data, x: usize, y: usize) -> usize {
    let current = input.get(x, y);

    let up: Vec<_> = (0..y).map(|y| input.get(x, y)).collect();
    let left: Vec<_> = (0..x).map(|x| input.get(x, y)).collect();
    let right: Vec<_> = (x + 1..input.width).map(|x| input.get(x, y)).collect();
    let down: Vec<_> = (y + 1..input.height).map(|y| input.get(x, y)).collect();

    let up_index = up.iter().enumerate().rev().find(|&(_, v)| v >= &current);
    let left_index = left.iter().enumerate().rev().find(|&(_, v)| v >= &current);
    let right_index = right.iter().enumerate().find(|&(_, v)| v >= &current);
    let down_index = down.iter().enumerate().find(|&(_, v)| v >= &current);

    let up = if let Some((index, _)) = up_index { y - index } else { up.len() };
    let left = if let Some((index, _)) = left_index { x - index } else { left.len() };
    let right = if let Some((index, _)) = right_index { index + 1 } else { right.len() };
    let down = if let Some((index, _)) = down_index { index + 1 } else { down.len() };

    up * left * right * down
}

pub fn process(input: &types::Data) -> usize {
    let mut max_scenic_score = 0;
    for y in 1..input.height - 1 {
        for x in 1..input.width - 1 {
            max_scenic_score = max_scenic_score.max(scenic_score(input, x, y))
        }
    }
    max_scenic_score
}
