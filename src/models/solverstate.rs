use crate::functions::analyse_final::analyse_final;
use crate::functions::enqueue::enqueue;
use crate::functions::new_clause::new_clause;
use crate::functions::search::var_rescale_activity;

use crate::models::clause::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::statsparams::*;
use crate::models::varorder::*;

#[derive(Clone)]
pub struct SolverState {
    pub ok: bool,
    pub clauses: Vec<Clause>,
    pub learnts: Vec<Clause>,
    pub cla_inc: f64,
    pub cla_decay: f64,
    pub activity: Vec<f64>,
    pub var_inc: f64,
    pub var_decay: f64,
    pub order: VarOrder,
    pub watches: Vec<Vec<Clause>>,
    pub assigns: Vec<Lbool>,
    pub trail: Vec<Lit>,
    pub trail_lim: Vec<i32>,
    pub reason: Vec<Option<Clause>>,
    pub level: Vec<i32>,
    pub trail_pos: Vec<i32>,
    pub root_level: i32,
    pub qhead: i32,
    pub simp_db_assigns: i32,
    pub simp_db_props: f64,
    pub analyze_seen: Vec<Lbool>,
    pub analyze_stack: Vec<Lit>,
    pub analyze_toclear: Vec<Lit>,
    pub add_unit_tmp: Vec<Lit>,
    pub add_binary_tmp: Vec<Lit>,
    pub add_ternary_tmp: Vec<Lit>,
    // DLPP(T)
    pub level_to_backtrack: i32,
    pub solver_stats: SolverStats,
    // SOLVING
    pub progress_estimate: f64,
    pub model: Vec<Lbool>,
    pub conflict: Vec<Lit>,
    //MOO
    pub default_parms: SearchParams,
    pub expensive_ccmin: bool,
    pub verbosity: i32,
}

pub trait NewState {
    fn new() -> Self;
}

impl NewState for SolverState {
    fn new() -> Self {
        let mut solver = Self {
            clauses: Vec::new(),
            learnts: Vec::new(),
            activity: Vec::new(),
            watches: Vec::new(),
            assigns: Vec::new(),
            trail_pos: Vec::new(),
            trail: Vec::new(),
            trail_lim: Vec::new(),
            reason: Vec::new(),
            level: Vec::new(),
            analyze_seen: Vec::new(),
            analyze_stack: Vec::new(),
            analyze_toclear: Vec::new(),
            add_unit_tmp: Vec::new(),
            add_binary_tmp: Vec::new(),
            add_ternary_tmp: Vec::new(),
            model: Vec::new(),
            conflict: Vec::new(),
            solver_stats: SolverStats::new(),
            ok: true,
            cla_inc: 1.0,
            cla_decay: 1.0,
            var_inc: 1.0,
            var_decay: 1.0,
            order: VarOrder::new(Vec::new(), Vec::new()),
            qhead: 0,
            simp_db_assigns: 0,
            simp_db_props: 0.0,
            default_parms: SearchParams {
                var_decay: 0.95,
                clause_decay: 0.999,
                random_var_freq: 0.02,
            },
            expensive_ccmin: true,
            verbosity: 0,
            progress_estimate: 0.0,
            root_level: 0,
            level_to_backtrack: 0,
        };

        solver.add_unit_tmp.resize(2, Lit::new(-1, false));
        solver.add_binary_tmp.resize(2, Lit::new(-1, false));
        solver.add_ternary_tmp.resize(3, Lit::new(-1, false));
        return solver;
    }
}

#[derive(Copy, Clone)]
pub struct SearchParams {
    pub var_decay: f64,
    pub clause_decay: f64,
    pub random_var_freq: f64,
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

pub trait Internal {
    fn i_analyze_final(&mut self, confl: Clause);
    fn i_enqueue(&mut self, fact: Lit) -> bool;
    fn var_bump_activity(&mut self, p: Lit);
    fn var_decay_activity(&mut self);
    fn cla_decay_activity(&mut self);
    fn i_new_clause(self, ps: &mut Vec<Lit>);
    fn cla_bump_activity(&mut self, c: Clause);
    fn remove(c: Clause);
    fn locked(&mut self, c: Clause) -> bool;
    fn decision_level(&mut self) -> i32;
}

pub trait NewVar {
    fn n_vars(&mut self) -> i32;
    fn add_unit(p: Lit);
    fn add_binary(p: Lit, q: Lit);
    fn add_ternary(p: Lit, q: Lit, r: Lit);
    fn add_clause(ps: Vec<Lit>);
}

pub trait SemiInternal {
    fn n_assigns(self) -> usize;
    fn n_clauses(self) -> usize;
    fn n_learnts(self) -> usize;
}

impl Internal for SolverState {
    fn i_analyze_final(&mut self, confl: Clause) {
        analyse_final(confl, false, self);
    }
    fn i_enqueue(&mut self, fact: Lit) -> bool {
        return enqueue(&fact, None, self);
    }
    fn var_bump_activity(&mut self, p: Lit) {
        if self.var_decay < 0.0 {
            return;
        }
        let index = var(&p) as f64 + self.var_inc;
        if self.activity[index as usize] > 1e100 {
            var_rescale_activity(self);
        }
    }
    fn var_decay_activity(&mut self) {
        if self.var_decay >= 0.0 {
            self.var_inc *= self.var_decay;
        }
    }
    fn cla_decay_activity(&mut self) {
        self.cla_inc *= self.cla_decay;
    }
    fn i_new_clause(mut self, ps: &mut Vec<Lit>) {
        new_clause(ps, false, &mut self);
    }
    fn cla_bump_activity(&mut self, c: Clause) {}
    fn remove(_c: Clause) {}
    fn locked(&mut self, _c: Clause) -> bool {
        match &self.reason[var(&_c.data[0]) as usize] {
            Some(x) => {
                return _c == *x;
            }
            _ => false,
        }
    }
    fn decision_level(&mut self) -> i32 {
        return 5;
    }
}

impl SemiInternal for SolverState {
    fn n_assigns(self) -> usize {
        return self.trail.len();
    }
    fn n_clauses(self) -> usize {
        return self.clauses.len();
    }
    fn n_learnts(self) -> usize {
        return self.clauses.len();
    }
}

impl NewVar for SolverState {
    fn n_vars(&mut self) -> i32 {
        return self.assigns.len() as i32;
    }
    fn add_unit(p: Lit) {}
    fn add_binary(p: Lit, q: Lit) {}
    fn add_ternary(p: Lit, q: Lit, r: Lit) {}
    fn add_clause(ps: Vec<Lit>) {}
}

pub fn move_back(_l1: Lit, _l2: Lit) {}
