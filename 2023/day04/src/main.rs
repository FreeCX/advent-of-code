mod card;
mod parse;
mod part_one;
mod part_two;

#[cfg(test)]
mod tests;

fn main() {
    let cards = parse::process(include_str!("../data/input"));

    println!("Part One = {}", part_one::process(&cards));
    println!("Part Two = {}", part_two::process(&cards));
}
