use crate::functions::analyse_final::analyse_final;
use crate::functions::enqueue::enqueue;
use crate::functions::search::var_rescale_activity;
use crate::functions::new_clause::new_clause;

use crate::models::clause::Clause;
use crate::models::lbool::Lbool;
use crate::models::lit::*;
use crate::models::varorder::VarOrder;

pub struct SolverState {
    ok: bool,
    clasues: Vec<Clause>,
    cla_inc: f64,
    cla_decay: f64,
    activity: Vec<f64>,
    var_inc: f64,
    var_decay: f64,
    order: VarOrder,
    watches: Vec<Vec<Clause>>,
    assigns: Vec<Lbool>,
    trail_lim: Vec<i32>,
    reason: Vec<Clause>,
    level: Vec<i32>,
    trail_pos: Vec<i32>,
    root_level: i32,
    qhead: i32,
    simp_db_assigns: i32,
    simp_db_props: f64,
    analyze_seen: Vec<Lbool>,
    analyze_stack: Vec<Lit>,
    analyze_toclear: Vec<Lit>,
    add_unit_tmp: Vec<Lit>,
    add_binary_tmp: Vec<Lit>,
    add_ternary_tmp: Vec<Lit>,
}

pub trait Internal {
    fn i_analyze_final(confl: Clause);
    fn i_enqueue(fact: Lit) -> bool;
    fn var_bump_activity(&self, p: Lit);
    fn var_decay_activity(self);
    fn cla_decay_activity(self);
    fn i_new_clause(ps: Vec<Lit>);
    fn cla_bump_activity(c: Clause);
    fn remove(c: Clause);
    fn locked(c: Clause) -> bool;
    fn decision_level() -> i32;
}

impl Internal for SolverState {
    fn i_analyze_final(confl: Clause) {
        analyse_final(confl, false);
    }
    fn i_enqueue(fact: Lit) -> bool {
        return enqueue(fact, None);
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
    fn i_new_clause(ps: Vec<Lit>) {
        new_clause(ps, false);
    }
    fn cla_bump_activity(c: Clause) {}
    fn remove(c: Clause) {}
    fn locked(c: Clause) -> bool {
        return true;
    }
    fn decision_level() -> i32 {
        return 5;
    }
}
