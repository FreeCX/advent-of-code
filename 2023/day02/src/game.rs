#[derive(Debug, PartialEq, Eq)]
pub struct CubeSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    pub id: u32,
    pub set: Vec<CubeSet>,
}

impl From<&str> for Game {
    fn from(line: &str) -> Self {
        let (game_id, game_set) = line.split_once(':').unwrap();
        let (_, id) = game_id.split_once(' ').unwrap();
        Game { id: id.parse().unwrap(), set: game_set.split(';').map(CubeSet::from).collect() }
    }
}

impl From<&str> for CubeSet {
    fn from(line: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for item in line.trim().split(',') {
            let (count, color) = item.trim().split_once(' ').unwrap();
            let count: u32 = count.parse().unwrap();
            match color {
                "red" => red += count,
                "green" => green += count,
                "blue" => blue += count,
                other => panic!("color `{other}` do not support!"),
            };
        }

        CubeSet { red, green, blue }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::*;

    #[test]
    fn parse() {
        let parsed = Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let real = Game {
            id: 1,
            set: vec![
                CubeSet { red: 4, green: 0, blue: 3 },
                CubeSet { red: 1, green: 2, blue: 6 },
                CubeSet { red: 0, green: 2, blue: 0 },
            ],
        };
        assert_eq!(parsed, real);
    }
}
