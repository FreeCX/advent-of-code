use std::collections::HashMap;

type Array = Vec<u16>;
type Tuple = Vec<(u16, u16)>;
type Diffs = HashMap<u16, u16>;

fn differences(mut data: Array) -> Array {
    // first is zero
    data.insert(0, 0);
    // last is your adapter
    data.push(data.last().unwrap() + 3);
    data.iter().skip(1).zip(data.iter()).map(|(x, y)| x - y).collect()
}

fn prepare<I: Into<Array>>(data: I) -> Array {
    let mut r = data.into();
    r.sort();
    differences(r)
}

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

fn task_one(data: &Array) -> Diffs {
    let increment = |data: &mut Diffs, key| *data.entry(key).or_insert(0) += 1;
    let mut result = Diffs::new();

    for item in data {
        increment(&mut result, *item);
    }

    result
}

// thanks reddit
fn task_two(data: &Array) -> i64 {
    groupby(data)
        .iter()
        .filter(|(k, _)| *k == 1)
        .map(|(_, v)| *v as u32)
        .fold(1, |acc, n| acc * (2_i64.pow(n - 1) - (n == 4) as i64))
}

#[cfg(test)]
mod tests {
    use crate::*;

    const E1: &[u16] = &[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    const E2: &[u16] = &[
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34,
        10, 3,
    ];

    #[test]
    fn example_11() {
        let r: Diffs = vec![(1, 7), (3, 5)].into_iter().collect();
        let n: Array = prepare(E1);
        assert_eq!(task_one(&n), r);
    }

    #[test]
    fn example_12() {
        let r: Diffs = vec![(1, 22), (3, 10)].into_iter().collect();
        let n = prepare(E2);
        assert_eq!(task_one(&n), r);
    }

    #[test]
    fn example_21() {
        let n = prepare(E1);
        assert_eq!(task_two(&n), 8);
    }

    #[test]
    fn example_22() {
        let n = prepare(E2);
        assert_eq!(task_two(&n), 19208);
    }
}

fn main() {
    let fold_one = |data: Diffs| data.get(&1).unwrap() * data.get(&3).unwrap();
    let input = prepare(include_str!("../input").lines().map(|x| x.parse().unwrap()).collect::<Array>());

    println!(" first = {}", fold_one(task_one(&input)));
    println!("second = {}", task_two(&input));
}
