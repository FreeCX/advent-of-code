use crate::part_one;

pub fn process(n: &[i64], header: usize) -> i64 {
    let invalid = part_one::process(n, header);
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
