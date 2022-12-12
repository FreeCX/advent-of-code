use crate::parse;

pub fn rule_one_check(rule: &parse::Rule) -> bool {
    let contains = rule.password.chars().filter(|&x| x == rule.symbol).count();
    contains >= rule.left_range && contains <= rule.right_range
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::part_one::*;

    #[test]
    fn default() {
        let mut counter = 0;
        for item in include_str!("../data/example").lines() {
            if rule_one_check(&parse::parse(item)) {
                counter += 1;
            }
        }
        assert_eq!(counter, 2);
    }
}
