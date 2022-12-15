use crate::id::Id;

pub fn is_valid_one(id: &Id) -> bool {
    id.birth_year.is_some()
        && id.issue_year.is_some()
        && id.expiration_year.is_some()
        && id.height.is_some()
        && id.hair_color.is_some()
        && id.eye_color.is_some()
        && id.passport_id.is_some()
}

pub fn process(ids: &Vec<Id>) -> u32 {
    let mut valid_count = 0;

    for id in ids {
        if is_valid_one(id) {
            valid_count += 1;
        }
    }

    valid_count
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::part_one;

    #[test]
    fn example01() {
        let result = part_one::process(&parse::parse(include_str!("../data/example01")));
        assert_eq!(result, 2);
    }
}
