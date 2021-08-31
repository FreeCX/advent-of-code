fn has_sum(data: &[i64], item: i64) -> bool {
    let mut count = 0;
    for a in data {
        count += 1;
        for b in data.iter().skip(count) {
            if a + b == item {
                return true;
            }
        }
    }
    false
}

fn task_one(n: &[i64], header: usize) -> i64 {
    let mut pointer = header;
    for item in n.iter().skip(header) {
        let window = &n[pointer - header..pointer];
        if !has_sum(&window, *item) {
            return *item;
        }
        pointer += 1;
    }
    unreachable!()
}

fn task_two(n: &[i64], header: usize) -> i64 {
    let invalid = task_one(n, header);
    let size = n.len();

    for i in 0..size {
        'inner: for j in i..size {
            let window = &n[i..j];
            let wsum: i64 = window.iter().sum();
            if wsum > invalid {
                break 'inner;
            }
            if wsum == invalid {
                let minv = window.iter().min().unwrap();
                let maxv = window.iter().max().unwrap();
                return minv + maxv;
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::*;

    const N: &[i64] = &[35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];

    #[test]
    fn example_one() {
        assert_eq!(task_one(N, 5), 127);
    }

    #[test]
    fn example_one_extra() {
        let mut n: Vec<i64> = (1..=25).collect();
        n.extend_from_slice(&[26, 49, 100, 50]);
        assert_eq!(task_one(&n, 25), 100);
    }

    #[test]
    fn example_two() {
        assert_eq!(task_two(N, 5), 62);
    }
}

fn main() {
    let input: Vec<i64> = include_str!("../input").lines().map(|x| x.parse::<i64>().unwrap()).collect();
    let header_size = 25;

    println!(" first = {}", task_one(&input, header_size));
    println!("second = {}", task_two(&input, header_size));
}
