use crate::document::Document;

pub fn win_count(time: u64, dist: u64) -> u64 {
    let mut count = 0;
    for hold in 0..time {
        if (time - hold) * hold > dist {
            count += 1;
        }
    }
    count
}

pub fn process(doc: &Document) -> u64 {
    let mut result = 1;

    for (&time, &dist) in doc.time.iter().zip(doc.dist.iter()) {
        let count = win_count(time, dist);
        if count > 0 {
            result *= count;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::document::Document;
    use crate::part_one::process;

    #[test]
    fn example() {
        let doc = Document::from(include_str!("../data/example"));
        assert_eq!(process(&doc), 288);
    }
}
