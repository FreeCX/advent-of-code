mod parse;
mod types;
mod part_one;
mod part_two;

#[cfg(test)]
mod tests;

fn main() {
    let data = parse::parse(include_str!("../data/input"));

    println!("Part One = {}", part_one::process(&data));
    println!("Part Two = {}", part_two::process(&data));
}
