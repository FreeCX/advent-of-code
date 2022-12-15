use std::collections::BTreeSet;

use crate::priority;
use crate::types::Compartment;

pub fn process(data: &[String]) -> u32 {
    let mut total = 0;
    for line in data {
        let (a, b) = line.split_at(line.len() / 2);
        let a: Compartment = a.chars().collect();
        let b: Compartment = b.chars().collect();
        let intersection: BTreeSet<_> = a.intersection(&b).collect();
        total += priority::priority(**intersection.first().unwrap());
    }
    total
}
