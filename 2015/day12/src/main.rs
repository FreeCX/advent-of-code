extern crate json;

use std::fs;

fn json_sum_one(data: &json::JsonValue) -> i32 {
    match data {
        json::JsonValue::Number(_) => data.as_i32().unwrap(),
        json::JsonValue::Object(obj) => obj.iter().map(|(_, v)| json_sum_one(&v)).sum(),
        json::JsonValue::Array(array) => array.iter().map(|x| json_sum_one(x)).sum(),
        _ => 0,
    }
}

fn json_sum_two(data: &json::JsonValue) -> i32 {
    match data {
        json::JsonValue::Number(_) => data.as_i32().unwrap(),
        json::JsonValue::Object(obj) => {
            let mut result = 0;
            for (_, v) in obj.iter() {
                match v {
                    json::JsonValue::Short(s) => if s == "red" { return 0; },
                    _ => result += json_sum_two(&v),
                };
            }
            result
        }
        json::JsonValue::Array(array) => array.iter().map(|x| json_sum_two(x)).sum(),
        _ => 0,
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("can't read `input` file");
    let data = json::parse(&input).unwrap();
    println!("sum_one = {}", json_sum_one(&data));
    println!("sum_two = {}", json_sum_two(&data));
}
