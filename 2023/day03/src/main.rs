mod part_one;
mod part_two;
mod schema;

#[cfg(test)]
mod tests;

fn main() {
    let schema = schema::Schema::from(include_str!("../data/input"));

    println!("Part One = {}", part_one::process(&schema));
    println!("Part Two = {}", part_two::process(&schema));
}
