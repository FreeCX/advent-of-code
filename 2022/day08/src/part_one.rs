use crate::types;

pub fn process(input: &types::Data) -> u16 {
    let mut total_count = 2 * (input.width as u16 + input.height as u16 - 2);

    for y in 1..input.height - 1 {
        for x in 1..input.width - 1 {
            let current = input.get(x, y);
            let up = (0..y).into_iter().map(|y| input.get(x, y)).max().unwrap_or(0);
            let left = (0..x).into_iter().map(|x| input.get(x, y)).max().unwrap_or(0);
            let right = (x + 1..input.width).into_iter().map(|x| input.get(x, y)).max().unwrap_or(0);
            let down = (y + 1..input.height).into_iter().map(|y| input.get(x, y)).max().unwrap_or(0);
            if current > up || current > left || current > right || current > down {
                total_count += 1;
            }
        }
    }

    total_count
}
