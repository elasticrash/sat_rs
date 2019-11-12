use crate::models::clause::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::solverstate::*;
use crate::models::logger::*;

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
    reportf("analyse final".to_string());
    solver_state.conflict.clear();
    if solver_state.root_level == 0 {
        return;
    }

    let mut seen: Vec<Lbool> = solver_state.analyze_seen.clone();
    let mut istart: i32;
    if _skip_first {
        istart = 1
    } else {
        istart = 0
    };
    for _y in istart.._confl.data.len() as i32 {
        let x: usize = var(&_confl.data[istart as usize]) as usize;
        if solver_state.level[x] > 0 {
            seen[x] = Lbool::True;
        }
        istart += 1;
    }

    let mut end: i32 = solver_state.trail_lim[solver_state.root_level as usize];
    if solver_state.root_level >= solver_state.trail_lim.len() as i32 {
        end = (solver_state.trail.len() - 1) as i32;
    }

    for y in (end..solver_state.trail_lim[0] + 1).rev() {
        let x: usize = var(&solver_state.trail[y as usize]) as usize;

        if seen[x] != Lbool::True {
            match solver_state.reason[x].clone() {
                Some(clause) => {
                    for j in 1..clause.data.len() {
                        if solver_state.level[var(&clause.data[j as usize]) as usize] > 0 {
                            seen[var(&clause.data[j as usize]) as usize] = Lbool::True;
                        }
                    }
                }
                None => {
                    solver_state.conflict.push(!solver_state.trail[y as usize]);
                }
            }

            seen[x] = Lbool::Undef0;
        }
    }
}
