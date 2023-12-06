use crate::almanac::Almanac;

pub fn seed_location(almanac: &Almanac, seed: u64) -> u64 {
    let mut curr = seed;
    let mut next = seed;

    for map in &almanac.maps {
        for range in &map.ranges {
            if curr >= range.source_start && curr < range.source_start + range.length {
                next = (range.dest_start as i64 - range.source_start as i64 + curr as i64) as u64;
                break;
            }
        }
        curr = next;
    }

    curr
}

pub fn process(almanac: &Almanac) -> u64 {
    let mut min_location = u64::MAX;

    for seed in &almanac.seeds {
        min_location = min_location.min(seed_location(almanac, *seed));
    }

    min_location
}

#[cfg(test)]
mod tests {
    use crate::almanac::Almanac;
    use crate::part_one::process;

    #[test]
    fn example() {
        let almanac = Almanac::from(include_str!("../data/example"));
        assert_eq!(process(&almanac), 35);
    }
}
