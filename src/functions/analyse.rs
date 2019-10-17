use crate::models::clause::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::solverstate::*;

/*_________________________________________________________________________________________________
|
|  analyze : (confl : Clause*) (out_learnt : vec<Lit>&) (out_btlevel : int&)  .  [void]
|
|  Description:
|    Analyze conflict and produce a reason clause.
|
|    Pre-conditions:
|      * 'out_learnt' is assumed to be cleared.
|      * Current decision level must be greater than root level.
|
|    Post-conditions:
|      * 'out_learnt[0]' is the asserting literal at level 'out_btlevel'.
|
|  Effect:
|    Will undo part of the trail, upto but not beyond the assumption of the current decision level.
|________________________________________________________________________________________________@*/

pub fn analyze(confl: Clause, out_learnt: &mut Vec<Lit>, solver_state: &mut SolverState) -> i32 {
    let out_btlevel: i32 = 0;
    let seen: Vec<Lbool> = solver_state.analyze_seen.clone();
    let mut path_c: i32 = 0;
    let p: Lit = Lit::new(VAR_UNDEFINED, true);

    out_learnt.push(Lit::new(VAR_UNDEFINED, true)); // (leave room for the asserting literal)
    let index: i32 = (solver_state.trail.len() - 1) as i32;

    while {
        path_c = do_analyze(confl.clone(), solver_state);
        path_c > 0
    } {}

    return out_btlevel;
}

fn do_analyze(confl: Clause, solver_state: &mut SolverState) -> i32 {
    let c: Clause = confl;

    if c.is_learnt {
        solver_state.cla_bump_activity(c);
    }

    return 0;
}
