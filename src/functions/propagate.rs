use crate::functions::enqueue::*;
use crate::models::clause::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::logger::*;
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

pub fn propagate(solver_state: &mut SolverState) -> Option<Clause> {
    reportf("propagate".to_string());

    let mut confl: Option<Clause> = None;

    while solver_state.qhead < solver_state.trail.len() as i32 {
        solver_state.solver_stats.propagations += 1.0;
        solver_state.simp_db_props -= 1.0;
        let p: Lit = solver_state.trail[solver_state.qhead as usize];
        solver_state.qhead += 1;
        let mut ws: Vec<Clause> = solver_state.watches[index(p.clone()) as usize].clone();
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let end = i + ws.len() as i32;
        while i != end {
            let mut c: Clause = ws[i as usize].clone();

            i += 1;
            let false_lit: Lit = !p;
            if c.data[0] == false_lit {
                c.data[0] = c.data[1];
                c.data[1] = false_lit;
            }

            let first: Lit = c.data[0].clone();
            let val: Lbool = value_by_lit(first, solver_state);
            let mut foundwatch: bool = false;
            if val == L_TRUE {
                ws[j as usize] = c;
                j += 1;
            } else {
                for k in 2..c.data.len() {
                    if value_by_lit(c.data[k], solver_state) != L_FALSE {
                        c.data[1] = c.data[k];
                        c.data[k] = false_lit;

                        solver_state.watches[index(!c.data[1]) as usize].push(c.clone());
                        foundwatch = true;
                        break;
                    }
                }

                if !foundwatch {
                    if enqueue(&first, Some(c.clone()), solver_state) {
                        if solver_state.decision_level() == 0 {
                            solver_state.ok = false;
                            confl = Some(c.clone());
                            solver_state.qhead = solver_state.trail.len() as i32;

                            while i < end {
                                ws[j as usize] = ws[i as usize].clone();
                                j += 1;
                                i += 1;
                            }
                        }
                    }
                }
            }
        }
        ws.truncate((i - j) as usize);
    }
    return confl;
}
