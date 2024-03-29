use crate::id::Id;
use crate::part_one;
use crate::types::*;

fn validate_height(height: &Option<Height>) -> bool {
    if let Some(height) = height {
        return match height.1 {
            Unit::Centimer => return height.0 >= 150 && height.0 <= 193,
            Unit::Inch => return height.0 >= 59 && height.0 <= 76,
            Unit::None => false,
        };
    };
    false
}

fn validate_hair_color(color: &Option<String>) -> bool {
    if let Some(color) = color {
        if color.len() != 7 || !color.starts_with('#') {
            return false;
        }
        for i in color.chars().skip(1) {
            if !((i >= 0x30 as char && i <= 0x39 as char) || ('a'..='f').contains(&i)) {
                return false;
            }
        }
        true
    } else {
        false
    }
}

fn validate_eye_color(eye: &Option<String>) -> bool {
    if let Some(eye) = eye {
        vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().any(|&color| eye == color)
    } else {
        false
    }
}

fn validate_passport_id(id: &Option<String>) -> bool {
    if id.is_none() {
        return false;
    }
    if let Some(id) = id {
        if id.len() != 9 {
            return false;
        }
        for i in id.chars() {
            if !(i >= 0x30 as char && i <= 0x39 as char) {
                return false;
            }
        }
    }
    true
}

fn validate_birth_year(value: &Option<u16>) -> bool {
    value >= &Some(1920) && value <= &Some(2002)
}

fn validate_issue_year(value: &Option<u16>) -> bool {
    value >= &Some(2010) && value <= &Some(2020)
}

fn validate_expiration_year(value: &Option<u16>) -> bool {
    value >= &Some(2020) && value <= &Some(2030)
}

fn is_valid_two(id: &Id) -> bool {
    validate_birth_year(&id.birth_year)
        && validate_issue_year(&id.issue_year)
        && validate_expiration_year(&id.expiration_year)
        && validate_height(&id.height)
        && validate_hair_color(&id.hair_color)
        && validate_eye_color(&id.eye_color)
        && validate_passport_id(&id.passport_id)
}

pub fn process(ids: &Vec<Id>) -> u32 {
    let mut valid_count = 0;

    for id in ids {
        if part_one::is_valid_one(id) && is_valid_two(id) {
            valid_count += 1;
        }
    }

    valid_count
}

#[cfg(test)]
mod tests {
    use crate::parse::parse;
    use crate::part_two::*;
    use crate::types::Unit;

    #[test]
    fn example02_invalid() {
        let result = process(&parse(include_str!("../data/example02_invalid")));
        assert_eq!(result, 0);
    }

    #[test]
    fn example02_valid() {
        let result = process(&parse(include_str!("../data/example02_valid")));
        assert_eq!(result, 4);
    }

    #[test]
    fn birth_year_validation() {
        assert!(validate_birth_year(&Some(2002)));
        assert!(!validate_birth_year(&Some(2003)));
        assert!(!validate_birth_year(&None));
    }

    #[test]
    fn height_validation() {
        assert!(validate_height(&Some((60, Unit::Inch))));
        assert!(validate_height(&Some((190, Unit::Centimer))));
        assert!(!validate_height(&Some((100, Unit::None))));
        assert!(!validate_height(&None));
    }

    #[test]
    fn hair_color_validation() {
        assert!(validate_hair_color(&Some("#123abc".to_string())));
        assert!(!validate_hair_color(&Some("#123abz".to_string())));
        assert!(!validate_hair_color(&Some("123abc".to_string())));
        assert!(!validate_hair_color(&None));
    }

    #[test]
    fn eye_color_validation() {
        assert!(validate_eye_color(&Some("brn".to_string())));
        assert!(!validate_eye_color(&Some("wat".to_string())));
        assert!(!validate_eye_color(&None));
    }

    #[test]
    fn passport_id_validation() {
        assert!(validate_passport_id(&Some("000000001".to_string())));
        assert!(!validate_passport_id(&Some("0123456789".to_string())));
        assert!(!validate_passport_id(&None));
    }
}
