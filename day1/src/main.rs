use std::io::prelude::Read;
use std::fs::File;

fn floor(string: &str) -> (i32, Option<usize>) {
    let mut floor: i32 = 0;
    let mut index: usize = 1;
    let mut not_catched = true;
    let mut problem_position = None;
    for item in string.chars() {
        match item {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => continue,
        }
        if floor == -1 && not_catched {
            problem_position = Some(index);
            not_catched = false;
        }
        index += 1;
    }
    (floor, problem_position)
}

#[test]
fn examples() {
    assert!(floor("(())") == (0, None) && floor("()()") == (0, None));
    assert!(floor("(((") == (3, None) && floor("(()(()(") == (3, None));
    assert!(floor("))(((((") == (3, Some(1)));
    assert!(floor("())") == (-1, Some(3)) && floor("))(") == (-1, Some(1)));
    assert!(floor(")))") == (-3, Some(1)) && floor(")())())") == (-3, Some(1)));
}

fn main() {
    let mut f = File::open("input.txt").expect("[error] can't open file!");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("[error] can't read from file!");
    println!("(floor, problem_at) = {:?}", floor(&buffer));
}
