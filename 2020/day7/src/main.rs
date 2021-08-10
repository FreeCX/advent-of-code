use std::collections::HashMap;

type BagContent = Vec<(usize, String)>;
type BagOfBugs = HashMap<String, BagContent>;

fn take_name(data: &str) -> String {
    let mut name = data.to_string();
    if name.contains("bags") {
        name = name.replace("bags", "");
    }
    if name.contains("bag") {
        name = name.replace("bag", "");
    }
    if name.contains(".") {
        name = name.replace(".", "");
    }
    name.trim().to_string()
}

fn items(data: Vec<String>) -> BagContent {
    let mut result = BagContent::new();
    for item in data {
        let v: Vec<_> = item.split(" ").collect();
        let name = item.replace(v[0], "").trim().to_string();
        result.push((v[0].parse::<usize>().unwrap(), name));
    }
    result
}

fn parse(data: &str) -> BagOfBugs {
    let mut result = HashMap::new();
    for line in data.lines() {
        let block: Vec<_> = line.split("contain").collect();
        let name = take_name(block[0]);
        let contain: Vec<_> = block[1].split(",").map(|x| take_name(x)).collect();
        if contain[0] == "no other" {
            result.insert(name, vec![]);
        } else {
            result.insert(name, items(contain));
        }
    }
    result
}

fn contain(from: &str, what: &str, bags: &BagOfBugs) -> bool {
    match bags.get(from) {
        Some(items) => {
            if items.len() == 0 {
                return false;
            }
            for (_, item) in items {
                if item == what || contain(item, what, bags) {
                    return true;
                }
            }
            false
        }
        None => false
    }
}

fn inner(name: &str, bags: &BagOfBugs) -> usize {
    match bags.get(name) {
        Some(items) => {
            if items.len() == 0 {
                return 0;
            }
            let mut result = 0;
            for (count, item) in items {
                result += count * (1 + inner(item, bags));
            }
            result
        }
        None => 0
    }
}

fn task_one(bags: &BagOfBugs) -> usize {
    let mut count = 0;
    for bag in bags.keys() {
        if contain(bag, "shiny gold", bags) {
            count += 1;
        }
    }
    count
}

fn task_two(bags: &BagOfBugs) -> usize {
    inner("shiny gold", bags)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example1() {
        let data = include_str!("../examples/example11.txt");
        let bags = parse(data);

        assert_eq!(task_one(&bags), 4);
    }

    #[test]
    fn example21() {
        let data = include_str!("../examples/example21.txt");
        let bags = parse(data);
        assert_eq!(task_two(&bags), 32);
    }

    #[test]
    fn example22() {
        let data = include_str!("../examples/example22.txt");
        let bags = parse(data);
        assert_eq!(task_two(&bags), 126);
    }
}

fn main() {
    let data = include_str!("../input");
    let bags = parse(&data);

    println!(" first = {}", task_one(&bags));
    println!("second = {}", task_two(&bags));
}
