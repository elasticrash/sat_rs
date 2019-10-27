use crate::models::clause::*;
use crate::models::lit::*;
use crate::models::solverstate::*;

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

    while (solver_state.qhead < solver_state.trail.len() as i32) {
        solver_state.solver_stats.propagations += 1.0;
        solver_state.simp_db_props -= 1.0;
        let p: Lit = solver_state.trail[solver_state.qhead as usize];
        solver_state.qhead += 1;
        let ws: Vec<Clause> = solver_state.watches[index(p.clone()) as usize].clone();
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut end = i + ws.len() as i32;
        while (i != end) {}
    }
}
