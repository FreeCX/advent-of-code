mod parse;
mod part_one;
mod part_two;
mod types;

#[cfg(test)]
mod tests;

fn main() {
    let land = parse::parse(include_str!("../data/input"));

    println!("Part One = {}", part_one::process((3, 1), &land));
    println!("Part Two = {}", part_two::process(&land));
}
