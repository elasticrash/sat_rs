use crate::models::logger::*;
use crate::models::statsparams::*;
use crate::models::timemem::*;

pub fn progress_estimate() -> f64 {
    return 0.0;
}

pub fn print_stats(stats: SolverStats) {
    reportf("print_stats".to_string(), 0);

    let cpu_time = cpu_time() - stats.start_time;
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
    if mem_used() != 0 {
        reportf(format!("Memory used           : {0} MB", mem_used()), 2);
    }
    reportf(format!("CPU time              : {0} s", cpu_time), 2);
}
