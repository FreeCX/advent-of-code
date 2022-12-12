use crate::types::BagOfBugs;

fn contain(from: &str, what: &str, bags: &BagOfBugs) -> bool {
    match bags.get(from) {
        Some(items) => {
            if items.is_empty() {
                return false;
            }
            for (_, item) in items {
                if item == what || contain(item, what, bags) {
                    return true;
                }
            }
            false
        }
        None => false,
    }
}

pub fn process(bags: &BagOfBugs) -> usize {
    let mut count = 0;
    for bag in bags.keys() {
        if contain(bag, "shiny gold", bags) {
            count += 1;
        }
    }
    count
}
