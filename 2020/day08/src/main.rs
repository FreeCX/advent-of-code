mod emulator;
mod part_one;
mod part_two;

#[cfg(test)]
mod tests;

fn main() {
    let app = include_str!("../data/input");

    println!("Part One = {}", part_one::process(app));
    println!("Part Two = {}", part_two::process(app));
}
