use crate::functions::analyse_final::*;
use crate::functions::enqueue::*;
use crate::functions::new_clause::*;
use crate::functions::search::*;

use crate::models::clause::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::logger::*;
use crate::models::statsparams::*;
use crate::models::varorder::*;

use std::cmp;

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
        let ass: Vec<Lbool> = Vec::new();
        let act: Vec<f64> = Vec::new();

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
            order: VarOrder::new(ass, act),
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

#[derive(Copy, Clone, Debug)]
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
    fn cla_bump_activity(&mut self, c: &mut Clause);
    fn locked(&mut self, _c: &Clause) -> bool;
    fn decision_level(&mut self) -> i32;
}

pub trait NewVar {
    fn n_vars(&mut self) -> i32;
    fn add_unit(&mut self, p: Lit);
    fn add_binary(&mut self, p: Lit, q: Lit);
    fn add_ternary(&mut self, p: Lit, q: Lit, r: Lit);
    fn add_clause(&mut self, ps: &mut Vec<Lit>);
}

pub trait SemiInternal {
    fn n_assigns(self) -> usize;
    fn n_clauses(self) -> usize;
    fn n_learnts(self) -> usize;
}

impl Internal for SolverState {
    fn i_analyze_final(&mut self, confl: Clause) {
        self.analyse_final(&confl, false);
    }
    fn i_enqueue(&mut self, fact: Lit) -> bool {
        return self.enqueue(&fact, None);
    }
    fn var_bump_activity(&mut self, p: Lit) {
        if self.var_decay < 0.0 {
            return;
        }
        let index: i32 = var(&p);
        self.activity[index as usize] += self.var_inc;
        if self.activity[index as usize] > 1e100 {
            self.var_rescale_activity();
        }
        self.order.update(var(&p));
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
        self.new_clause(ps, false);
    }
    fn cla_bump_activity(&mut self, c: &mut Clause) {
        c.activity += self.cla_inc;
        if c.activity > 1e20 {
            self.cla_rescale_activity();
        }
    }
    fn locked(&mut self, _c: &Clause) -> bool {
        return match &self.reason[var(&_c.data[0]) as usize] {
            Some(x) => _c == x,
            _ => false,
        };
    }
    fn decision_level(&mut self) -> i32 {
        return self.trail_lim.len() as i32;
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
        return self.learnts.len();
    }
}

impl NewVar for SolverState {
    fn n_vars(&mut self) -> i32 {
        return self.assigns.len() as i32;
    }
    fn add_unit(&mut self, _p: Lit) {
        self.add_unit_tmp[0] = _p;
        self.add_clause(&mut self.add_unit_tmp.clone());
    }
    fn add_binary(&mut self, _p: Lit, _q: Lit) {
        self.add_binary_tmp[0] = _p;
        self.add_binary_tmp[1] = _q;
        self.add_clause(&mut self.add_binary_tmp.clone());
    }
    fn add_ternary(&mut self, _p: Lit, _q: Lit, _r: Lit) {
        self.add_ternary_tmp[0] = _p;
        self.add_ternary_tmp[1] = _q;
        self.add_ternary_tmp[1] = _r;
        self.add_clause(&mut self.add_ternary_tmp.clone());
    }
    fn add_clause(&mut self, ps: &mut Vec<Lit>) {
        reportf("add_clause".to_string(), file!(), line!(), self.verbosity);
        self.new_clause(ps, false);
    }
}

pub fn move_back(_l1: Lit, _l2: Lit, solver_state: &mut SolverState) {
    reportf(
        "move_back".to_string(),
        file!(),
        line!(),
        solver_state.verbosity,
    );

    let mut lev1: i32 = solver_state.level[var(&_l1) as usize];
    let mut lev2: i32 = solver_state.level[var(&_l2) as usize];
    if lev1 == -1 {
        lev1 = i32::max_value();
    }
    if lev2 == -1 {
        lev2 = i32::max_value();
    }

    if lev1 < solver_state.level_to_backtrack || lev2 < solver_state.level_to_backtrack {
        if value_by_lit(_l1, solver_state) == Lbool::True {
            if value_by_lit(_l2, solver_state) == Lbool::True {
            } else if lev1 <= lev2 || solver_state.level_to_backtrack <= lev2 {
            } else {
                solver_state.level_to_backtrack = lev2;
            }
        } else {
            if value_by_lit(_l2, solver_state) == Lbool::True {
                if lev2 <= lev1 || solver_state.level_to_backtrack <= lev1 {
                } else {
                    solver_state.level_to_backtrack = lev1;
                }
            } else {
                solver_state.level_to_backtrack = cmp::min(lev1, lev2);
            }
        }
    }
}
