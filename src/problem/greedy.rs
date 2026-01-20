use crate::algorithms::traits::{GreedyState, SelectionStrategy};
use super::rect::Rect;
use super::solution::{RectangleSolution, BoxBin};
use super::instance::Instance;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct RectangleGreedyState {
    pub solution: RectangleSolution,
    pub remaining_rects: Vec<Rect>,
}

impl RectangleGreedyState {
    pub fn new(instance: Instance) -> Self {
        let rects = instance.rects.clone();
        Self { solution: RectangleSolution::new(instance), remaining_rects: rects }
    }
}

impl GreedyState for RectangleGreedyState {
    type Item = Rect;

    fn is_finished(&self) -> bool {
        self.remaining_rects.is_empty()
    }

    fn apply(&mut self, rect: Self::Item) {
        // Remove rect from remaining rects
        if let Some(pos) = self.remaining_rects.iter().position(|r| r.id == rect.id) {
            self.remaining_rects.remove(pos);
        }
        // Try to place in existing boxes
        for bin in self.solution.boxes.iter_mut() {
            if try_place_cp(bin, rect) {
                return;
            }
        }
        // If no box found, open new box
        let mut new_bin = BoxBin::new(self.solution.instance.box_size);
        // Place it left-bottom
        let placed = new_bin.try_place(rect, 0, 0, false);
        if !placed { panic!("Could'nt place rectangle in new box.")}

        self.solution.boxes.push(new_bin);
    }
}

/// Place a rectangle with candidate points
/// 
/// # Arguments
/// * `bin` - BoxBin
/// * `rect` - Rectangle
/// 
/// # Returns
/// true if placement succesfull
fn try_place_cp(bin: &mut BoxBin, rect: Rect) -> bool {
    // Create candidate list
    let mut candidates = HashSet::new();
    candidates.insert((0, 0));

    for placement in &bin.placements {
        // Right top corner of rect
        let c1 = (placement.x + placement.width(), placement.y);
        // Left top corner of rect
        let c2 = (placement.x, placement.y + placement.height());

        if c1.0 < bin.capacity && c1.1 < bin.capacity { candidates.insert(c1); }
        if c2.0 < bin.capacity && c2.1 < bin.capacity {candidates.insert(c2); }
    }

    // Sort candidates over y then x
    let mut sorted_candidates: Vec<(u32, u32)> = candidates.into_iter().collect();
    sorted_candidates.sort_by(|a, b| {
        if a.1 != b.1 { a.1.cmp(&b.1) } else { a.0.cmp(&b.0) } 
    });
    // Try to place at candidate place
    for (x, y) in sorted_candidates {
        if bin.try_place(rect, x, y, false) {
            return true;
        }
        // Try again rotated
        if bin.try_place(rect, x, y, true) {
            return true;
        }
    }
    false
}

// Selection strategies for greedy

/// Sort by area strategy
pub struct SortByAreaStrategy;

impl SelectionStrategy<RectangleGreedyState> for SortByAreaStrategy {
    fn next_candidate(&mut self, problem: &RectangleGreedyState) -> Option<Rect> {
        // Select rectangle with largest area
        problem.remaining_rects.iter().max_by_key(|r| r.area()).cloned()
    }
}

/// Sort by longest side
pub struct SortByMaxSideStrategy;

impl SelectionStrategy<RectangleGreedyState> for SortByMaxSideStrategy {
    fn next_candidate(&mut self, problem: &RectangleGreedyState) -> Option<Rect> {
        // Select rectangle with longest side, either y or x
        problem.remaining_rects.iter().max_by_key(|r| r.width.max(r.height)).cloned()
    }
}
