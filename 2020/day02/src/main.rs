mod parse;
mod part_one;
mod part_two;

#[cfg(test)]
mod tests;

fn main() {
    let passwords = include_str!("../data/input");
    let mut first_counter = 0;
    let mut second_counter = 0;

    for item in passwords.lines() {
        let rule = parse::parse(item);
        if part_one::rule_one_check(&rule) {
            first_counter += 1;
        }
        if part_two::rule_two_check(&rule) {
            second_counter += 1;
        }
    }

    println!("Part One = {}", first_counter);
    println!("Part Two = {}", second_counter);
}
