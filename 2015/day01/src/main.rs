#[cfg(test)]
mod tests;

fn floor(string: &str) -> (i32, Option<usize>) {
    let mut floor: i32 = 0;
    let mut index: usize = 1;
    let mut not_catched = true;
    let mut problem_position = None;
    for item in string.chars() {
        match item {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => continue,
        }
        if floor == -1 && not_catched {
            problem_position = Some(index);
            not_catched = false;
        }
        index += 1;
    }
    (floor, problem_position)
}

fn main() {
    let buffer = include_str!("../data/input");
    println!("(floor, problem_at) = {:?}", floor(buffer));
}
