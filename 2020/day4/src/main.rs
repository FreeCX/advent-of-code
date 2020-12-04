#![allow(dead_code, unused_variables)]
use std::fs;

#[derive(Debug)]
enum Unit {
    Centimer,
    Inch,
}

type Height = (u16, Unit);

#[derive(Debug)]
struct Id {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<Height>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Id {
    fn new() -> Id {
        Id { byr: None, iyr: None, eyr: None, hgt: None, hcl: None, ecl: None, pid: None, cid: None }
    }

    fn set(&mut self, data: &str) {
        let kv: Vec<_> = data.split(":").collect();
        match kv[0] {
            "byr" => self.byr = kv[1].parse().ok(),
            "iyr" => self.iyr = kv[1].parse().ok(),
            "eyr" => self.eyr = kv[1].parse().ok(),
            "hgt" => {
                let size = kv[1].len();
                self.hgt = None;

                if size == 0 {
                    return;
                }
                let unit = match &kv[1][size - 2..] {
                    "in" => Unit::Inch,
                    "cm" => Unit::Centimer,
                    _ => return,
                };
                let value: u16 = kv[1][..size - 2].parse().unwrap();
                self.hgt = Some((value, unit));
            }
            "hcl" => self.hcl = Some(kv[1].to_owned()),
            "ecl" => self.ecl = Some(kv[1].to_owned()),
            "pid" => self.pid = Some(kv[1].to_owned()),
            "cid" => self.cid = Some(kv[1].to_owned()),
            _ => panic!("bad key"),
        }
    }

    fn is_valid_one(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid_two(&self) -> bool {
        self.byr >= Some(1920)
            && self.byr <= Some(2002)
            && self.iyr >= Some(2010)
            && self.iyr <= Some(2020)
            && self.eyr >= Some(2020)
            && self.eyr <= Some(2030)
            && validate_height(&self.hgt)
            && validate_color(&self.hcl)
            && validate_eye(&self.ecl)
            && validate_id(&self.pid)
    }
}

fn validate_height(height: &Option<Height>) -> bool {
    if height.is_some() {
        let h = height.as_ref().unwrap();
        match h.1 {
            Unit::Centimer => return h.0 >= 150 && h.0 <= 193,
            Unit::Inch => return h.0 >= 59 && h.0 <= 76,
        }
    }
    false
}

fn validate_color(color: &Option<String>) -> bool {
    if color.is_some() {
        let v = color.as_ref().unwrap();
        if v.len() != 7 || v.chars().nth(0) != Some('#') {
            return false;
        }
        for i in v.chars().skip(1) {
            if !((i >= 0x30 as char && i <= 0x39 as char) || (i >= 'a' && i <= 'f')) {
                return false;
            }
        }
        true
    } else {
        false
    }
}

fn validate_eye(eye: &Option<String>) -> bool {
    if eye.is_some() {
        let colors: Vec<String> =
            vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().map(|&x| x.to_owned()).collect();
        let v = eye.clone().unwrap();
        colors.contains(&v)
    } else {
        false
    }
}

fn validate_id(id: &Option<String>) -> bool {
    if id.is_some() {
        let v = id.as_ref().unwrap();
        if v.len() != 9 {
            return false;
        }
        for i in v.chars() {
            if !(i >= 0x30 as char && i <= 0x39 as char) {
                return false;
            }
        }
    }
    true
}

fn parse(data: String) -> Vec<Id> {
    let mut result = Vec::new();

    for block in data.split("\n\n") {
        let mut ident = Id::new();
        let block = block.replace("\n", " ");
        for item in block.split(" ") {
            if item.len() > 0 {
                ident.set(item);
            }
        }
        result.push(ident);
    }

    result
}

fn task_one(ids: &Vec<Id>) -> u32 {
    let mut valid_count = 0;

    for id in ids {
        if id.is_valid_one() {
            valid_count += 1;
        }
    }

    valid_count
}

fn task_two(ids: &Vec<Id>) -> u32 {
    let mut valid_count = 0;

    for id in ids {
        if id.is_valid_one() && id.is_valid_two() {
            valid_count += 1;
        }
    }

    valid_count
}

fn main() {
    let ids = parse(fs::read_to_string("input").unwrap());

    println!(" first = {}", task_one(&ids));
    println!("second = {}", task_two(&ids));
}
