extern crate sys_info;
use sys_info::*;

pub fn mem_used() -> MemInfo {
    return mem_info().unwrap();
}
