extern crate rand;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use rand::{thread_rng, Rng};

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
            self.counter - 1
        } else {
            *self.map.get(value).unwrap()
        }
    }
    fn precalc(&mut self) {
        let size: usize = self.map.len();
        self.route = vec![vec![0; size]; size];
        for &(a, b, v) in &self.tmp {
            self.route[a as usize][b as usize] = v;
            self.route[b as usize][a as usize] = v;
        }
    }
}

fn get_len(map: &Mapper, route: &[u32]) -> u32 {
    let mut result = 0;
    let tmp = &route[1..];
    for (&a, &b) in route.iter().zip(tmp.iter()) {
        result += map.route[a as usize][b as usize];
    }
    result
}

fn print_matrix(map: &Mapper) {
    for row in map.route.iter() {
        for item in row {
            print!("{:3}, ", item);
        }
        println!("");
    }
}

fn main() {
    let mut file = File::open("./input.txt").expect("can't open file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("can't read from file");
    let mut map = Mapper::new();
    for line in buffer.lines() {
        let raw: Vec<_> = line.split_whitespace().collect();
        let value = raw.last().unwrap().parse().unwrap();
        map.add_pair(raw[0], raw[2], value);
    }
    map.precalc();
    let route_c = map.route.len() as u32;
    let mut route: Vec<_> = (0..route_c).collect();
    let mut minimum = get_len(&map, &route);
    let mut maximum = minimum;
    for _ in 0..100_000 {
        thread_rng().shuffle(&mut route);
        let now = get_len(&map, &route);
        if now < minimum {
            minimum = now;
        }
        if now > maximum {
            maximum = now;
        }
    }
    println!("min = {}; max = {}", minimum, maximum);
}