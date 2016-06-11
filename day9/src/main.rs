use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Mapper {
    map: HashMap<String, u8>,
    tmp: Vec<(u8, u8, u32)>,
    route: Vec<Vec<u32>>,
    counter: u8
}

impl Mapper {
    fn new() -> Mapper {
        Mapper { 
            map: HashMap::new(),
            tmp: Vec::new(),
            route: Vec::new(),
            counter: 0 
        }
    }
    fn add_pair(&mut self, a: &str, b: &str, v: u32) {
        let first = self.add(a);
        let second = self.add(b);
        self.tmp.push((first, second, v));
    }
    fn add(&mut self, value: &str) -> u8 {
        if !self.map.contains_key(value) {
            self.map.insert(value.to_owned(), self.counter);
            self.counter += 1;
            self.counter
        } else {
            *self.map.get(value).unwrap()
        }
    }
    fn precalc(&mut self) {
        let size: usize = self.map.len() + 1;
        self.route = vec![vec![0; size]; size];
        for &(a, b, v) in &self.tmp {
            self.route[a as usize][b as usize] = v;
        }
    }
}

fn main() {
    let mut file = File::open("./input.txt").expect("can't open file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer);
    let mut map = Mapper::new();
    for line in buffer.lines() {
        let raw: Vec<_> = line.split_whitespace().collect();
        let value = raw.last().unwrap().parse().unwrap();
        map.add_pair(raw[0], raw[2], value);
    }
    map.precalc();
    for row in map.route.into_iter() {
        for item in row {
            print!("{:3} ", item);
        }
        println!("");
    }
}
