struct Top3 {
    a: u32,
    b: u32,
    c: u32,
}

impl Top3 {
    fn new() -> Top3 {
        Top3 { a: 0, b: 0, c: 0 }
    }

    fn push(&mut self, value: u32) {
        if value > self.a {
            self.c = self.b;
            self.b = self.a;
            self.a = value;
        }
        else if value > self.b {
            self.c = self.b;
            self.b = value;
        }
        else if value > self.c {
            self.c = value;
        }
    }

    fn sum(&self) -> u32 {
        self.a + self.b + self.c
    }
}

pub fn process(input: &str) -> u32 {
    let mut items = Top3::new();
    let mut cur_value = 0;

    for item in input.split('\n') {
        if item.is_empty() {
            items.push(cur_value);
            cur_value = 0;
            continue;
        }

        cur_value += item.parse::<u32>().unwrap();
    }

    items.sum()
}

#[cfg(test)]
mod tests {
    use crate::part_two::process;

    #[test]
    fn default() {
        assert_eq!(process(include_str!("../data/demo")), 45_000);
    }
}
