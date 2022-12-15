use crate::types::Questions;

pub fn parse(data: &str) -> Vec<Vec<Questions>> {
    let mut result = Vec::new();
    for group in data.trim().split("\n\n") {
        let mut question_group = Vec::new();
        for passenger in group.split('\n') {
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
