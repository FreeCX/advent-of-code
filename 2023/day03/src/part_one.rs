use crate::schema::{Item::*, Schema};

pub fn process(schema: &Schema) -> u32 {
    let mut result = 0;
    for (&(x, y), item) in &schema.items {
        if let Value(v) = item {
            let len = v.ilog10() as isize + 1;
            let on_left_right = schema.any(x as isize - 1, y as isize) || schema.any(x as isize + len, y as isize);
            let on_top_bottom = (x as isize - 1..x as isize + len + 1)
                .any(|index| schema.any(index, y as isize - 1) || schema.any(index, y as isize + 1));
            if on_left_right || on_top_bottom {
                result += v;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::part_one::process;
    use crate::schema::Schema;

    #[test]
    fn example() {
        let schema = Schema::from(include_str!("../data/example"));
        assert_eq!(process(&schema), 4361);
    }
}
