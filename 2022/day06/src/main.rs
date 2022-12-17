mod part_one;
mod part_two;

#[cfg(test)]
mod tests;

fn main() {
    let input = include_str!("../data/input").as_bytes();

    println!("Part One = {}", part_one::process(input));
    println!("Part Two = {}", part_two::process(input));
}
