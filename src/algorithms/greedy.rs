use super::traits::{GreedyState, SelectionStrategy};

pub fn solve<P, S>(problem: &mut P, strategy: &mut S) 
where 
    P: GreedyState,
    S: SelectionStrategy<P>,
{
    // As long as rectangle is left
    while !problem.is_finished() {
        if let Some(candidate) = strategy.next_candidate(problem) {
            problem.apply(candidate);
        } else {
            break;
        }
    }
}