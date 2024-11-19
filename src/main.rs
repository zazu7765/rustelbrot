use num_complex::Complex;

/// Determine if C is in set, using `threshold` iterations to limit computation
/// 
/// If not a member, return number of iterations taken to leave circle (centered on origin).
/// 
/// Else, return None
fn complex_square(C: Complex<f64>, threshold: usize) -> Option<usize>{
    assert!(threshold != 0);
    assert!(C != Complex{re:0.0, im:0.0});
    let mut z: Complex<f64> = Complex{im:0.0, re: 0.0};

    for i in 0..threshold{
        if z.norm_sqr() > 4_f64{
            return Some(i);
        }

        z = z * z + C;
    }
    None
}


fn main() {
    println!("Hello, world!");
}
