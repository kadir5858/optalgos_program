use rand::Rng;
use crate::problem::instance::Instance;
use crate::problem::rect::Rect;

pub struct Generator;

impl Generator {
    pub fn generate_instance(num_rects: usize, width_range: (u32, u32), height_range: (u32, u32), box_size: u32) -> Instance {
        let mut rng = rand::rng();
        let mut rects = Vec::with_capacity(num_rects);

        let (min_w, max_w) = width_range;
        let (min_h, max_h) = height_range;
        // Test values
        assert!(min_w <= max_w, "Min width must be <= max width");
        assert!(min_h <= max_h, "Min height must be <= max height");

        for i in 0..num_rects {
            // Random width and height in interval inclusive both borders
            let width = rng.random_range(min_w..=max_w);
            let height = rng.random_range(min_h..=max_h);
            // Check box limit L
            assert!(width <= box_size && height <= box_size, 
                "Generated rectangle ({}, {}) doesn't fit in box ({})", width, height, box_size);
            
            rects.push(Rect::new(i, width, height));
        }
        
        Instance::new(box_size, rects)
    }
}