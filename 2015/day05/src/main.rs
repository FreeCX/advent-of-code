#[cfg(test)]
mod tests;

fn checker_one(line: &str) -> bool {
    fn rule_two(line: &str) -> bool {
        let mut last_char = line.chars().next().unwrap();
        for curr_char in line.chars().skip(1) {
            if curr_char == last_char {
                return true;
            }
            last_char = curr_char;
        }
        false
    }
    fn rule_three(line: &str) -> bool {
        let items = ["ab", "cd", "pq", "xy"];
        for item in items.iter() {
            if line.contains(item) {
                return false;
            }
        }
        true
    }
    let rule_one = line.chars().filter(|&x| "aeiou".find(x).is_some()).count() >= 3;
    rule_one && rule_two(line) && rule_three(line)
}

fn nice_count_one(buffer: &str) -> usize {
    let mut nice_count = 0_usize;
    for line in buffer.lines() {
        if checker_one(line) {
            nice_count += 1;
        }
    }
    nice_count
}

fn nice_count_two(_buffer: &str) -> usize {
    todo!()
}

fn main() {
    let buffer = include_str!("../data/input");

    println!("[1] nice string = {}", nice_count_one(buffer));
    println!("[2] nice string = {}", nice_count_two(buffer));
}
