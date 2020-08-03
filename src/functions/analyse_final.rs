use crate::models::clause::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::logger::*;
use crate::models::solverstate::*;

/*_________________________________________________________________________________________________
|
|  analyzeFinal
|
|  Description:
|    Specialized analysis procedure to express the final conflict in terms of assumptions.
|    'root_level' is allowed to point beyond end of trace (useful if called after conflict while
|    making assumptions). If 'skip_first' is TRUE, the first literal of 'confl' is  ignored (needed
|    if conflict arose before search even started).
|________________________________________________________________________________________________@*/

pub fn analyse_final(_confl: Clause, _skip_first: bool, solver_state: &mut SolverState) {
    reportf(
        "analyse final".to_string(),
        file!(),
        line!(),
        solver_state.verbosity,
    );
    solver_state.conflict.clear();
    if solver_state.root_level == 0 {
        return;
    }

    let istart: i32;
    if _skip_first {
        istart = 1
    } else {
        istart = 0
    };
    for _y in istart.._confl.data.len() as i32 {
        let x: usize = var(&_confl.data[_y as usize]) as usize;
        if solver_state.level[x] > 0 {
            solver_state.analyze_seen[x] = Lbool::True;
        }
    }

    let start: i32;
    if solver_state.root_level >= solver_state.trail_lim.len() as i32 {
        start = (solver_state.trail.len() - 1) as i32;
    } else {
        start = solver_state.trail_lim[solver_state.root_level as usize];
    }

    for y in (start..solver_state.trail_lim[0] + 1).rev() {
        let x: usize = var(&solver_state.trail[y as usize]) as usize;

        if solver_state.analyze_seen[x] != Lbool::Undef0 {
            match solver_state.reason[x].clone() {
                Some(clause) => {
                    for j in 1..clause.data.len() {
                        if solver_state.level[var(&clause.data[j as usize]) as usize] > 0 {
                            solver_state.analyze_seen[var(&clause.data[j as usize]) as usize] =
                                Lbool::True;
                        }
                    }
                }
                None => {
                    assert!(solver_state.level[x] > 0);
                    solver_state.conflict.push(!solver_state.trail[y as usize]);
                }
            }

            solver_state.analyze_seen[x] = Lbool::Undef0;
        }
    }
}
