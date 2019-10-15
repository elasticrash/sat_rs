pub struct SolverStats {
    starts: f64,
    decisions: f64,
    propagations: f64,
    conflicts: f64,
    clauses_literals: f64,
    learnts_literals: f64,
    max_literals: f64,
    tot_literals: f64,
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
