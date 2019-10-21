use crate::models::clause::*;
use crate::models::solverstate::*;
use crate::models::lbool::*;

/*_________________________________________________________________________________________________
|
|  analyzeFinal : (confl : Clause*) (skip_first : bool)  .  [void]
|
|  Description:
|    Specialized analysis procedure to express the final conflict in terms of assumptions.
|    'root_level' is allowed to point beyond end of trace (useful if called after conflict while
|    making assumptions). If 'skip_first' is TRUE, the first literal of 'confl' is  ignored (needed
|    if conflict arose before search even started).
|________________________________________________________________________________________________@*/

pub fn analyse_final(_confl: Clause, _skip_first: bool, solver_state: &mut SolverState) {
    solver_state.conflict.clear();
    if solver_state.root_level == 0 {return;}

    let seen:Vec<Lbool> = Vec ::new();
}
