use crate::models::solverstate::*;

pub fn model_found(solver_state: &mut SolverState) -> bool {
    solver_state.level_to_backtrack = i32::max_value();
    let res: bool = false;
    if !solver_state.ok {
        return false;
    }
    return res;
}
