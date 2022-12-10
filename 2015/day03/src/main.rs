use std::collections::HashSet;

#[cfg(test)]
mod tests;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    fn new(x: i32, y: i32) -> Coords {
        Coords { x, y }
    }
}

fn house_counter(string: &str) -> usize {
    let mut set = HashSet::new();
    let mut pos = Coords::new(0, 0);
    if !string.is_empty() {
        set.insert(pos);
    } else {
        return 0;
    }
    for item in string.chars() {
        let (x, y) = get_move(item);
        pos.x += x as i32;
        pos.y += y as i32;
        set.insert(pos);
    }
    set.len()
}

fn get_move(dir: char) -> (i8, i8) {
    match dir {
        '>' => (1, 0),
        '<' => (-1, 0),
        '^' => (0, -1),
        'v' => (0, 1),
        _ => (0, 0),
    }
}

fn with_robot(string: &str) -> usize {
    let mut s1 = HashSet::new();
    let mut s2 = HashSet::new();
    let mut p1 = Coords::new(0, 0);
    let mut p2 = Coords::new(0, 0);
    if !string.is_empty() {
        s1.insert(p1);
        s2.insert(p2);
    } else {
        return 0;
    }
    let mut seq = string.chars();
    loop {
        let (a, b) = (seq.next(), seq.next());
        if a.is_none() {
            break;
        }
        let (x, y) = get_move(a.unwrap());
        p1.x += x as i32;
        p1.y += y as i32;
        s1.insert(p1);
        if b.is_none() {
            continue;
        }
        let (x, y) = get_move(b.unwrap());
        p2.x += x as i32;
        p2.y += y as i32;
        s2.insert(p2);
    }
    let result: HashSet<_> = s1.union(&s2).cloned().collect();
    result.len()
}

fn main() {
    let buffer = include_str!("../data/input");

    println!("part one = {}", house_counter(buffer));
    println!("part two = {}", with_robot(buffer));
}
