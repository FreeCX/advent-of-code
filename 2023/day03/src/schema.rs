use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Item {
    Value(u32),
    Symbol,
    Gear,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Schema {
    pub items: HashMap<(usize, usize), Item>,
    pub max_x: usize,
    pub max_y: usize,
}

impl Schema {
    fn get(&self, x: isize, y: isize) -> Option<&Item> {
        if x < 0 || x > self.max_x as isize || y < 0 || y > self.max_y as isize {
            None
        } else {
            self.items.get(&(x as usize, y as usize))
        }
    }

    pub fn any(&self, x: isize, y: isize) -> bool {
        let value = self.get(x, y);
        value == Some(&Item::Symbol) || value == Some(&Item::Gear)
    }

    pub fn gear(&self, x: isize, y: isize) -> bool {
        self.get(x, y) == Some(&Item::Gear)
    }
}

impl From<&str> for Schema {
    fn from(input: &str) -> Self {
        let mut items = HashMap::new();

        let max_y = input.len();
        let mut max_x = 0;

        for (y, line) in input.lines().enumerate() {
            let mut value = 0;
            let mut index = 0;
            if max_x == 0 {
                max_x = line.len();
            }
            for (x, symbol) in line.chars().enumerate() {
                if let '0'..='9' = symbol {
                    value = value * 10 + symbol as u32 - 0x30;
                } else {
                    if value > 0 {
                        items.insert((index, y), Item::Value(value));
                    }
                    if symbol != '.' {
                        items.insert((x, y), if symbol == '*' { Item::Gear } else { Item::Symbol });
                    }
                    index = x + 1;
                    value = 0;
                }
            }
            if value > 0 {
                items.insert((index, y), Item::Value(value));
            }
        }

        Schema { items, max_x, max_y }
    }
}

#[cfg(test)]
mod tests {
    use crate::schema::{Item::*, Schema};

    #[test]
    fn example() {
        let mut real = vec![
            ((0, 0), Value(467)),
            ((5, 0), Value(114)),
            ((3, 1), Gear),
            ((2, 2), Value(35)),
            ((6, 2), Value(633)),
            ((6, 3), Symbol),
            ((0, 4), Value(617)),
            ((3, 4), Gear),
            ((5, 5), Symbol),
            ((7, 5), Value(58)),
            ((2, 6), Value(592)),
            ((6, 7), Value(755)),
            ((3, 8), Symbol),
            ((5, 8), Gear),
            ((1, 9), Value(664)),
            ((5, 9), Value(598)),
        ];
        real.sort_by_key(|k| k.0);

        let mut parsed: Vec<_> = Vec::from_iter(Schema::from(include_str!("../data/example")).items);
        parsed.sort_by_key(|k| k.0);

        assert_eq!(parsed, real);
    }
}
