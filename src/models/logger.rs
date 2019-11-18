pub fn reportf(a: String, verbosity: i32) {
    if verbosity > 1 {
        println!("{:?}", a);
    }
}

pub fn debug(a: String) {
    println!("{:?}", a);
}
