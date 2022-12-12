pub struct Rule {
    pub left_range: usize,
    pub right_range: usize,
    pub symbol: char,
    pub password: String,
}

pub fn parse(line: &str) -> Rule {
    let parts: Vec<_> = line.split(" ").collect();
    let range: Vec<usize> = parts[0].split("-").map(|x| x.parse().unwrap()).collect();
    let symbol = parts[1].chars().nth(0).unwrap();
    Rule {
        left_range: range[0],
        right_range: range[1],
        symbol,
        password: parts[2].to_string(),
    }
}
