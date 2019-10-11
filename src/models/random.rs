fn drand(mut seed: f64) -> f64 {
    seed *= 1389796 as f64;
    let q: i32 = (seed / 2147483647 as f64) as i32;
    seed -= q as f64 * 2147483647 as f64;
    return seed / 2147483647 as f64;
}

fn irand(seed: f64, size: i32) -> i32 {
    return drand(seed * size as f64) as i32 ;
}
