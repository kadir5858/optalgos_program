use crate::algorithms::traits::{Neighborhood};
use super::solution::{RectangleSolution, PermutationSolution, Placement};
use rand::seq::SliceRandom;
use rand::{Rng, rng};

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
// Geometric Neighborhood with Overlaping
// ---------------------------------------------------------
