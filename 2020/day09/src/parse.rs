pub fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}
