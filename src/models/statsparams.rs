use std::fmt::Display;
use std::time::Instant;
use sys_info::mem_info;
use sys_info::MemInfo;

#[derive(Copy, Clone)]
pub struct SolverStats {
    pub start_time: Instant,
    pub starts: f64,
    pub decisions: f64,
    pub propagations: f64,
    pub conflicts: f64,
    pub clauses_literals: f64,
    pub learnts_literals: f64,
    pub max_literals: f64,
    pub tot_literals: f64,
}

pub trait ISolverStats {
    fn new() -> Self;
}

impl ISolverStats for SolverStats {
    fn new() -> Self {
        return Self {
            start_time: Instant::now(),
            starts: 0.0,
            decisions: 0.0,
            propagations: 0.0,
            conflicts: 0.0,
            clauses_literals: 0.0,
            learnts_literals: 0.0,
            max_literals: 0.0,
            tot_literals: 0.0,
        };
    }
}

impl Display for SolverStats {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let duration: u64 = (self.start_time.elapsed().subsec_nanos() / 1000000) as u64;
        println!("restarts              : {0}", self.starts);
        println!(
            "conflicts             : {0}    ({1}/ms)",
            self.conflicts,
            (self.conflicts / duration as f64)
        );
        println!(
            "decisions             : {0}    ({1}/ms)",
            self.decisions,
            (self.decisions / duration as f64)
        );
        println!(
            "propagations          : {0}    ({1}/ms)",
            self.propagations,
            (self.propagations / duration as f64)
        );
        println!(
            "conflict literals     : {0}   ({1} %% deleted)",
            self.tot_literals,
            (self.max_literals - self.tot_literals) * 100.0 / self.max_literals
        );
        println!(
            "Available Memory      : {0} / {1} MB",
            mem_used().free,
            mem_used().total
        );
        println!("CPU time              : {0} ms", duration);

        Ok(())
    }
}

pub fn mem_used() -> MemInfo {
    return mem_info().unwrap();
}