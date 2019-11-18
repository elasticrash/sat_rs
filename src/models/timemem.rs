use std::time::Instant;
extern crate sys_info;
use sys_info::*;

pub fn mem_used() -> u64 {
    let mem = mem_info().unwrap();
    return mem.total;
}
