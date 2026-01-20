use optalgos_program::testing::{self, TestConfig};

fn main() {
    println!("=== OptAlgos Program Test Suite ===");

    // Small Instances
    println!("\n>>> Mode 1: Small Instances");
    let tests_demo = vec![
        TestConfig {
            num_instances: 5,
            num_rects: 30,
            width_range: (5, 20),
            height_range: (5, 20),
            box_size: 40, 
        },
        TestConfig {
            num_instances: 5,
            num_rects: 100,
            width_range: (10, 30),
            height_range: (10, 30),
            box_size: 100,
        }
    ];
    testing::run_suite(&tests_demo);

    // Big Instances for protocol
    println!("\n>>> Mode 2: Big Instances)");    
    let tests_large = vec![
         TestConfig {
            num_instances: 3,
            num_rects: 500,
            width_range: (10, 50),
            height_range: (10, 50),
            box_size: 150,
        },
        TestConfig {
            // 1000 Rect test
            num_instances: 1,
            num_rects: 1000,
            width_range: (10, 80),
            height_range: (10, 80),
            box_size: 300, 
        }
    ];
    testing::run_suite(&tests_large);
    
    println!("\n=== Tests completed! ===");
}

