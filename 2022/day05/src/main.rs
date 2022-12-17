mod parse;
mod part_one;
mod part_two;
mod types;

#[cfg(test)]
mod tests;

fn main() {
    let (ship, app) = parse::parse(include_str!("../data/input"));

    println!("Part One = {}", part_one::process(&ship, &app));
    println!("Part Two = {}", part_two::process(&ship, &app));
}
