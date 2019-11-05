use crate::models::solverstate::*;

/*_________________________________________________________________________________________________
|
|  solve : (assumps : const vec<Lit>&)  .  [bool]
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

pub fn solve(solver_state: &mut SolverState) {}
