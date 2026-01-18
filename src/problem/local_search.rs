use crate::algorithms::traits::Neighborhood;
use super::solution::{RectangleSolution, BoxBin, Placement};
use super::rect::Rect;
use std::collections::HashSet;

pub struct GeometricNeighborhood;

impl Neighborhood<RectangleSolution> for GeometricNeighborhood {
    fn neighbors<'a>(&'a self, solution: &'a RectangleSolution) -> Box<dyn Iterator<Item = RectangleSolution> + 'a> {
        // Iterate over all boxes and all rectangles in it
        let moves = solution.boxes.iter().enumerate().flat_map(move |(src_idx, src_box)| {
            src_box.placements.iter().enumerate().flat_map(move |(p_idx, placement) | {
                let rect = placement.rect;
                // Try to move rectangle into every other box
                solution.boxes.iter().enumerate().filter_map(move |(tgt_idx, tgt_box)| {
                    if src_idx == tgt_idx {
                        return None;
                    }
                    // Check if rectangle fit in target box
                    if let Some((x, y, rotated)) = find_position_in_box(tgt_box, rect) {
                        // Create new neighbor
                        let mut new_solution = solution.clone();
                        // Remove rectangle from source box 
                        new_solution.boxes[src_idx].placements.swap_remove(p_idx);
                        // Insert into target box
                        new_solution.boxes[tgt_idx].placements.push(Placement { rect, x, y, rotated });
                        // Remove source box if empty
                        if new_solution.boxes[src_idx].placements.is_empty() {
                            new_solution.boxes.swap_remove(src_idx);
                        }
                        return Some(new_solution);
                    }
                    None
                }) 
            })
        });

        Box::new(moves)
    }
}

fn find_position_in_box(bin: &BoxBin, rect: Rect) -> Option<(u32, u32, bool)> {
    // Collect candidates (origin + edges of existing rectangles)
    let mut candidates = HashSet::new();
    candidates.insert((0, 0));

    for p in &bin.placements {
        let c1 = (p.x + p.width(), p.y);    // Right bottom of p
        let c2 = (p.x, p.y + p.height());   // Left top of p
        
        if c1.0 < bin.capacity && c1.1 < bin.capacity { candidates.insert(c1); }
        if c2.0 < bin.capacity && c1.1 < bin.capacity { candidates.insert(c2); }
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
        if can_place(bin, rect, x, y, false) { return Some((x, y, false)); } 
        if can_place(bin, rect, x, y, true) { return Some((x, y, true)); }
    }
    None    
}

// Checks if rectangles on position x, y can placed correctly
fn can_place(bin: &BoxBin, rect: Rect, x: u32, y: u32, rotated: bool) -> bool {
    let w = if rotated { rect.height } else { rect.width };
    let h = if rotated { rect.width } else { rect.height };
    // Boundary check
    if x + w > bin.capacity || y + h > bin.capacity {
        return false;
    }
    // Intersection check
    let candidate = Placement { rect, x, y, rotated };
    for existing in &bin.placements {
        if candidate.intersects(existing) {
            return false;
        }
    }
    true
}
