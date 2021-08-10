pub struct Seat {
    pub row: u8,
    pub col: u8,
}

impl Seat {
    fn identify(item: &str) -> Seat {
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

    fn id(&self) -> u16 {
        self.row as u16 * 8 + self.col as u16
    }
}

fn task_one(tickets: &Vec<&str>) -> u16 {
    let mut max_id = 0;

    for ticket in tickets {
        let seat = Seat::identify(ticket);
        max_id = max_id.max(seat.id());
    }

    max_id
}

fn task_two(tickets: &Vec<&str>) -> u16 {
    let mut ids: Vec<_> = tickets.iter().map(|x| Seat::identify(x).id()).collect();
    ids.sort();
    for (a, b) in ids.iter().zip(ids.iter().skip(1)) {
        if b - a > 1 {
            return a + 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::Seat;

    #[test]
    fn ticket_one() {
        let seat = Seat::identify("BFFFBBFRRR");

        assert_eq!(seat.row, 70);
        assert_eq!(seat.col, 7);
        assert_eq!(seat.id(), 567);
    }

    #[test]
    fn ticket_two() {
        let seat = Seat::identify("FFFBBBFRRR");

        assert_eq!(seat.row, 14);
        assert_eq!(seat.col, 7);
        assert_eq!(seat.id(), 119);
    }

    #[test]
    fn ticket_three() {
        let seat = Seat::identify("BBFFBBFRLL");

        assert_eq!(seat.row, 102);
        assert_eq!(seat.col, 4);
        assert_eq!(seat.id(), 820);
    }
}

fn main() {
    let data = include_str!("../input");
    let tickets: Vec<_> = data.lines().collect();

    println!(" first = {}", task_one(&tickets));
    println!("second = {}", task_two(&tickets));
}
