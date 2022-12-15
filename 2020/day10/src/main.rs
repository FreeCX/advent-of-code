mod parse;
mod part_one;
mod part_two;
mod types;

#[cfg(test)]
mod tests;

fn main() {
    let input = parse::parse(include_str!("../data/input"));

    println!("Part One = {}", part_one::fold_one(part_one::process(&input)));
    println!("Part Two = {}", part_two::process(&input));
}
