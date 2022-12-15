use crate::types::*;

pub struct Id {
    pub birth_year: Option<u16>,
    pub issue_year: Option<u16>,
    pub expiration_year: Option<u16>,
    pub height: Option<Height>,
    pub hair_color: Option<String>,
    pub eye_color: Option<String>,
    pub passport_id: Option<String>,
    pub country_id: Option<String>,
}

impl Id {
    pub fn new() -> Id {
        Id {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        }
    }

    pub fn set(&mut self, data: &str) {
        let kv: Vec<_> = data.split(':').collect();
        match kv[0] {
            "byr" => self.birth_year = kv[1].parse().ok(),
            "iyr" => self.issue_year = kv[1].parse().ok(),
            "eyr" => self.expiration_year = kv[1].parse().ok(),
            "hgt" => {
                if kv[1].is_empty() {
                    return;
                }

                let size = kv[1].len();
                let (value, unit) = if kv[1].chars().last().unwrap().is_alphabetic() {
                    let value = kv[1][..size - 2].parse().unwrap();
                    let unit = match &kv[1][size - 2..] {
                        "in" => Unit::Inch,
                        "cm" => Unit::Centimer,
                        unit => panic!("unsupported unit: {}", unit),
                    };
                    (value, unit)
                } else {
                    // if data contains unitless value
                    (kv[1].parse().unwrap(), Unit::None)
                };

                self.height = Some((value, unit));
            }
            "hcl" => self.hair_color = Some(kv[1].to_owned()),
            "ecl" => self.eye_color = Some(kv[1].to_owned()),
            "pid" => self.passport_id = Some(kv[1].to_owned()),
            "cid" => self.country_id = Some(kv[1].to_owned()),
            key => panic!("bad key: {}", key),
        }
    }
}
