pub fn parse(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        result.push(line.to_owned());
    }
    result
}
