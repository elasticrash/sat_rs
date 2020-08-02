use crate::models::clause::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::logger::*;
use crate::models::solverstate::*;

/*_________________________________________________________________________________________________
|
|  enqueue
|
|  Description:
|    Puts a new fact on the propagation queue as well as immediately updating the variable's value.
|    Should a conflict arise, FALSE is returned.
|
|  Input:
|    p    - The fact to enqueue
|    from - [Optional] Fact propagated from this (currently) unit clause. Stored in 'reason[]'.
|           Default value is null (no reason).
|
|  Output:
|    TRUE if fact was enqueued without conflict, FALSE otherwise.
|________________________________________________________________________________________________@*/

pub fn enqueue(_fact: &Lit, _from: Option<Clause>, solver_state: &mut SolverState) -> bool {
    reportf(
        "enqueue".to_string(),
        file!(),
        line!(),
        solver_state.verbosity,
    );

    if !is_undefined(value_by_lit(*_fact, solver_state)) {
        return value_by_lit(*_fact, solver_state) != L_FALSE;
    } else {
        let x: usize = var(&_fact) as usize;
        solver_state.assigns[x] = to_bool(!sign(_fact));
        solver_state.level[x] = solver_state.decision_level();
        solver_state.trail_pos[x] = solver_state.trail.len() as i32;
        solver_state.trail.push(*_fact);
        solver_state.reason[x] = _from;

        return true;
    }
}

pub fn internal_enqueue(_fact: &Lit, solver_state: &mut SolverState) -> bool {
    reportf(
        "internal_enqueue".to_string(),
        file!(),
        line!(),
        solver_state.verbosity,
    );

    return enqueue(&_fact, None, solver_state);
}
