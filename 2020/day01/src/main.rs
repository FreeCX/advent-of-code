mod part_one;
mod part_two;
mod prepare;

#[cfg(test)]
mod tests;

fn main() {
    let value = 2020;
    let items = prepare::process(value, include_str!("../data/input"));

    let part_one = part_one::process(value, &items).unwrap();
    println!("Part One = {part_one}");

    let part_two = part_two::process(value, &items).unwrap();
    println!("Part Two = {part_two}");
}
