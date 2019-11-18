use crate::functions::analyse::*;
use crate::functions::analyse_final::*;
use crate::functions::dpll::*;
use crate::functions::new_clause::*;
use crate::functions::propagate::*;
use crate::functions::reduce_db::*;
use crate::functions::simplify_db::*;
use crate::functions::stats::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::logger::*;
use crate::models::solverstate::*;
use crate::models::varorder::*;
use std::cmp::max;

/*_________________________________________________________________________________________________
|
|  search : (nof_conflicts : int) (nof_learnts : int) (parms : const SearchParams&)  .  [lbool]
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

pub fn search(
    nof_conflicts: i32,
    nof_learnts: i32,
    parms: SearchParams,
    solver_state: &mut SolverState,
) -> Lbool {
    reportf("search".to_string());

    if !solver_state.ok {
        return Lbool::False;
    }

    solver_state.solver_stats.starts += 1.0;
    let mut conflict_c: i32 = 0;
    solver_state.var_decay = 1.0 / parms.var_decay;
    solver_state.cla_decay = 1.0 / parms.clause_decay;
    solver_state.model.clear();

    loop {
        match propagate(solver_state) {
            Some(_c) => {
                solver_state.solver_stats.conflicts += 1.0;
                conflict_c += 1;

                if solver_state.decision_level() == solver_state.root_level {
                    analyse_final(_c, false, solver_state);
                    return L_FALSE;
                }
                let mut learnt_clause: Vec<Lit> = Vec::new();
                let backtrack_level: i32 = analyze(Some(_c), &mut learnt_clause, solver_state);
                cancel_until(max(backtrack_level, solver_state.root_level), solver_state);
                new_clause(&mut learnt_clause, true, solver_state);
                if learnt_clause.len() == 1 {
                    solver_state.level[var(&learnt_clause[0]) as usize] = 0;
                }
                solver_state.var_decay_activity();
                solver_state.cla_decay_activity();
            }
            None => {
                if nof_conflicts >= 0 && conflict_c >= nof_conflicts {
                    solver_state.progress_estimate = progress_estimate();
                    cancel_until(solver_state.root_level, solver_state);
                    return Lbool::Undef0;
                }

                if solver_state.decision_level() == 0 {
                    simplify_db(solver_state);
                    if !solver_state.ok {
                        return L_FALSE;
                    }
                }

                if nof_learnts >= 0
                    && solver_state.learnts.len() as i32 - solver_state.clone().n_assigns() as i32
                        >= nof_learnts
                {
                    reduce_db(solver_state);
                }

                solver_state.solver_stats.decisions += 1.0;
                let next: Lit = solver_state.order.select(parms.random_var_freq);

                if next == Lit::new(VAR_UNDEFINED, false) {
                    if model_found(solver_state) {
                        continue;
                    }
                    solver_state
                        .model
                        .resize(solver_state.clone().n_vars() as usize, Lbool::Undef0);

                    for y in 0..solver_state.clone().n_vars() {
                        solver_state.model[y as usize] = value_by_var(y, solver_state);
                    }
                    cancel_until(solver_state.root_level, solver_state);
                    return L_TRUE;
                }

                assume(next, solver_state);
            }
        }
    }
}

pub fn var_rescale_activity(solver_state: &mut SolverState) {
    reportf("var_rescale_activity".to_string());

    for y in 0..solver_state.clone().n_vars() {
        solver_state.activity[y as usize] *= 1e-100;
    }
    solver_state.var_inc *= 1e-100;
}

pub fn cla_rescale_activity(solver_state: &mut SolverState) {
    reportf("cla_rescale_activity".to_string());

    for y in 0..solver_state.learnts.len() {
        solver_state.learnts[y as usize].activity *= 1e-20;
    }
    solver_state.cla_inc *= 1e-20;
}
