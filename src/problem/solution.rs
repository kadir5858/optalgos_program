use super::rect::Rect;
use super::instance::Instance;
use crate::algorithms::traits::Solution;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Placement {
    pub rect: Rect,
    // x and y denotes the left-bottom corner of the Placement (Rect)
    pub x: u32,
    pub y: u32,
    pub rotated: bool,
}

impl Placement {
    pub fn width(&self) -> u32 {
        if self.rotated { self.rect.height } else { self.rect.width}
    }

    pub fn height(&self) -> u32 {
        if self.rotated { self.rect.width } else { self.rect.height }
    }

    pub fn intersects(&self, other: &Placement) -> bool {
        // Borders of current Rect Placement
        let r1_x2 = self.x + self.width();      // Right border
        let r1_y2 = self.y + self.height();     // Top border
        // Borders of other Rect Placement
        let r2_x2 = other.x + other.width();    // Right border
        let r2_y2 = other.y + other.height();   // Top border
        // Check if one Placement is left, right, top or down of other Placement -> no intersect
        let intersects = !(r1_x2 <= other.x || r2_x2 <= self.x || r1_y2 <= other.y || r2_y2 <= self.y);
        intersects
    }
}

#[derive(Clone, Debug)]
pub struct BoxBin {
    pub capacity: u32,  // Denotes box length L
    pub placements: Vec<Placement>
}

impl BoxBin {
    pub fn new(capacity: u32) -> Self {
        Self { capacity, placements: Vec::new() }
    }

    pub fn try_place(&mut self, rect: Rect, x: u32, y: u32, rotated: bool) -> bool {
        let new_placement = Placement { rect, x, y, rotated };
        // Check box bounds
        if x + new_placement.width() > self.capacity || y + new_placement.height() > self.capacity {
            return false;
        }
        // Check collision with other placements
        for existing in &self.placements {
            if new_placement.intersects(existing) {
                return false;
            }
        }

        self.placements.push(new_placement);
        true
    }

}

#[derive(Clone, Debug)]
pub struct RectangleSolution {
    pub instance: Instance,
    pub boxes: Vec<BoxBin>
}

impl RectangleSolution {
    pub fn new(instance: Instance) -> Self {
        Self { instance, boxes: Vec::new() }
    }
}

impl Solution for RectangleSolution {
    // Number of boxes and score as cost
    type Cost = (usize, i64);

    fn cost(&self) -> Self::Cost {
        let num_boxes = self.boxes.len();

        // Score: sum of squares of used area in each box
        let mut score: i64 = 0;
        for b in &self.boxes {
            let used_area: u32 = b.placements.iter().map(|p| p.rect.area()).sum();
            score += (used_area as i64).pow(2);
        }
        (num_boxes, -score)
    }
}
