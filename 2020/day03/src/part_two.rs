use crate::part_one;
use crate::types::*;

pub fn process(land: &Land) -> usize {
    let r11 = part_one::process((1, 1), land);
    let r31 = part_one::process((3, 1), land);
    let r51 = part_one::process((5, 1), land);
    let r71 = part_one::process((7, 1), land);
    let r12 = part_one::process((1, 2), land);

    r11 * r31 * r51 * r71 * r12
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::part_two;

    #[test]
    fn default() {
        let land = parse::parse(include_str!("../data/example"));
        assert_eq!(part_two::process(&land), 336);
    }
}
