use std::collections::HashMap;

use crate::types::LineType;

pub fn report(input: &[LineType]) -> HashMap<String, u32> {
    let mut path: Vec<String> = Vec::new();
    let mut dirs: HashMap<String, u32> = HashMap::new();

    for line in input {
        match line {
            LineType::ChangeDirectory(name) => path.push(name.to_string()),
            LineType::ExitDirectory => {
                path.pop();
            }
            LineType::File { size, .. } => {
                let mut filepath = String::new();
                // skip root dir /
                for dir in &path {
                    filepath.push_str(&format!("{}/", dir));
                    dirs.entry(filepath.clone()).and_modify(|value| *value += size).or_insert(*size);
                }
            }
            _ => (),
        }
    }

    dirs
}

pub fn process(input: &[LineType]) -> u32 {
    let mut result = 0;
    for (_, size) in report(input) {
        if size < 100000 {
            result += size;
        }
    }

    result
}
