use crate::functions::analyse_final::analyse_final;
use crate::functions::enqueue::enqueue;
use crate::functions::new_clause::new_clause;
use crate::functions::search::var_rescale_activity;

use crate::models::clause::Clause;
use crate::models::lbool::Lbool;
use crate::models::lit::*;
use crate::models::statsparams::*;
use crate::models::varorder::VarOrder;

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
    pub moo: MOO,
    // SOLVING
    pub progress_estimate: f64,
    pub mode: Vec<Lbool>,
    pub conflict: Vec<Lit>,
}

pub trait Internal {
    fn i_analyze_final(&mut self, confl: Clause);
    fn i_enqueue(&mut self, fact: Lit) -> bool;
    fn var_bump_activity(&self, p: Lit);
    fn var_decay_activity(self);
    fn cla_decay_activity(self);
    fn i_new_clause(self, ps: &mut Vec<Lit>);
    fn cla_bump_activity(&mut self, c: Clause);
    fn remove(c: Clause);
    fn locked(c: Clause) -> bool;
    fn decision_level(&mut self) -> i32;
}

impl Internal for SolverState {
    fn i_analyze_final(&mut self, confl: Clause) {
        analyse_final(confl, false, self);
    }
    fn i_enqueue(&mut self, fact: Lit) -> bool {
        return enqueue(&fact, None, self);
    }
    fn var_bump_activity(&self, p: Lit) {
        if self.var_decay < 0.0 {
            return;
        }
        let index = var(&p) as f64 + self.var_inc;
        if self.activity[index as usize] > 1e100 {
            var_rescale_activity();
        }
    }
    fn var_decay_activity(mut self) {
        if self.var_decay >= 0.0 {
            self.var_inc *= self.var_decay;
        }
    }
    fn cla_decay_activity(mut self) {
        self.cla_inc *= self.cla_decay;
    }
    fn i_new_clause(mut self, ps: &mut Vec<Lit>) {
        new_clause(ps, false, &mut self);
    }
    fn cla_bump_activity(&mut self, c: Clause) {}
    fn remove(_c: Clause) {}
    fn locked(_c: Clause) -> bool {
        return true;
    }
    fn decision_level(&mut self) -> i32 {
        return 5;
    }
}

pub fn move_back(l1: Lit, l2: Lit) {}

pub fn locked(c: Clause, solver_state: &mut SolverState) -> bool {
    match &solver_state.reason[var(&c.data[0]) as usize] {
        Some(x) => {
            return c == *x;
        }
        _ => false,
    }
}
