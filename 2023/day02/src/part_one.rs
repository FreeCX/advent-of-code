use crate::game::*;

pub fn process(games: &[Game]) -> u32 {
    let rule = CubeSet { red: 12, green: 13, blue: 14 };
    let mut result = 0;

    for game in games {
        if game.set.iter().all(|set| set.red <= rule.red && set.green <= rule.green && set.blue <= rule.blue) {
            result += game.id;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::part_one::process;

    #[test]
    fn example() {
        let games = parse::process(include_str!("../data/example"));
        assert_eq!(process(&games), 8);
    }
}
