use std::collections::BTreeSet;

use crate::priority;
use crate::types::Compartment;

pub fn process(data: &[String]) -> u32 {
    let mut total = 0;
    for chunk in data.chunks(3) {
        let a: Compartment = chunk[0].chars().collect();
        let b: Compartment = chunk[1].chars().collect();
        let c: Compartment = chunk[2].chars().collect();
        let ab: Compartment = a.intersection(&b).cloned().collect();
        let intersection: BTreeSet<_> = ab.intersection(&c).collect();
        total += priority::priority(**intersection.first().unwrap());
    }
    total
}
