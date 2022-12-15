extern crate rand;

use std::collections::HashMap;

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
struct Mapper {
    map: HashMap<String, u8>,
    tmp: Vec<(u8, u8, u32)>,
    route: Vec<Vec<u32>>,
    counter: u8,
}

impl Mapper {
    fn new() -> Mapper {
        Mapper { map: HashMap::new(), tmp: Vec::new(), route: Vec::new(), counter: 0 }
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

fn main() {
    let buffer = include_str!("../data/input");

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
    let mut rnd = thread_rng();
    for _ in 0..100_000 {
        route.shuffle(&mut rnd);
        let now = get_len(&map, &route);
        if now < minimum {
            minimum = now;
        }
        if now > maximum {
            maximum = now;
        }
    }

    println!("min = {}\nmax = {}", minimum, maximum);
}
