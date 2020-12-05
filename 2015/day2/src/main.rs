use std::cmp;
use std::fs;

fn max3(a: i32, b: i32, c: i32) -> i32 {
    cmp::max(cmp::max(a, b), c)
}

fn paper_one(l: i32, w: i32, h: i32) -> i32 {
    2 * l * w + 2 * w * h + 2 * h * l + (l * w * h / max3(l, w, h))
}

fn paper_two(a: i32, b: i32, c: i32) -> i32 {
    let max = max3(a, b, c);
    let min = cmp::min(cmp::min(a, b), c);
    let x = a + b + c - max - min;
    a * b * c + 2 * (x + min)
}

#[test]
fn examples() {
    assert!(paper_one(2, 3, 4) == 58);
    assert!(paper_one(1, 1, 10) == 43);
    assert!(paper_two(2, 3, 4) == 34);
    assert!(paper_two(1, 1, 10) == 14);
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    
    let (mut sum_one, mut sum_two) = (0, 0);
    for line in input.lines() {
        let data: Vec<_> = line.split("x").map(|x| x.parse::<i32>().unwrap()).collect();
        let (l, w, h) = (data[0], data[1], data[2]);
        sum_one += paper_one(l, w, h);
        sum_two += paper_two(l, w, h);
    }
    
    println!("[ first] sum = {}", sum_one);
    println!("[second] sum = {}", sum_two);
}
