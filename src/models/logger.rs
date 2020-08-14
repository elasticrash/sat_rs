pub fn reportf(a: String, file: &str, line: u32, verbosity: i32) {
    if verbosity > 1 {
        println!("{}:{}: {:?}", file, line, a);
    }
}
