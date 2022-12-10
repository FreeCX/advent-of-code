#[cfg(test)]
mod tests;

fn compress(data: &str) -> String {
    let mut result = String::new();
    let mut current = 0;
    let mut counter = 0;
    for (index, item) in data.chars().map(|x| x as u8 - 0x30).enumerate() {
        if index == 0 {
            current = item;
        }
        if item != current {
            result.push_str(&format!("{}{}", counter, current));
            current = item;
            counter = 0;
        }
        counter += 1;
    }
    result.push_str(&format!("{}{}", counter, current));
    result
}

fn look_and_say(start: &str, count: usize) -> usize {
    let mut current = start.to_string();
    for _ in 0..count {
        current = compress(&current);
    }
    current.len()
}

fn main() {
    let input = "3113322113";
    for iter in &[40_usize, 50_usize] {
        println!("input = {}; iter = {}; size = {}", input, iter, look_and_say(input, *iter));
    }
}
