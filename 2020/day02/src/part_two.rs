use crate::parse;

pub fn rule_two_check(rule: &parse::Rule) -> bool {
    if rule.right_range > rule.password.len() {
        return false;
    }

    let chr_left = rule.password.chars().nth(rule.left_range - 1).unwrap();
    let chr_right = rule.password.chars().nth(rule.right_range - 1).unwrap();
    chr_left != chr_right && (chr_left == rule.symbol || chr_right == rule.symbol)
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::part_two::*;

    #[test]
    fn default() {
        let mut counter = 0;
        for item in include_str!("../data/example").lines() {
            if rule_two_check(&parse::parse(item)) {
                counter += 1;
            }
        }
        assert_eq!(counter, 1);
    }
}
