use std::collections::HashMap;

use crate::schema::{Item::*, Schema};

type GearMap = HashMap<(usize, usize), Vec<u32>>;

pub fn process(schema: &Schema) -> u32 {
    let push = |map: &mut GearMap, x, y, v| {
        map.entry((x, y)).and_modify(|e| e.push(v)).or_insert(vec![v]);
    };

    let mut adj_gears: GearMap = GearMap::new();
    for (&(x, y), item) in &schema.items {
        if let Value(v) = item {
            let len = v.ilog10() as isize + 1;
            if schema.gear(x as isize - 1, y as isize) {
                push(&mut adj_gears, x - 1, y, *v);
            }
            if schema.gear(x as isize + len, y as isize) {
                push(&mut adj_gears, x + len as usize, y, *v);
            }
            for index in x as isize - 1..x as isize + len + 1 {
                if schema.gear(index, y as isize - 1) {
                    push(&mut adj_gears, index as usize, y - 1, *v);
                }
                if schema.gear(index, y as isize + 1) {
                    push(&mut adj_gears, index as usize, y + 1, *v);
                }
            }
        }
    }

    adj_gears.into_values().fold(0, |acc, x| acc + if x.len() == 2 { x[0] * x[1] } else { 0 })
}

#[cfg(test)]
mod tests {
    use crate::part_two::process;
    use crate::schema::Schema;

    #[test]
    fn example() {
        let schema = Schema::from(include_str!("../data/example"));
        assert_eq!(process(&schema), 467835);
    }
}
