use crate::functions::analyse_final::*;
use crate::functions::enqueue::*;
use crate::functions::new_clause::*;
use crate::functions::search::*;

use crate::models::clause::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::statsparams::*;
use crate::models::varorder::*;

use std::cmp;

#[derive(Clone)]
pub struct Activity {
    pub col: Vec<f64>,
}

#[derive(Clone)]
pub struct Assigns {
    pub col: Vec<Lbool>,
}

#[derive(Clone)]
pub struct SolverState {
    pub ok: bool,
    pub clauses: Vec<Clause>,
    pub learnts: Vec<Clause>,
    pub cla_inc: f64,
    pub cla_decay: f64,
    pub activity: Activity,
    pub var_inc: f64,
    pub var_decay: f64,
    pub order: VarOrder,
    pub watches: Vec<Vec<Clause>>,
    pub assigns: Assigns,
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
}

pub trait NewState {
    fn new() -> Self;
}

impl NewState for SolverState {
    fn new() -> Self {
        let mut solver = Self {
            clauses: Vec::new(),
            learnts: Vec::new(),
            activity: Activity { col: Vec::new() },
            watches: Vec::new(),
            assigns: Assigns { col: Vec::new() },
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
            order: VarOrder::default(),
            qhead: 0,
            simp_db_assigns: 0,
            simp_db_props: 0.0,
            default_parms: SearchParams {
                var_decay: 0.95,
                clause_decay: 0.999,
                random_var_freq: 0.02,
            },
            expensive_ccmin: true,
            progress_estimate: 0.0,
            root_level: 0,
            level_to_backtrack: 0,
        };

        solver.add_unit_tmp.resize(2, Lit::new(-1, false));
        solver.add_binary_tmp.resize(2, Lit::new(-1, false));
        solver.add_ternary_tmp.resize(3, Lit::new(-1, false));
        solver
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SearchParams {
    pub var_decay: f64,
    pub clause_decay: f64,
    pub random_var_freq: f64,
}

pub trait ISearchParams {
    fn new(v: f64, c: f64, r: f64) -> Self;
    fn clone(self, other: SearchParams);
    fn unit(&mut self);
}

impl ISearchParams for SearchParams {
    fn new(v: f64, c: f64, r: f64) -> Self {
        Self {
            var_decay: v,
            clause_decay: c,
            random_var_freq: r,
        }
    }
    fn clone(mut self, other: SearchParams) {
        self.var_decay = other.var_decay;
        self.clause_decay = other.clause_decay;
        self.random_var_freq = other.random_var_freq;
    }
    fn unit(&mut self) {
        self.var_decay = 1.0;
        self.clause_decay = 1.0;
        self.random_var_freq = 0.0;
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

pub trait Setters {
    fn value_by_var(&mut self, x: i32) -> Lbool;
    fn value_by_lit(&mut self, x: Lit) -> Lbool;
    fn add_activity(&mut self, val: f64);
    fn add_assigns(&mut self, val: Lbool);
    fn update_activity(&mut self, val: f64, i: usize);
    fn update_assigns(&mut self, val: Lbool, i: usize);
}

impl Setters for SolverState {
    fn value_by_var(&mut self, x: i32) -> Lbool {
        self.assigns.col[x as usize]
    }

    fn value_by_lit(&mut self, x: Lit) -> Lbool {
        let mut assign = self.assigns.col[var(&x) as usize];
        if sign(&x) {
            assign = bit_not(assign);
        }
        assign
    }
    fn add_activity(&mut self, val: f64) {
        self.activity.col.push(val);
        self.order.activity.col.push(val);
    }
    fn add_assigns(&mut self, val: Lbool) {
        self.assigns.col.push(val);
        self.order.assigns.col.push(val);
    }
    fn update_activity(&mut self, val: f64, i: usize) {
        self.activity.col[i] = val;
        self.order.activity.col[i] = val;
    }
    fn update_assigns(&mut self, val: Lbool, i: usize) {
        self.assigns.col[i] = val;
        self.order.assigns.col[i] = val;
    }
}

impl Internal for SolverState {
    fn i_analyze_final(&mut self, confl: Clause) {
        self.analyse_final(&confl, false);
    }
    fn i_enqueue(&mut self, fact: Lit) -> bool {
        self.enqueue(&fact, None)
    }
    fn var_bump_activity(&mut self, p: Lit) {
        if self.var_decay < 0.0 {
            return;
        }
        let index: i32 = var(&p);
        self.update_activity(
            self.activity.col[index as usize] + self.var_inc,
            index as usize,
        );
        if self.activity.col[index as usize] > 1e100 {
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
        match &self.reason[var(&_c.data[0]) as usize] {
            Some(x) => _c == x,
            _ => false,
        }
    }
    fn decision_level(&mut self) -> i32 {
        self.trail_lim.len() as i32
    }
}

impl SemiInternal for SolverState {
    fn n_assigns(self) -> usize {
        self.trail.len()
    }
    fn n_clauses(self) -> usize {
        self.clauses.len()
    }
    fn n_learnts(self) -> usize {
        self.learnts.len()
    }
}

impl NewVar for SolverState {
    fn n_vars(&mut self) -> i32 {
        self.assigns.col.len() as i32
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
        trace!(
            "{}|{}|{}|{:?}",
            "add_clause".to_string(),
            file!(),
            line!(),
            ps
        );
        self.new_clause(ps, false);
    }
}

pub fn move_back(_l1: Lit, _l2: Lit, solver_state: &mut SolverState) {
    trace!(
        "{}|{}|{}|{:?}|{:?}",
        "move_back".to_string(),
        file!(),
        line!(),
        _l1,
        _l2
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
        if solver_state.value_by_lit(_l1) == Lbool::True {
            if solver_state.value_by_lit(_l2) == Lbool::True
                || lev1 <= lev2
                || solver_state.level_to_backtrack <= lev2
            {
            } else {
                solver_state.level_to_backtrack = lev2;
            }
        } else if solver_state.value_by_lit(_l2) == Lbool::True {
            if lev2 <= lev1 || solver_state.level_to_backtrack <= lev1 {
            } else {
                solver_state.level_to_backtrack = lev1;
            }
        } else {
            solver_state.level_to_backtrack = cmp::min(lev1, lev2);
        }
    }
}
