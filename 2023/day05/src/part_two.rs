use rayon::prelude::*;

use crate::almanac::Almanac;
use crate::part_one::seed_location;

pub fn process(almanac: &Almanac) -> u64 {
    let mut min_location = u64::MAX;

    for chunk in almanac.seeds.chunks(2) {
        let start = chunk[0];
        let length = chunk[1];
        let seed_min_location =
            (start..start + length).into_par_iter().map(|seed| seed_location(almanac, seed)).min().unwrap();

        min_location = min_location.min(seed_min_location);
    }

    min_location
}

#[cfg(test)]
mod tests {
    use crate::almanac::Almanac;
    use crate::part_two::process;

    #[test]
    fn example() {
        let almanac = Almanac::from(include_str!("../data/example"));
        assert_eq!(process(&almanac), 46);
    }
}
