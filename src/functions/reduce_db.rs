use crate::functions::new_clause::*;
use crate::models::clause::*;
use crate::models::lit::*;
use crate::models::solverstate::*;
use std::cmp::Ordering;
/*_________________________________________________________________________________________________
|
|  reduceDB : ()  .  [void]
|
|  Description:
|    Remove half of the learnt clauses, minus the clauses locked by the current assignment. Locked
|    clauses are clauses that are reason to some assignment. Binary clauses are never removed.
|________________________________________________________________________________________________@*/

fn reduce_db(solver_state: &mut SolverState) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;

    let extra_lim: f64 = solver_state.cla_inc / solver_state.learnts.len() as f64;

    solver_state.learnts.sort_by(|x, y| {
        if x.size() > 2 && (y.size() == 2 || x.activity < y.activity) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    for y in 0..solver_state.learnts.len() / 2 {
        if solver_state.learnts[y].data.len() > 2
            && !locked(solver_state.learnts[y].clone(), solver_state)
        {
            remove(solver_state.learnts[y].clone(), false, solver_state);
        } else {
            solver_state.learnts[j as usize] = solver_state.learnts[i as usize].clone();
            j += 1;
        }
    }

    for y in i..solver_state.learnts.len() as i32 {
        if solver_state.learnts[y as usize].data.len() > 2
            && !locked(solver_state.learnts[y as usize].clone(), solver_state)
            && solver_state.learnts[y as usize].activity < extra_lim
        {
            remove(
                solver_state.learnts[y as usize].clone(),
                false,
                solver_state,
            );
        } else {
            solver_state.learnts[j as usize] = solver_state.learnts[i as usize].clone();
            j += 1;
        }
    }
    solver_state.learnts.truncate((i - j) as usize)
}
