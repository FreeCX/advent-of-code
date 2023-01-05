#[derive(Default)]
pub struct Data {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Data {
    pub fn get(&self, x: usize, y: usize) -> u8 {
        assert!(x < self.width);
        assert!(y < self.height);

        *self.data.get(y * self.width + x).unwrap()
    }
}
