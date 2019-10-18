use crate::models::clause::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::solverstate::*;
use std::cmp::max;

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

pub fn analyze(
    mut confl: Option<Clause>,
    mut out_learnt: &mut Vec<Lit>,
    solver_state: &mut SolverState,
) -> i32 {
    let mut out_btlevel: i32 = 0;
    let mut seen: Vec<Lbool> = solver_state.analyze_seen.clone();
    let mut path_c: i32 = 0;
    let mut p: Lit = Lit::new(VAR_UNDEFINED, true);

    out_learnt.push(Lit::new(VAR_UNDEFINED, true)); // (leave room for the asserting literal)
    let index: i32 = (solver_state.trail.len() - 1) as i32;

    while {
        {
            let c: Clause = confl.clone().unwrap();

            if c.is_learnt {
                solver_state.cla_bump_activity(c.clone());
            }

            let mut start: usize = 1;
            if p.x == VAR_UNDEFINED {
                start = 0;
            }
            for j in start..c.clone().data.len() {
                let q: Lit = c.data[j];
                if seen[var(&q) as usize] == Lbool::Undef0
                    && solver_state.level[var(&q) as usize] > 0
                {
                    solver_state.var_bump_activity(q);
                    seen[var(&q) as usize] = Lbool::True;
                    if solver_state.level[var(&q) as usize] == solver_state.decision_level() {
                        path_c += 1;
                    } else {
                        out_learnt.push(q);
                        out_btlevel = max(out_btlevel, solver_state.level[var(&q) as usize])
                    }
                }
            }
            let mut n_index: usize = 0;
            while {
                n_index = (index - 1) as usize;
                seen[var(&solver_state.trail[n_index]) as usize] == Lbool::Undef0
            } {
                n_index += 1;
                p = solver_state.trail[n_index];
                confl = solver_state.reason[var(&p) as usize].clone();
                seen[var(&p) as usize] = Lbool::Undef0;
                path_c -= 1;
            }
        }
        path_c > 0
    } {}

    return out_btlevel;
}
