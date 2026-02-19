pub fn drand(seed: &mut f64) -> f64 {
    *seed *= 1389796_f64;
    let q: i32 = (*seed / 2147483647_f64) as i32;
    *seed -= q as f64 * 2147483647_f64;
    *seed / 2147483647_f64
}

pub fn irand(seed: &mut f64, size: i32) -> i32 {
    (drand(seed) * size as f64) as i32
}
