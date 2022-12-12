use crate::types::{Array, Tuple};

fn groupby(data: &Array) -> Tuple {
    let mut result = Tuple::new();
    let mut current = data[0];
    let mut count = 1;
    for item in data.iter().skip(1) {
        if current != *item {
            result.push((current, count));
            current = *item;
            count = 1;
        } else {
            count += 1;
        }
    }
    result.push((current, count));
    result
}

// thanks reddit
pub fn process(data: &Array) -> i64 {
    groupby(data)
        .iter()
        .filter(|(k, _)| *k == 1)
        .map(|(_, v)| *v as u32)
        .fold(1, |acc, n| acc * (2_i64.pow(n - 1) - (n == 4) as i64))
}
