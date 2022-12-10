pub mod part_one;
pub mod part_two;

fn main() {
    let input = include_str!("../data/input");

    println!("Part One = {}", part_one::process(input));
    println!("Part Two = {}", part_two::process(input));
}
