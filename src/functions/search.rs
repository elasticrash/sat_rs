use crate::functions::analyse::*;
use crate::functions::analyse_final::*;
use crate::functions::dpll::*;
use crate::functions::new_clause::*;
use crate::functions::propagate::*;
use crate::functions::reduce_db::*;
use crate::functions::simplify_db::*;
use crate::functions::solve::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::solverstate::*;
use crate::models::varorder::*;
use std::cmp::max;

/*_________________________________________________________________________________________________
|
|  search
|
|  Description:
|    Search for a model the specified number of conflicts, keeping the number of learnt clauses
|    below the provided limit. NOTE! Use negative value for 'nof_conflicts' or 'nof_learnts' to
|    indicate infinity.
|
|  Output:
|    'l_True' if a partial assigment that is consistent with respect to the clauseset is found. If
|    all variables are decision variables, this means that the clause set is satisfiable. 'l_False'
|    if the clause set is unsatisfiable. 'l_Undef' if the bound on number of conflicts is reached.
|________________________________________________________________________________________________@*/
pub trait Search {
    fn search(&mut self, nof_conflicts: i32, nof_learnts: i32, parms: SearchParams) -> Lbool;
    fn var_rescale_activity(&mut self);
    fn cla_rescale_activity(&mut self);
}

impl Search for SolverState {
    fn search(&mut self, nof_conflicts: i32, nof_learnts: i32, parms: SearchParams) -> Lbool {
        trace!(
            "{}|{}|{}|{}|{}",
            "search".to_string(),
            file!(),
            line!(),
            nof_conflicts,
            nof_learnts
        );

        if !self.ok {
            return Lbool::False;
        }
        assert!(self.root_level == self.decision_level());

        self.solver_stats.starts += 1.0;
        let mut conflict_c: i32 = 0;
        self.var_decay = 1.0 / parms.var_decay;
        self.cla_decay = 1.0 / parms.clause_decay;
        self.model.clear();

        loop {
            match self.propagate() {
                Some(_c) => {
                    self.solver_stats.conflicts += 1.0;
                    conflict_c += 1;
                    let mut learnt_clause: Vec<Lit> = Vec::new();

                    if self.decision_level() == self.root_level {
                        self.analyse_final(&_c, false);
                        return L_FALSE;
                    }

                    let backtrack_level: i32 = self.analyze(Some(_c.clone()), &mut learnt_clause);

                    self.cancel_until(max(backtrack_level, self.root_level));
                    self.new_clause(&mut learnt_clause, true);
                    if learnt_clause.len() == 1 {
                        self.level[var(&learnt_clause[0]) as usize] = 0;
                    }
                    self.var_decay_activity();
                    self.cla_decay_activity();
                }
                None => {
                    if nof_conflicts >= 0 && conflict_c >= nof_conflicts {
                        self.progress_estimate = self.progress_estimate();
                        self.cancel_until(self.root_level);
                        return Lbool::Undef0;
                    }

                    if self.decision_level() == 0 {
                        self.simplify_db();
                        if !self.ok {
                            return L_FALSE;
                        }
                    }

                    if nof_learnts >= 0
                        && self.learnts.len() as i32 - self.trail.len() as i32 >= nof_learnts
                    {
                        self.reduce_db();
                    }

                    self.solver_stats.decisions += 1.0;
                    let next: Lit = self.order.select(parms.random_var_freq);

                    if next == Lit::undefined() {
                        if self.model_found() {
                            continue;
                        }
                        self.model
                            .resize(self.clone().n_vars() as usize, Lbool::Undef0);

                        for y in 0..self.clone().n_vars() {
                            self.model[y as usize] = self.value_by_var(y);
                        }
                        self.cancel_until(self.root_level);
                        return L_TRUE;
                    }

                    assert!(self.assume(next));
                }
            }
        }
    }

    fn var_rescale_activity(&mut self) {
        trace!(
            "{}|{}|{}",
            "var_rescale_activity".to_string(),
            file!(),
            line!(),
        );

        for y in 0..self.clone().n_vars() {
            self.update_activity(self.activity.col[y as usize] * 1e-100, y as usize);
        }
        self.var_inc *= 1e-100;
    }

    fn cla_rescale_activity(&mut self) {
        trace!(
            "{}|{}|{}",
            "cla_rescale_activity".to_string(),
            file!(),
            line!()
        );

        for y in 0..self.learnts.len() {
            self.learnts[y as usize].activity *= 1e-20;
        }
        self.cla_inc *= 1e-20;
    }
}
