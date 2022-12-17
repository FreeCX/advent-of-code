use crate::types::{App, Command, Ship};

pub fn process(ship: &Ship, app: &App) -> String {
    let mut ship = ship.clone();

    for Command { count, from, to } in app {
        let index = ship[(from - 1) as usize].len() - *count as usize;
        let items = ship[(from - 1) as usize].split_off(index);
        ship[(to - 1) as usize].extend_from_slice(&items);
    }

    ship.iter_mut().map(|x| x.pop().unwrap()).collect()
}
