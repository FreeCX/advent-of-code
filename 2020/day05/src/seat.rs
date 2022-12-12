pub struct Seat {
    pub row: u8,
    pub col: u8,
}

impl Seat {
    pub fn identify(item: &str) -> Seat {
        let (l, r) = item.split_at(7);

        let mut row = 0;
        for i in l.chars() {
            row <<= 1;
            if i == 'B' {
                row += 1;
            }
        }

        let mut col = 0;
        for i in r.chars() {
            col <<= 1;
            if i == 'R' {
                col += 1;
            }
        }

        Seat { row, col }
    }

    pub fn id(&self) -> u16 {
        self.row as u16 * 8 + self.col as u16
    }
}
