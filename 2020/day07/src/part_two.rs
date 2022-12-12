use crate::types::BagOfBugs;

fn inner(name: &str, bags: &BagOfBugs) -> usize {
    match bags.get(name) {
        Some(items) => {
            if items.is_empty() {
                return 0;
            }
            let mut result = 0;
            for (count, item) in items {
                result += count * (1 + inner(item, bags));
            }
            result
        }
        None => 0,
    }
}

pub fn process(bags: &BagOfBugs) -> usize {
    inner("shiny gold", bags)
}
