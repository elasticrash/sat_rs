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

pub struct SearchParams {
    var_decay: f64,
    clause_decay: f64,
    random_var_freq: f64,
}

pub trait ISearchParams {
    fn new(self, v: f64, c: f64, r: f64);
    fn clone(self, other: SearchParams);
    fn unit(self);
}

impl ISearchParams for SearchParams {
    fn new(mut self, v: f64, c: f64, r: f64) {
        self.var_decay = v;
        self.clause_decay = c;
        self.random_var_freq = r;
    }
    fn clone(mut self, other: SearchParams) {
        self.var_decay = other.var_decay;
        self.clause_decay = other.clause_decay;
        self.random_var_freq = other.random_var_freq;
    }
    fn unit(self) {
        self.new(1.0, 1.0, 0.0);
    }
}
