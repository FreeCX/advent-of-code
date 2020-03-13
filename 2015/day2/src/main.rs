use std::cmp;
use std::io;
use std::io::prelude::BufRead;

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
    let (mut sum_one, mut sum_two) = (0, 0);
    let input = io::stdin();
    for line in input.lock().lines() {
        match line {
            Ok(value) => {
                let data: Vec<_> = value
                    .split("x")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();
                let (l, w, h) = (data[0], data[1], data[2]);
                sum_one += paper_one(l, w, h);
                sum_two += paper_two(l, w, h);
            }
            Err(why) => {
                println!("{}", why);
                break;
            }
        };
    }
    println!("[old] sum = {}", sum_one);
    println!("[new] sum = {}", sum_two);
}
