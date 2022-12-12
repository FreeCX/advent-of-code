use crate::types::Questions;

pub fn process(data: &Vec<Vec<Questions>>) -> usize {
    let mut count = 0;
    for group in data {
        let mut first = group[0].clone();
        for item in group.iter().skip(1) {
            first.retain(|e| item.contains(e));
        }
        count += first.len();
    }
    count
}
