use crate::types::Array;

fn differences(mut data: Array) -> Array {
    // first is zero
    data.insert(0, 0);
    // last is your adapter
    data.push(data.last().unwrap() + 3);
    data.iter()
        .skip(1)
        .zip(data.iter())
        .map(|(x, y)| x - y)
        .collect()
}

pub fn prepare<I: Into<Array>>(data: I) -> Array {
    let mut r = data.into();
    r.sort();
    differences(r)
}

pub fn parse(input: &str) -> Array {
    let mut data = input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Array>();
    data.sort();
    differences(data)
}
