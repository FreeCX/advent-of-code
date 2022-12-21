mod parse;
mod part_one;
mod part_two;
mod types;

#[cfg(test)]
mod tests;

fn main() {
    let bags = parse::parse(include_str!("../data/input"));

    println!("Part One = {}", part_one::process(&bags));
    println!("Part Two = {}", part_two::process(&bags));
}
