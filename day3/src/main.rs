use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::Read;

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
    if string.len() > 0 {
        s1.insert(p1);
        s2.insert(p2);
    } else {
        return 0;
    }
    let mut seq = string.chars();
    loop {
        let (a, b) = (seq.next(), seq.next());
        if a == None {
            break;
        }
        let (x, y) = get_move(a.unwrap());
        p1.x += x as i32;
        p1.y += y as i32;
        s1.insert(p1);
        if b == None {
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

#[test]
fn examples() {
    let one_str = "^v";
    let two_str = "^>v<";
    let thr_str = "^v^v^v^v^v";
    assert!(house_counter(one_str) == 2);
    assert!(house_counter(two_str) == 4);
    assert!(house_counter(thr_str) == 2);
    assert!(with_robot(one_str) == 3);
    assert!(with_robot(two_str) == 3);
    assert!(with_robot(thr_str) == 11);
}

fn main() {
    let mut f = File::open("input.txt").expect("[error] can't open file!");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)
        .expect("[error] can't read from file!");
    println!("part one = {}", house_counter(&buffer));
    println!("part two = {}", with_robot(&buffer));
}
