use super::rect::Rect;

#[derive(Clone, Debug)]
pub struct Instance {
    pub box_size: u32,
    pub rects: Vec<Rect>,
}

impl Instance {
    pub fn new(box_size: u32, rects: Vec<Rect>) -> Self {
        Self { box_size, rects }
    }
}
