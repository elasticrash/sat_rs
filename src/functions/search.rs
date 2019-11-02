use crate::functions::propagate::*;
use crate::models::clause::*;
use crate::models::lbool::*;
use crate::models::solverstate::*;
use crate::models::statsparams::*;

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

pub fn search(nof_conflicts: i32, parms: SearchParams, solver_state: &mut SolverState) -> Lbool {
    if !solver_state.ok {
        return Lbool::False;
    }

    solver_state.solver_stats.starts += 1.0;
    let mut conflict_c: i32 = 0;
    solver_state.var_decay = 1.0 / parms.var_decay;
    solver_state.cla_decay = 1.0 / parms.clause_decay;
    solver_state.model.clear();

    while true {
        match propagate(solver_state) {
            Some(_c) => {}
            None => {
                solver_state.solver_stats.conflicts += 1.0;
                conflict_c += 1;
            }
        }
    }

    return Lbool::True;
}

pub fn var_rescale_activity() {}

pub fn cla_rescale_activity() {}
