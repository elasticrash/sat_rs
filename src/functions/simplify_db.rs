use crate::functions::new_clause::*;
use crate::functions::propagate::*;
use crate::models::clause::*;
use crate::models::lit::*;
use crate::models::logger::*;
use crate::models::solverstate::*;

/*_________________________________________________________________________________________________
|
|  simplifyDB : [void]  .  [bool]
|
|  Description:
|    Simplify the clause database according to the current top-level assigment. Currently, the only
|    thing done here is the removal of satisfied clauses, but more things can be put here.
|________________________________________________________________________________________________@*/

pub fn simplify_db(solver_state: &mut SolverState) {
    reportf("simplify_db".to_string());

    if !solver_state.ok {
        return;
    }
    match propagate(solver_state) {
        None => {
            reportf("propagate match none".to_string());
            if solver_state.clone().n_assigns() == solver_state.simp_db_assigns as usize
                || solver_state.simp_db_props > 0.0
            {
                return;
            }

            for y in solver_state.simp_db_assigns..solver_state.clone().n_assigns() as i32 {
                let _p: Lit = solver_state.trail[y as usize];
                solver_state.watches[index(_p) as usize].clear();
                solver_state.watches[index(!_p) as usize].clear();
            }

            for t in 0..2 {
                let mut cs: Vec<Clause>;
                if t != 0 {
                    cs = solver_state.learnts.clone();
                } else {
                    cs = solver_state.clauses.clone();
                }

                let mut j: i32 = 0;
                for k in 0..cs.len() {
                    if solver_state.locked(cs[k].clone()) && simplify(cs[k].clone(), solver_state) {
                        remove(cs[k].clone(), false, solver_state);
                    } else {
                        cs[j as usize] = cs[k].clone();
                        j += 1;
                    }
                }
                cs.truncate(cs.len() - j as usize);
            }

            solver_state.simp_db_assigns = solver_state.clone().n_assigns() as i32;
            solver_state.simp_db_props = solver_state.solver_stats.clauses_literals
                + solver_state.solver_stats.learnts_literals;
        }
        _ => {
            reportf("solver state false".to_string());
            solver_state.ok = false;
            return;
        }
    }
}
