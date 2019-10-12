use std::time::Instant;
extern crate sys_info;
use sys_info::*;

fn cpu_time() -> u64 {
    let now = Instant::now();
    return now.elapsed().as_secs();
}

fn mem_used() -> u64 {
    let mem = mem_info().unwrap();
    return mem.total;
}
