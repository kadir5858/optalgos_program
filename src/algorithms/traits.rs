
pub trait Solution: Clone {
    // Values of cost must be comparable and copyable
    type Cost: Ord + Copy;

    fn cost(&self) -> Self::Cost;
}

pub trait Neighborhood<S> {
    // Returns a iterator over neighbor solutions, lifetime 'a binds it to input data
    fn neighbors<'a>(&'a self, solution: &'a S) -> Box<dyn Iterator<Item = S> + 'a>;
}

pub trait GreedyState {
    type Item;

    fn is_finished(&self) -> bool;
    // Add an item to current solution
    fn apply(&mut self, item: Self::Item);
}

pub trait SelectionStrategy<P: GreedyState> {
    fn next_candidate(&mut self, problem: &P) -> Option<P::Item>;
}

