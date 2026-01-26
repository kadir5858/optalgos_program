use crate::algorithms::traits::{Neighborhood};
use super::solution::{RectangleSolution, PermutationSolution, Placement, BoxBin};
use super::rect::Rect;
use rand::{Rng, rng};
use core::panic;
use std::collections::HashSet;
use std::iter::once_with;

// ---------------------------------------------------------
// Geometric Neighborhood
// ---------------------------------------------------------
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
                    if let Some((x, y, rotated)) = tgt_box.find_position_in_box(rect) {
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

// ---------------------------------------------------------
// Rule based Neighborhood
// ---------------------------------------------------------
pub struct RuleBasedNeighborhood {
    // Optional tuning parameter to reduce neighbors
    pub max_swaps: Option<usize>,
}

impl RuleBasedNeighborhood {
    pub fn new(max_swaps: Option<usize>) -> Self {
        Self { max_swaps }
    }
}

impl Neighborhood<PermutationSolution> for RuleBasedNeighborhood {
    fn neighbors<'a>(&'a self, solution: &'a PermutationSolution) -> Box<dyn Iterator<Item = PermutationSolution> + 'a> {
        let n = solution.sequence.len();
        if n < 2 {
            return Box::new(std::iter::empty());
        }
        // Random selection of neighbors if k is set
        if let Some(k) = self.max_swaps {
            let mut rng = rng();
            let mut neighbors = Vec::with_capacity(k);

            for _ in 0..k {
                // Select two random idxs
                let i = rng.random_range(0..n);
                let j = rng.random_range(0..n);

                if i != j {
                    let mut new_sol = solution.clone();
                    new_sol.sequence.swap(i, j);
                    neighbors.push(new_sol);
                }
            }
            return Box::new(neighbors.into_iter());
        } else {
            // Without k
            let moves = (0..n).flat_map(move |i| {
                (i + 1..n).map(move |j| {
                    let mut new_sol = solution.clone();
                    new_sol.sequence.swap(i, j);
                    new_sol
                })
            });
            return Box::new(moves);
        }
    }
}

// ---------------------------------------------------------
// Geometric Neighborhood with Overlapping
// ---------------------------------------------------------
pub struct OverlappingNeighborhood {
    pub max_overlap_percent: f64, // 0.0 bis 1.0 (1.0 = 100%)
}

impl Neighborhood<RectangleSolution> for OverlappingNeighborhood {
    fn neighbors<'a>(&'a self, solution: &'a RectangleSolution) -> Box<dyn Iterator<Item = RectangleSolution> + 'a> {
        if solution.penalty_factor.is_none() {
            panic!("Penalty factor for Overlapping Neighborhood not set.")
        }
        let moves = solution.boxes.iter().enumerate().flat_map(move |(src_idx, src_box)| {
            src_box.placements.iter().enumerate().flat_map(move |(p_idx, placement)| {
                let rect = placement.rect;

                // Move rectangle in existing box
                let existing_box_moves = solution.boxes.iter().enumerate().filter_map(move |(tgt_idx, tgt_box)| {
                    if src_idx == tgt_idx { return None; }
                    // Search position with allowed overlap
                    if let Some((x, y, rotated)) = find_position_with_overlap(tgt_box, rect, self.max_overlap_percent) {
                        let mut new_sol = solution.clone();
                        // Move rectangle
                        new_sol.boxes[src_idx].placements.swap_remove(p_idx);
                        new_sol.boxes[tgt_idx].placements.push(Placement { rect, x, y, rotated });

                        if new_sol.boxes[src_idx].placements.is_empty() {
                            new_sol.boxes.swap_remove(src_idx);
                        }

                        return Some(new_sol);
                    }
                    None
                });
                // Create new box and place rectangle left-bottom
                // Once with creates iterator with only one element
                let new_box_move = once_with(move || {
                    let mut new_sol = solution.clone();
                    // Remove from source box
                    new_sol.boxes[src_idx].placements.swap_remove(p_idx);
                    if new_sol.boxes[src_idx].placements.is_empty() {
                         new_sol.boxes.swap_remove(src_idx);
                    }
                    // Create new box and place rectangle
                    let mut new_bin = BoxBin::new(solution.instance.box_size);
                    new_bin.try_place(rect, 0, 0, false);
                    new_sol.boxes.push(new_bin);
                    
                    Some(new_sol)
                }).flatten();

                // Combine both iterators, Local Search will first use all exisitng box neighbors
                // before trying new box
                existing_box_moves.chain(new_box_move)
            })
        });

        Box::new(moves)
    }
}

fn find_position_with_overlap(bin: &BoxBin, rect: Rect, max_overlap_percent: f64) -> Option<(u32, u32, bool)> {
    // Create candidates
    let mut candidates = HashSet::new();
    candidates.insert((0, 0));
    for p in &bin.placements {
        let c1 = (p.x + p.width(), p.y);        // Right top corner of rect
        let c2 = (p.x, p.y + p.height());       // Left top corner of rect
        if c1.0 < bin.capacity && c1.1 < bin.capacity { candidates.insert(c1); }
        if c2.0 < bin.capacity && c2.1 < bin.capacity { candidates.insert(c2); }
    }
    
    let mut sorted_candidates: Vec<(u32, u32)> = candidates.into_iter().collect();
    sorted_candidates.sort_by(|a, b| if a.1 != b.1 { a.1.cmp(&b.1) } else { a.0.cmp(&b.0) });

    for (x, y) in sorted_candidates {
        if check_overlap_limit(bin, rect, x, y, false, max_overlap_percent) {
            return Some((x, y, false));
        }
        if check_overlap_limit(bin, rect, x, y, true, max_overlap_percent) {
            return Some((x, y, true));
        }
    }
    None
}

fn check_overlap_limit(bin: &BoxBin, rect: Rect, x: u32, y: u32, rotated: bool, limit: f64) -> bool {
    let w = if rotated { rect.height } else { rect.width };
    let h = if rotated { rect.width } else { rect.height };
    
    if x + w > bin.capacity || y + h > bin.capacity {
        return false;
    }

    let candidate = Placement { rect, x, y, rotated };
    
    for existing in &bin.placements {
        let intersection = candidate.intersection_area(existing);
        if intersection > 0 {
            let max_area = candidate.rect.area().max(existing.rect.area());
            let percent = (intersection as f64) / (max_area as f64);
            
            if percent > limit {
                return false;
            }
        }
    }
    true
}

