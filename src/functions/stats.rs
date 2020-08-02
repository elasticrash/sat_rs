use crate::models::statsparams::*;
use crate::models::timemem::*;

pub fn progress_estimate() -> f64 {
    return 0.0;
}

pub fn print_stats(stats: SolverStats) {
    let duration: u64 = (stats.start_time.elapsed().subsec_nanos() / 1000000) as u64;
    println!("restarts              : {0}", stats.starts);
    println!(
        "conflicts             : {0}    ({1}/ms)",
        stats.conflicts,
        (stats.conflicts / duration as f64)
    );
    println!(
        "decisions             : {0}    ({1}/ms)",
        stats.decisions,
        (stats.decisions / duration as f64)
    );
    println!(
        "propagations          : {0}    ({1}/ms)",
        stats.propagations,
        (stats.propagations / duration as f64)
    );
    println!(
        "conflict literals     : {0}   ({1} %% deleted)",
        stats.tot_literals,
        (stats.max_literals - stats.tot_literals) * 100.0 / stats.max_literals
    );
    println!(
        "Available Memory      : {0} / {1} MB",
        mem_used().free,
        mem_used().total
    );
    println!("CPU time              : {0} ms", duration);
}
