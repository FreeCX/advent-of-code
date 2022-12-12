use crate::types::*;

fn take_name(data: &str) -> String {
    let mut name = data.to_string();
    if name.contains("bags") {
        name = name.replace("bags", "");
    }
    if name.contains("bag") {
        name = name.replace("bag", "");
    }
    if name.contains(".") {
        name = name.replace('.', "");
    }
    name.trim().to_string()
}

fn items(data: Vec<String>) -> BagContent {
    let mut result = BagContent::new();
    for item in data {
        let v: Vec<_> = item.split(' ').collect();
        let name = item.replace(v[0], "").trim().to_string();
        result.push((v[0].parse::<usize>().unwrap(), name));
    }
    result
}

pub fn parse(data: &str) -> BagOfBugs {
    let mut result = BagOfBugs::new();
    for line in data.lines() {
        let block: Vec<_> = line.split("contain").collect();
        let name = take_name(block[0]);
        let contain: Vec<_> = block[1].split(',').map(take_name).collect();
        if contain[0] == "no other" {
            result.insert(name, vec![]);
        } else {
            result.insert(name, items(contain));
        }
    }
    result
}
