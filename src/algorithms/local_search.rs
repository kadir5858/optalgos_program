use super::traits::{Neighborhood, Solution};

pub fn solve<S, N>(mut current: S, neighborhood: &N) -> S
where 
    S: Solution,
    N: Neighborhood<S>,
{
    loop {
        let current_cost = current.cost();
        let mut found_improvement = false;
        // Search in neighborhood
        for neighbor in neighborhood.neighbors(&current) {
            // First Improvement
            if neighbor.cost() < current_cost {
                current = neighbor;
                found_improvement = true;
                break;
            }
        }

        if !found_improvement {
            break;
        }
    }
    current
}

