use crate::functions::analyse_final::*;
use crate::functions::new_clause::*;
use crate::functions::propagate::*;
use crate::functions::search::*;
use crate::functions::simplify_db::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::solverstate::*;

/*_________________________________________________________________________________________________
|
|  solve
|
|  Description:
|    Top-level solve. If using assumptions (non-empty 'assumps' vector), you must call
|    'simplifyDB()' first to see that no top-level conflict is present (which would put the solver
|    in an undefined state).
|
|  Input:
|    A list of assumptions (unit clauses coded as literals). Pre-condition: The assumptions must
|    not contain both 'x' and '~x' for any variable 'x'.
|________________________________________________________________________________________________@*/

pub trait Solver {
    fn solve(&mut self, assumptions: Vec<Lit>) -> bool;
    fn solve_no_assumptions(&mut self) -> bool;
    fn progress_estimate(&mut self) -> f64;
}

impl Solver for SolverState {
    fn solve(&mut self, assumptions: Vec<Lit>) -> bool {
        trace!(
            "{}|{}|{}|{:?}",
            "solve".to_string(),
            file!(),
            line!(),
            assumptions
        );

        self.simplify_db();
        if !self.ok {
            return false;
        };

        let parms: SearchParams = self.default_parms;

        let mut nof_conflicts: f64 = 100.0;
        let mut nof_learnts: f64 = self.clauses.len() as f64 / 3.0;
        let mut status: Lbool = Lbool::Undef0;

        self.root_level = assumptions.len() as i32;
        for assume in &assumptions {
            let p: Lit = *assume;
            assert!(var(&p) < self.n_vars());

            if !self.assume(p) {
                match &self.reason[var(&p) as usize] {
                    Some(r) => {
                        self.clone().analyse_final(&(r.clone()), true);
                        self.conflict.push(!p);
                    }
                    None => {
                        self.conflict.clear();
                        self.conflict.push(!p);
                    }
                }

                self.cancel_until(0);
                return false;
            }
            if let Some(confl) = self.propagate() {
                self.analyse_final(&confl, false);
                assert!(!self.conflict.is_empty());
                self.cancel_until(0);
                return false;
            }
        }
        assert!(self.root_level == self.decision_level());

        info!("==================================[MINISAT]=======================================");
        info!(
            "| Conflicts |       ORIGINAL        |              LEARNT              | Progress |"
        );
        info!(
            "|           | Clauses      Literals |   Limit Clauses Literals  Lit/Cl |          |"
        );
        info!("==================================================================================");

        while is_undefined(status) {
            info!(
                    "|      {0}    |     {1}        {2}    |   {3}      {4}       {5}       {6}   |   {7} %   |",
                    self.solver_stats.conflicts,
                    self.clone().n_clauses(),
                    self.solver_stats.clauses_literals,
            nof_learnts.floor(),
                    self.clone().n_learnts(),
                    self.solver_stats.learnts_literals,
                    (self.solver_stats.learnts_literals
                        / self.clone().n_learnts() as f64).floor(),
                        self.progress_estimate * 100.0
            );

            status = self.search(nof_conflicts as i32, nof_learnts as i32, parms);
            nof_conflicts *= 1.5;
            nof_learnts *= 1.1;
        }

        info!("==================================================================================");
        self.cancel_until(0);
        true
    }

    fn solve_no_assumptions(&mut self) -> bool {
        trace!(
            "{}|{}|{}",
            "solve_no_assumptions".to_string(),
            file!(),
            line!(),
        );
        let assumptions: Vec<Lit> = Vec::new();
        self.solve(assumptions)
    }

    fn progress_estimate(&mut self) -> f64 {
        let mut progress = 0.0;
        let f = 1.0 / self.n_vars() as f64;

        for i in 0..self.n_vars() {
            if is_undefined(self.value_by_var(i)) {
                progress += f.powf(self.level[i as usize] as f64);
            }
        }

        progress / self.n_vars() as f64
    }
}
