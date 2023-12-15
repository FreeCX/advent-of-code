use crate::document::Document;
use crate::part_one::win_count;

pub fn process(doc: &Document) -> u64 {
    win_count(doc.time_one, doc.dist_one)
}

#[cfg(test)]
mod tests {
    use crate::document::Document;
    use crate::part_two::process;

    #[test]
    fn example() {
        let doc = Document::from(include_str!("../data/example"));
        assert_eq!(process(&doc), 71503);
    }
}
