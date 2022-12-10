extern crate json;

fn json_sum_one(data: &json::JsonValue) -> i32 {
    match data {
        json::JsonValue::Number(_) => data.as_i32().unwrap(),
        json::JsonValue::Object(obj) => obj.iter().map(|(_, v)| json_sum_one(&v)).sum(),
        json::JsonValue::Array(array) => array.iter().map(json_sum_one).sum(),
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
                    json::JsonValue::Short(s) => {
                        if s == "red" {
                            return 0;
                        }
                    }
                    _ => result += json_sum_two(v),
                };
            }
            result
        }
        json::JsonValue::Array(array) => array.iter().map(json_sum_two).sum(),
        _ => 0,
    }
}

fn main() {
    let input = include_str!("../data/input");
    let data = json::parse(input).unwrap();
    println!("sum_one = {}", json_sum_one(&data));
    println!("sum_two = {}", json_sum_two(&data));
}
