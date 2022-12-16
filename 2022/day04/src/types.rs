#[derive(Debug, PartialEq, Eq)]
pub struct Assignment {
    pub left: u16,
    pub right: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Pair {
    pub left: Assignment,
    pub right: Assignment,
}

impl Assignment {
    pub fn new(left: u16, right: u16) -> Self {
        Assignment { left, right }
    }
}

impl Pair {
    pub fn new(left: Assignment, right: Assignment) -> Self {
        Pair { left, right }
    }
}

// FIXME: i am too lazy to implement TryFrom

impl From<&str> for Assignment {
    fn from(value: &str) -> Self {
        let items: Vec<u16> = value.split('-').map(|x| x.parse().unwrap()).collect();
        assert!(items.len() == 2);
        Assignment::new(items[0], items[1])
    }
}

impl From<&str> for Pair {
    fn from(value: &str) -> Self {
        let items: Vec<&str> = value.split(',').collect();
        assert!(items.len() == 2);
        let left = Assignment::from(items[0]);
        let right = Assignment::from(items[1]);
        Pair::new(left, right)
    }
}
