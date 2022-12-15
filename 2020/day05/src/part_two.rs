use crate::seat::Seat;

pub fn process(tickets: &[&str]) -> u16 {
    let mut ids: Vec<_> = tickets.iter().map(|x| Seat::identify(x).id()).collect();
    ids.sort();
    for (a, b) in ids.iter().zip(ids.iter().skip(1)) {
        if b - a > 1 {
            return a + 1;
        }
    }
    0
}
