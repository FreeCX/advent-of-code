use crate::game::Game;

pub fn process(games: &[Game]) -> u32 {
    let mut total = 0;

    for game in games {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for set in &game.set {
            max_red = max_red.max(set.red);
            max_green = max_green.max(set.green);
            max_blue = max_blue.max(set.blue);
        }

        total += max_red * max_green * max_blue;
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::part_two::process;

    #[test]
    fn example() {
        let games = parse::process(include_str!("../data/example"));
        assert_eq!(process(&games), 2286);
    }
}
