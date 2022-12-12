use crate::types::{Array, Diffs};

pub fn fold_one(data: Diffs) -> u16 {
    data.get(&1).unwrap() * data.get(&3).unwrap()
}

pub fn process(data: &Array) -> Diffs {
    let increment = |data: &mut Diffs, key| *data.entry(key).or_insert(0) += 1;
    let mut result = Diffs::new();

    for item in data {
        increment(&mut result, *item);
    }

    result
}
