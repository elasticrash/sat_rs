use crate::models::logger::*;
use crate::models::statsparams::*;
use crate::models::timemem::*;

pub fn progress_estimate() -> f64 {
    return 0.0;
}

pub fn print_stats(stats: SolverStats) {
    reportf(format!("restarts              : {0}\n", stats.starts));
    reportf(format!("conflicts             : {0}\n", stats.conflicts,));
    reportf(format!("decisions             : {0}\n", stats.decisions,));
    reportf(format!("propagations          : {0}\n", stats.propagations,));
    reportf(format!(
        "conflict literals     : {0}   ({1} %% deleted)\n",
        stats.tot_literals,
        (stats.max_literals - stats.tot_literals) * 100.0 / stats.max_literals
    ));
    if mem_used() != 0 {
        reportf(format!("Memory used           : {0} MB\n", mem_used()));
    }
}
