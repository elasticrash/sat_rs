pub struct SolverStats {
    starts: f32,
    decisions: f32,
    propagations: f32,
    conflicts: f32,
    clauses_literals: f32,
    learnts_literals: f32,
    max_literals: f32,
    tot_literals: f32,
}

pub struct SearchParams {
    var_decay: f32,
    clause_decay: f32,
    random_var_freq: f32,
}

pub trait ISearchParams {
    fn new(mut self, v: f32, c: f32, r: f32);
    fn clone(mut self, other: SearchParams);
    fn unit(mut self);
}

impl ISearchParams for SearchParams {
    fn new(mut self, v: f32, c: f32, r: f32) {
        self.var_decay = v;
        self.clause_decay = c;
        self.random_var_freq = r;
    }
    fn clone(mut self, other: SearchParams) {
        self.var_decay = other.var_decay;
        self.clause_decay = other.clause_decay;
        self.random_var_freq = other.random_var_freq;
    }
    fn unit(mut self) {
        self.new(1.0, 1.0, 0.0);
    }
}
