#[derive(Debug, PartialEq, Eq)]
pub struct Almanac {
    pub seeds: Vec<u64>,
    pub maps: Vec<Map>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Map {
    pub from: String,
    pub to: String,
    pub ranges: Vec<Range>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Range {
    pub dest_start: u64,
    pub source_start: u64,
    pub length: u64,
}

impl From<&str> for Almanac {
    fn from(input: &str) -> Self {
        let (seeds, other) = input.split_once("\n\n").unwrap();

        let (_, values) = seeds.split_once(':').unwrap();
        let seeds = values.split_whitespace().map(|x| x.parse().unwrap()).collect();

        let maps = other.split("\n\n").map(Map::from).collect();

        Almanac { seeds, maps }
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let (name, ranges) = input.split_once(':').unwrap();

        let (name, _) = name.split_once(' ').unwrap();
        let (from, to) = name.split_once("-to-").unwrap();

        let ranges = ranges.trim().split('\n').map(Range::from).collect();

        Map { from: from.to_string(), to: to.to_string(), ranges }
    }
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        match &value.split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<_>>()[..] {
            &[a, b, c] => Range { dest_start: a, source_start: b, length: c },
            other => panic!("format `{other:?}` not supported"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::almanac::{Almanac, Map, Range};

    #[test]
    fn parse_almanac() {
        let parsed = Almanac::from("seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n");
        let real = Almanac {
            seeds: vec![79, 14, 55, 13],
            maps: vec![Map {
                from: "seed".to_string(),
                to: "soil".to_string(),
                ranges: vec![
                    Range { dest_start: 50, source_start: 98, length: 2 },
                    Range { dest_start: 52, source_start: 50, length: 48 },
                ],
            }],
        };
        assert_eq!(parsed, real);
    }

    #[test]
    fn parse_map() {
        let parsed = Map::from("seed-to-soil map:\n50 98 2\n52 50 48");
        let real = Map {
            from: "seed".to_string(),
            to: "soil".to_string(),
            ranges: vec![
                Range { dest_start: 50, source_start: 98, length: 2 },
                Range { dest_start: 52, source_start: 50, length: 48 },
            ],
        };
        assert_eq!(parsed, real);
    }

    #[test]
    fn parse_range() {
        let parsed = Range::from("50 98 2");
        let real = Range { dest_start: 50, source_start: 98, length: 2 };
        assert_eq!(parsed, real);
    }
}
