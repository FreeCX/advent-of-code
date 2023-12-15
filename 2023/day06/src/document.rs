#[derive(Debug, PartialEq, Eq)]
pub struct Document {
    pub time: Vec<u64>,
    pub dist: Vec<u64>,
    pub time_one: u64,
    pub dist_one: u64,
}

impl From<&str> for Document {
    fn from(input: &str) -> Self {
        let (time, dist) = input.split_once('\n').unwrap();

        let (_, time) = time.split_once(':').unwrap();
        let time_one = time.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap();
        let time = time.split_whitespace().map(|x| x.parse().unwrap()).collect();

        let (_, dist) = dist.split_once(':').unwrap();
        let dist_one = dist.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap();
        let dist = dist.split_whitespace().map(|x| x.parse().unwrap()).collect();

        Document { time, dist, time_one, dist_one }
    }
}

#[cfg(test)]
mod tests {
    use crate::document::Document;

    #[test]
    fn example() {
        let parsed = Document::from(include_str!("../data/example"));
        let real = Document { time: vec![7, 15, 30], dist: vec![9, 40, 200], time_one: 71530, dist_one: 940200 };
        assert_eq!(parsed, real);
    }
}
