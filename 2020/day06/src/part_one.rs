use crate::types::Questions;

pub fn process(data: &Vec<Vec<Questions>>) -> usize {
    let mut count = 0;
    for group in data {
        let mut q = Questions::new();
        for user in group {
            for item in user {
                q.insert(*item);
            }
        }
        count += q.len();
    }
    count
}
