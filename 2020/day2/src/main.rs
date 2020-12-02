use std::fs;

struct Rule {
    left_range: usize,
    right_range: usize,
    symbol: char,
    password: String
}

fn parse(line: &str) -> Rule {
    let parts: Vec<_> = line.split(" ").collect();
    let range: Vec<usize> = parts[0].split("-").map(|x| x.parse().unwrap()).collect();
    let symbol = parts[1].chars().nth(0).unwrap();
    Rule {
        left_range: range[0],
        right_range: range[1],
        symbol,
        password: parts[2].to_string()
    }
}

fn rule_one_check(rule: &Rule) -> bool {
    let contains = rule.password.chars().filter(|&x| x == rule.symbol).count();
    contains >= rule.left_range && contains <= rule.right_range
}

fn rule_two_check(rule: &Rule) -> bool {
    if rule.right_range > rule.password.len() {
        return false;
    }

    let chr_left = rule.password.chars().nth(rule.left_range - 1).unwrap();
    let chr_right = rule.password.chars().nth(rule.right_range - 1).unwrap();
    chr_left != chr_right && (chr_left == rule.symbol || chr_right == rule.symbol)
}

fn main() {
    let passwords = fs::read_to_string("input").unwrap();
    let mut first_counter = 0;
    let mut second_counter = 0;

    for item in passwords.lines() {
        let rule = parse(item);
        if rule_one_check(&rule) {
            first_counter += 1;
        }
        if rule_two_check(&rule) {
            second_counter += 1;
        }
    }

    println!(" first = {}", first_counter);
    println!("second = {}", second_counter);
}
