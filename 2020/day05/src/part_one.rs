use crate::seat::Seat;

pub fn process(tickets: &Vec<&str>) -> u16 {
    let mut max_id = 0;

    for ticket in tickets {
        let seat = Seat::identify(ticket);
        max_id = max_id.max(seat.id());
    }

    max_id
}
