use super::traits::{Neighborhood, Solution};

pub fn solve<S, N>(mut current: S, neighborhood: &N) -> S
where 
    S: Solution,
    N: Neighborhood<S>,
{
    loop {
        let current_cost = current.cost();
        let mut improved_solution = None;
        // Search in neighborhood
        for neighbor in neighborhood.neighbors(&current) {
            // First Improvement
            if neighbor.cost() < current_cost {
                improved_solution = Some(neighbor);
                break;
            }
        }
        // Apply improvement if found
        if let Some(improvement) = improved_solution {
            current = improvement;
        } else {
            break;
        }
    }
    current
}

