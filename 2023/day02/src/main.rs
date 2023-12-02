mod game;
mod parse;
mod part_one;
mod part_two;

#[cfg(test)]
mod tests;

fn main() {
    let games = parse::process(include_str!("../data/input"));

    println!("Part One = {}", part_one::process(&games));
    println!("Part Two = {}", part_two::process(&games));
}
