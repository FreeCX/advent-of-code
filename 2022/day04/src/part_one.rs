use crate::types::{Assignment, Pair};

fn contains(a: &Assignment, b: &Assignment) -> bool {
    a.left >= b.left && a.right <= b.right || b.left >= a.left && b.right <= a.right
}

pub fn process(input: &[Pair]) -> u16 {
    let mut result = 0;
    for Pair { left, right } in input {
        if contains(left, right) {
            result += 1;
        }
    }
    result
}
