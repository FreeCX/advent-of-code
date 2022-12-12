mod parse;
mod part_one;
mod part_two;

#[cfg(test)]
mod tests;

fn main() {
    let header_size = 25;
    let input = parse::parse(include_str!("../data/input"));

    println!(" first = {}", part_one::process(&input, header_size));
    println!("second = {}", part_two::process(&input, header_size));
}
