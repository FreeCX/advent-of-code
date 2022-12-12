use crate::types::*;

pub fn process(slope: Pos, land: &Land) -> usize {
    let mut pos = slope;
    let mut counter = 0;

    while pos.1 < land.height {
        if land.data[pos.1][pos.0 % land.width] == Field::Tree {
            counter += 1;
        }
        pos.0 += slope.0;
        pos.1 += slope.1;
    }

    counter
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::part_one;

    #[test]
    fn default() {
        let land = parse::parse(include_str!("../data/example"));
        assert_eq!(part_one::process((3, 1), &land), 7);
    }
}
