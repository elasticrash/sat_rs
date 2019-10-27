use crate::models::solverstate::*;
use crate::models::clause::*;

/*_________________________________________________________________________________________________
|
|  propagate : [void]  .  [Clause*]
|
|  Description:
|    Propagates all enqueued facts. If a conflict arises, the conflicting clause is returned,
|    otherwise null. NOTE! This method has been optimized for speed rather than readability.
|
|    Post-conditions:
|      * the propagation queue is empty, even if there was a conflict.
|________________________________________________________________________________________________@*/

pub fn propagate(solver_state: &mut SolverState) {
    let confl: Clause;

    while(solver_state.qhead < solver_state.trail.len()){
        
    }
}
