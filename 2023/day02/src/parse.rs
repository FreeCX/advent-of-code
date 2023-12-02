use crate::game::Game;

pub fn process(data: &str) -> Vec<Game> {
    let mut games = Vec::new();
    for line in data.lines() {
        games.push(Game::from(line));
    }
    games
}
