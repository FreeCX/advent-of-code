use std::io::prelude::Read;
use std::fs::File;

fn main() {
    let mut f = File::open("input.txt").expect("[error] can't open file!");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("[error] can't read from file!");
    // input code
}