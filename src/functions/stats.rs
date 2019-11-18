use crate::models::logger::*;
use crate::models::statsparams::*;
use crate::models::timemem::*;
use std::time::Duration;

pub fn progress_estimate() -> f64 {
    return 0.0;
}

pub fn print_stats(stats: SolverStats) {
    reportf("print_stats".to_string(), 0);

    let duration: u64 = (stats.start_time.elapsed().subsec_nanos() / 1000000) as u64;
    reportf(format!("restarts              : {0}", stats.starts), 2);
    reportf(format!("conflicts             : {0}", stats.conflicts,), 2);
    reportf(format!("decisions             : {0}", stats.decisions,), 2);
    reportf(
        format!("propagations          : {0}", stats.propagations,),
        2,
    );
    reportf(
        format!(
            "conflict literals     : {0}   ({1} %% deleted)",
            stats.tot_literals,
            (stats.max_literals - stats.tot_literals) * 100.0 / stats.max_literals
        ),
        2,
    );
    reportf(
        format!("Available Memory      : {0} / {1} MB", mem_used().free, mem_used().total),
        2,
    );
    reportf(format!("CPU time              : {0} ms", duration), 2);
}
