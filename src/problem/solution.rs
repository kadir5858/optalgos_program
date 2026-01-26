use super::rect::Rect;
use super::instance::Instance;
use crate::algorithms::traits::Solution;
use std::collections::HashSet;


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

    pub fn intersection_area(&self, other: &Placement) -> u32 {
        // Determine coordinates of intersection rectangle
        let x_overlap_start = self.x.max(other.x);
        let x_overlap_end = (self.x + self.width()).min(other.x + other.width());
        let y_overlap_start = self.y.max(other.y);
        let y_overlap_end = (self.y + self.height()).min(other.y + other.height());
        // Check if overlap exists
        if x_overlap_start < x_overlap_end && y_overlap_start < y_overlap_end {
            let width = x_overlap_end - x_overlap_start;
            let height = y_overlap_end - y_overlap_start;
            return width * height;
        }
        0
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

    pub fn find_position_in_box(&self, rect: Rect) -> Option<(u32, u32, bool)> {
        // Collect candidates (origin + edges of existing rectangles)
        let mut candidates = HashSet::new();
        candidates.insert((0, 0));

        for p in &self.placements {
            let c1 = (p.x + p.width(), p.y);    // Right bottom of p
            let c2 = (p.x, p.y + p.height());   // Left top of p
            
            if c1.0 < self.capacity && c1.1 < self.capacity { candidates.insert(c1); }
            if c2.0 < self.capacity && c2.1 < self.capacity { candidates.insert(c2); }
        }
        // Sort candidates by bottom-left heuristic
        let mut sorted_candidates: Vec<(u32, u32)> = candidates.into_iter().collect();
        sorted_candidates.sort_by(|a, b| {
            if a.1 != b.1 { 
                a.1.cmp(&b.1) 
            } else {
                a.0.cmp(&b.0)
            }
        });
        // Check candidates
        for (x, y) in sorted_candidates {
            if self.can_place(rect, x, y, false) { return Some((x, y, false)); } 
            if self.can_place(rect, x, y, true) { return Some((x, y, true)); }
        }
        None    
    }

    // Checks if rectangles on position x, y can placed correctly
    fn can_place(&self, rect: Rect, x: u32, y: u32, rotated: bool) -> bool {
        let w = if rotated { rect.height } else { rect.width };
        let h = if rotated { rect.width } else { rect.height };
        // Boundary check
        if x + w > self.capacity || y + h > self.capacity {
            return false;
        }
        // Intersection check
        let candidate = Placement { rect, x, y, rotated };
        for existing in &self.placements {
            if candidate.intersects(&existing) {
                return false;
            }
        }
        true
    }
}

/// Solution for both Greedy selection strategies, geometric Local Search 
/// and Local Search with overlaping
#[derive(Clone, Debug)]
pub struct RectangleSolution {
    pub instance: Instance,
    pub boxes: Vec<BoxBin>,
    // Penalty for overlaping mode
    pub penalty_factor: Option<i64>,
}

impl RectangleSolution {
    // Standard constructor
    pub fn new(instance: Instance) -> Self {
        Self { instance, boxes: Vec::new(), penalty_factor: None }
    }
    // Constructor for overlaping mode
    pub fn with_penalty(mut self,factor: i64) -> Self {
        self.penalty_factor = Some(factor);
        self
    }
}

impl Solution for RectangleSolution {
    // Number of boxes and score as cost
    type Cost = (usize, i64);

    fn cost(&self) -> Self::Cost {
        let num_boxes = self.boxes.len();
        // Score: sum of squares of used area in each box
        let mut score: i64 = 0;
        // Case distinction for overlaping and standard cost calculation
        if let Some(penalty_factor) = self.penalty_factor {
            let mut total_penalty: i64 = 0;
            let mut density_score: i64 = 0;

            for bin in &self.boxes {
                // Calculate overlaping
                let mut bin_penalty = 0;
                for i in 0..bin.placements.len() {
                    for j in (i+1)..bin.placements.len() {
                        let intersect = bin.placements[i].intersection_area(&bin.placements[j]);
                        if intersect > 0 {
                            bin_penalty += intersect as i64;
                        }
                    }
                }
                total_penalty += bin_penalty;
                // Negative density score
                let used_area: u32 = bin.placements.iter().map(|p| p.rect.area()).sum();
                density_score -= (used_area as i64).pow(2);
            }
            // Dynamic Weighting: Weight must be > max possible density score (L^4)
            // to ensure box reduction is prioritized over density.
            let box_weight = (self.instance.box_size as i64).pow(4) + 1;
            score = total_penalty * penalty_factor + density_score + (num_boxes as i64) * box_weight;

            (0, score)
        } else {
            for b in &self.boxes {
                let used_area: u32 = b.placements.iter().map(|p| p.rect.area()).sum();
                score -= (used_area as i64).pow(2);
            }
            (num_boxes, score)
        }

    }
}

/// Solution for rule based Local Search
#[derive(Clone, Debug)]
pub struct PermutationSolution {
    pub instance: Instance,
    pub sequence: Vec<Rect>,
}

impl PermutationSolution {
    pub fn new(instance: Instance, sequence: Vec<Rect>) -> Self {
        Self { instance, sequence }
    }
}

impl Solution for PermutationSolution {
    type Cost = (usize, i64);

    fn cost(&self) -> Self::Cost {
        let mut boxes: Vec<BoxBin> = Vec::new();
        let box_size = self.instance.box_size;

        for &rect in &self.sequence {
            let mut placed = false;
            
            for bin in boxes.iter_mut() {
                if let Some((x, y, rotated)) = bin.find_position_in_box(rect) {
                    bin.placements.push(Placement { rect, x, y, rotated });
                    placed = true;
                    break;
                }
            }
            if !placed {
                let mut new_bin = BoxBin::new(box_size);
                new_bin.placements.push(Placement { rect, x: 0, y: 0, rotated: false });
                boxes.push(new_bin);
            }
        }
        let num_boxes = boxes.len();
        let mut score: i64 = 0;
        for b in &boxes {
            let used_area: u32 = b.placements.iter().map(|p| p.rect.area()).sum();
            score += (used_area as i64).pow(2);
        }
        (num_boxes, -score)
    }
}

