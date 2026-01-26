use std::time::{Duration, Instant};
use rand::seq::SliceRandom;
use rand::rng;

use crate::algorithms;
use crate::algorithms::traits::Solution;
use crate::generator::Generator;
use crate::problem::instance::Instance;
use crate::problem::greedy::{RectangleGreedyState, SortByAreaStrategy, SortByMaxSideStrategy};
use crate::problem::local_search::{GeometricNeighborhood, RuleBasedNeighborhood, OverlappingNeighborhood};
use crate::problem::solution::{RectangleSolution, PermutationSolution, BoxBin};

pub struct TestConfig {
    pub num_instances: usize,
    pub num_rects: usize,
    pub width_range: (u32, u32),
    pub height_range: (u32, u32),
    pub box_size: u32,
}

/// Run testsuite with given configuration
pub fn run_suite(configs: &[TestConfig]) {
    println!("Start Test Suite");
    
    for config in configs {
        println!("\nConfiguration: {} Rectangles, Box-Size L={}, Rectangle Ranges (width)-(height) {:?}-{:?}", 
            config.num_rects, config.box_size, config.width_range, config.height_range);
        println!("Number Instances: {}", config.num_instances);
        
        println!("\n{:<25} | {:<12} | {:<15}", "Algorithm", "Ø Boxes", "Ø Time (ms)");
        println!("{:-<58}", "-");

        let mut results_greedy_area = Vec::new();
        let mut results_greedy_side = Vec::new();
        let mut results_ls_geo = Vec::new();
        let mut results_ls_rule = Vec::new();
        let mut results_ls_overlap = Vec::new();

        for _ in 0..config.num_instances {
            // Generate Instances
            let instance = Generator::generate_instance(config.num_rects, config.width_range, config.height_range, config.box_size);
            
            // Greedy (Area)
            let start = Instant::now();
            let mut state = RectangleGreedyState::new(instance.clone());
            let mut strat = SortByAreaStrategy;
            algorithms::greedy::solve(&mut state, &mut strat);
            let dur = start.elapsed();
            results_greedy_area.push((state.solution.boxes.len(), dur));

            // Greedy (Max Side)
            let start = Instant::now();
            let mut state = RectangleGreedyState::new(instance.clone());
            let mut strat = SortByMaxSideStrategy;
            algorithms::greedy::solve(&mut state, &mut strat);
            let dur = start.elapsed();
            results_greedy_side.push((state.solution.boxes.len(), dur));

            // Trivial bad start solution, one bin for one rectangle
            let trivial_sol = create_trivial_solution(&instance);

            // Local Search Geometric
            let start = Instant::now();
            let neigh_geo = GeometricNeighborhood;
            let sol_geo = algorithms::local_search::solve(trivial_sol.clone(), &neigh_geo);
            let dur = start.elapsed();
            results_ls_geo.push((sol_geo.boxes.len(), dur));

            // Local Search Rule Based
            // Start with random permutation
            let mut rects_perm = instance.rects.clone();
            rects_perm.shuffle(&mut rng());
            let start_perm = PermutationSolution::new(instance.clone(), rects_perm);
            
            let start = Instant::now();
            let neigh_rule = RuleBasedNeighborhood::new(Some(50)); 
            let sol_perm = algorithms::local_search::solve(start_perm, &neigh_rule);
            let dur = start.elapsed();
            results_ls_rule.push((sol_perm.cost().0, dur));

            // Local Search Overlapping
            let start = Instant::now();
            let sol_overlap = run_overlapping_ls(trivial_sol.clone());
            let dur = start.elapsed();
            results_ls_overlap.push((sol_overlap.boxes.len(), dur));
        }

        print_stats("Greedy SortByArea", &results_greedy_area);
        print_stats("Greedy SortByMaxSide", &results_greedy_side);
        print_stats("Local Search Geometric", &results_ls_geo);
        print_stats("Local Search Permutation", &results_ls_rule);
        print_stats("Local Search Overlap", &results_ls_overlap);
    }
}

/// Create trivial solution: each rectangle in one box
fn create_trivial_solution(instance: &Instance) -> RectangleSolution {
    let mut sol = RectangleSolution::new(instance.clone());
    for r in &instance.rects {
        let mut b = BoxBin::new(instance.box_size);
        // Place it at left-bottom
        b.try_place(*r, 0, 0, false); 
        sol.boxes.push(b);
    }
    sol
}

/// Run Overlap Local Search with decreasing overlapping percentage
fn run_overlapping_ls(start_sol: RectangleSolution) -> RectangleSolution {
    // Start parameter
    let mut current_sol = start_sol.with_penalty(10); 
    let mut percent = 1.0; // 100% start overlapping
    
    let steps = 10;
    
    for _ in 0..steps {
        let neigh = OverlappingNeighborhood { max_overlap_percent: percent };
        // Local Search for this level
        current_sol = algorithms::local_search::solve(current_sol, &neigh);
        // Tighten parameter
        percent -= 1.0 / (steps as f64);
        if percent < 0.0 { percent = 0.0; }
        // Increase penalty
        if let Some(p) = current_sol.penalty_factor {
             current_sol.penalty_factor = Some(p * 5);
        }
    }
    // Solve last time without penalty factor
    let mut strict_sol = current_sol;
    strict_sol.penalty_factor = None; 
    
    algorithms::local_search::solve(strict_sol, &GeometricNeighborhood)
}

/// Helping function to print statistics
fn print_stats(name: &str, results: &[(usize, Duration)]) {
    if results.is_empty() { return; }
    
    let avg_boxes: f64 = results.iter().map(|r| r.0 as f64).sum::<f64>() / results.len() as f64;
    let avg_time: f64 = results.iter().map(|r| r.1.as_millis() as f64).sum::<f64>() / results.len() as f64;
    
    println!("{:<25} | {:<12.2} | {:<15.2}", name, avg_boxes, avg_time);
}

