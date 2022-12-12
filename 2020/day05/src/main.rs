mod part_one;
mod part_two;
mod seat;

#[cfg(test)]
mod tests;

fn main() {
    let tickets: Vec<_> = include_str!("../data/input").lines().collect();

    println!("Part One = {}", part_one::process(&tickets));
    println!("Part Two = {}", part_two::process(&tickets));
}
