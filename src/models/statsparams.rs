#[derive(Copy, Clone)]
pub struct SolverStats {
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
