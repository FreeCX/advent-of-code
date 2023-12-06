mod almanac;
mod part_one;
mod part_two;

#[cfg(test)]
mod tests;

fn main() {
    let almanac = almanac::Almanac::from(include_str!("../data/input"));

    println!("Part One = {}", part_one::process(&almanac));
    println!("Part Two = {}", part_two::process(&almanac));
}
