use std::collections::HashSet;
use std::fs;

type Questions = HashSet<char>;

fn parse(data: String) -> Vec<Vec<Questions>> {
    let mut result = Vec::new();
    for group in data.split("\n\n") {
        let mut question_group = Vec::new();
        for passenger in group.split("\n") {
            let mut questions = Questions::new();
            for question in passenger.chars() {
                if question != ' ' {
                    questions.insert(question);
                }
            }
            question_group.push(questions);
        }
        result.push(question_group);
    }
    result
}

fn task_one(data: &Vec<Vec<Questions>>) -> usize {
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

fn task_two(data: &Vec<Vec<Questions>>) -> usize {
    let mut count = 0;
    for group in data {
        let mut first = group[0].clone();
        for item in group.iter().skip(1) {
            first = first.into_iter().filter(|e| item.contains(e)).collect();
        }
        count += first.len();
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_one() {
        let input = String::from("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb");
        let data = parse(input);
        let result = task_one(&data);
        assert_eq!(result, 11);
    }

    #[test]
    fn example_two() {
        let input = String::from("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb");
        let data = parse(input);
        let result = task_two(&data);
        assert_eq!(result, 6);
    }
}

fn main() {
    let data = parse(fs::read_to_string("input").unwrap().trim().to_string());
    
    println!(" first = {}", task_one(&data));
    println!("second = {}", task_two(&data));
}
