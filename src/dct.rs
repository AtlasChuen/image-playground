use std::f64::consts::PI;

fn one_square_root_of_two() -> f64{
    return 1.0/(2f64).sqrt();
}

// type-II DCT
pub fn dct(nums: &mut [f64; 64]) {
    let size = 8; 
    let mut coefficients = Vec::new();
    for v in 0..size {
        for u in 0..size {
            let au = if u == 0 { one_square_root_of_two() } else { 1.0 };
            let av = if v == 0 { one_square_root_of_two() } else { 1.0 };
            let mut sum = 0.0;
            for y in 0..size {
                for x in 0..size {
                    sum +=
                        nums[(y * size + x) as usize] *
                            ((((2 * x + 1) * u) as f64 * PI) / 16.0).cos() *
                            ((((2 * y + 1) * v) as f64 * PI) / 16.0).cos();
                }
            }
            
            coefficients.push((sum * au * av) / 4.0);
        }
    }
    
    // in-place update
    for i in 0..coefficients.len() {
        nums[i] = coefficients[i];
    }
    
}

// type-III DCT
pub fn idct(coefficients : &mut [f64; 64]) {
    let size = 8;
    let mut nums = Vec::new();
    for y in 0..size {
        for x in 0..size {
            let mut sum = 0 as f64;
            for v in 0..size {
                for u in 0..size {
                    let au = if u == 0 { one_square_root_of_two() } else { 1.0 };
                    let av = if v == 0 { one_square_root_of_two() } else { 1.0 };
                    sum +=
                        au *
                            av *
                            coefficients[v * size + u] *
                            (((2 as f64 * x as f64 + 1.0) * u as f64 * PI) / 16.0).cos() *
                            (((2 as f64 * y as f64 + 1.0) * v as f64 * PI) / 16.0).cos();
                }
            }
            nums.push(sum / 4 as f64);
        }
    }
    // in-place update
    for i in 0.. nums.len() {
        coefficients[i] = nums[i];
    }
} 

