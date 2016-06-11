use std::collections::HashSet;
use std::io::prelude::Read;
use std::fs::File;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    fn new(x: i32, y: i32) -> Coords {
        Coords { x: x, y: y }
    }
}

fn house_counter(string: &str) -> usize {
    let mut set = HashSet::new();
    let mut pos = Coords::new(0, 0);
    if string.len() > 0 {
        set.insert(pos);
    } else {
        return 0;
    }
    for item in string.chars() {
        match item {
            '>' => pos.x += 1,
            '<' => pos.x -= 1,
            '^' => pos.y -= 1,
            'v' => pos.y += 1,
            _ => continue,
        };
        set.insert(pos);
    }
    set.len()
}

#[test]
fn examples() {
    assert!(house_counter(">") == 2);
    assert!(house_counter("^>v<") == 4);
    assert!(house_counter("^v^v^v^v^v") == 2);
}

fn main() {
    let mut f = File::open("input.txt").expect("[error] can't open file!");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("[error] can't read from file!");
    println!("houses = {}", house_counter(&buffer));
}
