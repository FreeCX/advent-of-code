use crate::types::{App, Command, Ship};

pub fn process(ship: &Ship, app: &App) -> String {
    let mut ship = ship.clone();

    for Command { count, from, to } in app {
        for _ in 0..*count {
            // not best way to do this
            let item = ship[(from - 1) as usize].pop().unwrap();
            ship[(to - 1) as usize].push(item);
        }
    }

    // take last element from each stack and concatenate it
    ship.iter_mut().map(|x| x.pop().unwrap()).collect()
}
