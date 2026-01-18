#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct Rect {
    pub id: usize,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(id: usize, width: u32, height: u32) -> Self {
        Self { id, width, height }        
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}
