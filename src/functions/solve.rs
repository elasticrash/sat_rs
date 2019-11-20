use crate::functions::analyse_final::*;
use crate::functions::new_clause::*;
use crate::functions::propagate::*;
use crate::functions::search::*;
use crate::functions::simplify_db::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::logger::*;
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

pub fn solve(assumptions: Vec<Lit>, solver_state: &mut SolverState) -> bool {
    reportf("solve".to_string(), solver_state.verbosity);

    simplify_db(solver_state);
    if !solver_state.ok {
        return false;
    };

    let parms: SearchParams = solver_state.default_parms;
    let mut nof_conflicts: f64 = 100.0;
    let mut nof_learnts: f64 = solver_state.clone().n_clauses() as f64 / 3.0;
    let mut status: Lbool = Lbool::Undef0;

    solver_state.root_level = assumptions.len() as i32;
    for y in 0..assumptions.len() {
        let p: Lit = assumptions[y];
        assert!(var(&p) < solver_state.n_vars());

        if !assume(p, solver_state) {
            match &solver_state.reason[var(&p) as usize] {
                Some(r) => {
                    analyse_final(r.clone(), true, solver_state);
                    solver_state.conflict.push(!p);
                }
                None => {
                    solver_state.conflict.clear();
                    solver_state.conflict.push(!p);
                }
            }

            cancel_until(0, solver_state);
            return false;
        }
        {
            match propagate(solver_state) {
                Some(confl) => {
                    analyse_final(confl.clone(), false, solver_state);
                    assert!(solver_state.conflict.len() > 0);
                    cancel_until(0, solver_state);
                    return false;
                }
                None => {}
            }
        }
    }
    assert!(solver_state.root_level == solver_state.decision_level());

    if solver_state.verbosity >= 1 {
        reportf(
            "==================================[MINISAT]======================================="
                .to_string(),
            2,
        );
        reportf(
            "| Conflicts |       ORIGINAL        |              LEARNT              | Progress |"
                .to_string(),
            2,
        );
        reportf(
            "|           | Clauses      Literals |   Limit Clauses Literals  Lit/Cl |          |"
                .to_string(),
            2,
        );
        reportf(
            "=================================================================================="
                .to_string(),
            2,
        );
    }

    while is_undefined(status) {
        if solver_state.verbosity >= 1 {
            reportf(
                format!(
                    "|      {0}    |     {1}        {2}    |   {3}      {4}       {5}       {6}   |   {7} %%   |",
                    solver_state.solver_stats.conflicts,
                    solver_state.clone().n_clauses(),
                    solver_state.solver_stats.clauses_literals,
                    nof_learnts.floor(),
                    solver_state.clone().n_learnts(),
                    solver_state.solver_stats.learnts_literals,
                    (solver_state.solver_stats.learnts_literals
                        / solver_state.clone().n_learnts() as f64).floor(),
                    solver_state.progress_estimate * 100.0
                ),
                2,
            );

            status = search(
                nof_conflicts as i32,
                nof_learnts as i32,
                parms,
                solver_state,
            );

            nof_conflicts *= 1.5;
            nof_learnts *= 1.1;
        }
    }

    if solver_state.verbosity >= 1 {
        reportf(
            "=================================================================================="
                .to_string(),
            2,
        );
    }
    cancel_until(0, solver_state);
    return true;
}

pub fn solve_no_assumptions(solver_state: &mut SolverState) {
    reportf("solve_no_assumptions".to_string(), solver_state.verbosity);
    let assumptions: Vec<Lit> = Vec::new();
    solve(assumptions, solver_state);
}
