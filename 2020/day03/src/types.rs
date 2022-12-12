pub type Pos = (usize, usize);

#[derive(Eq, PartialEq)]
pub enum Field {
    Empty,
    Tree,
}

pub struct Land {
    pub data: Vec<Vec<Field>>,
    pub width: usize,
    pub height: usize,
}
