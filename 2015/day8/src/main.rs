use std::fs;

fn analyze(string: &str) -> usize {
    let tmp = string[1..string.len()-1].replace("\\\"", "\"")
                                       .replace("\\\\", "\\");
    let mut result = 0;
    let mut first_flag = false;
    for f in tmp.split("\\x") {
        result += if !first_flag {
            first_flag = true;
            f.len()
        } else {
            (&f[2..]).len() + 1
        }
    }
    result
}

#[test]
fn example() {
    let mut result = 0;
    let mut sum_len = 0;
    let data = vec!["\"\"", "\"abc\"", "\"aaa\\\"aaa\"", "\"\\x27\""];
    for item in data {
        sum_len += item.len();
        result += analyze(item);
    }
    assert_eq!(sum_len - result, 12);
}

fn main() {
    // all-string = 6310
    //  in-memory = 4977     <--- find bug (returned 4968)
    //     result = 1333
    let buffer = fs::read_to_string("input").unwrap();

    let mut result = 0;
    let mut sum_len = 0;
    for line in buffer.lines() {
        sum_len += line.len();
        result += analyze(&line);
    }

    println!("all-string = {}", sum_len);
    println!(" in-memory = {}", result);
    println!("    result = {}", sum_len - result);
}
