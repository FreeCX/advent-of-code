mod document;
mod part_one;
mod part_two;

#[cfg(test)]
mod tests;

fn main() {
    let doc = document::Document::from(include_str!("../data/input"));

    println!("Part One = {}", part_one::process(&doc));
    println!("Part Two = {}", part_two::process(&doc));
}
