use crate::functions::enqueue::*;
use crate::models::clause::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::logger::*;
use crate::models::solverstate::*;
use crate::models::varorder::*;

/*_________________________________________________________________________________________________
|
|  newClause : (ps : const vec<Lit>&) (learnt : bool)  .  [void]
|
|  Description:
|    Allocate and add a new clause to the SAT solvers clause database. If a conflict is detected,
|    the 'ok' flag is cleared and the solver is in an unusable state (must be disposed).
|
|  Input:
|    ps     - The new clause as a vector of literals.
|    learnt - Is the clause a learnt clause? For learnt clauses, 'ps[0]' is assumed to be the
|             asserting literal. An appropriate 'enqueue()' operation will be performed on this
|             literal. One of the watches will always be on this literal, the other will be set to
|             the literal with the highest decision level.
|
|  Effect:
|    Activity heuristics are updated.
|________________________________________________________________________________________________@*/

#[derive(Clone)]
struct Dict {
    index: i32,
    l: Lit,
}

pub fn basic_clause_simplification(_ps: Vec<Lit>, _copy: bool) -> Option<Vec<Lit>> {
    reportf("basic_clause_simplification".to_string(), 0);

    let mut qs: Vec<Lit>;

    if _copy {
        qs = _ps.to_vec();
    } else {
        qs = _ps;
    }

    let mut dict: Vec<Dict> = Vec::new();
    let mut ptr: i32 = 0;

    for i in 0..qs.len() {
        let l: Lit = qs[i];
        let v: i32 = var(&l);
        let mut has_value: bool = false;

        match dict.iter().find(|&x| x.index == v) {
            Some(d) => {
                if d.l == l {
                } else {
                    return None;
                }
            }
            None => {
                dict.push(Dict { index: v, l: l });
                qs[ptr as usize] = l;
                ptr += 1;
            }
        }
    }
    qs.truncate(ptr as usize);

    return Some(qs);
}

pub fn reorder_by_level(mut _ps: &mut Vec<Lit>, solver_state: &mut SolverState) {
    reportf("reorder_by_level".to_string(), solver_state.verbosity);

    let mut max: i32 = std::i32::MIN;
    let mut max_at: i32 = -1;
    let mut max2: i32 = std::i32::MIN;
    let mut max2_at: i32 = -1;

    for i in 0.._ps.len() {
        let mut lev: i32 = solver_state.level[var(&_ps[i]) as usize];
        if lev == -1 {
            lev = std::i32::MAX;
        } else if value_by_lit(_ps[i], &solver_state) == Lbool::True {
            lev = std::i32::MAX;
        }

        if lev >= max {
            max2_at = max_at;
            max2 = max;
            max = lev;
            max_at = i as i32;
        } else if lev > max2 {
            max2 = lev;
            max2_at = i as i32;
        }
    }

    if max_at == 0 {
        swap(1, max2_at, &mut _ps);
    } else if max_at == 1 {
        swap(0, max2_at, &mut _ps);
    } else if max2_at == 0 {
        swap(1, max_at, &mut _ps);
    } else if max2_at == 1 {
        swap(0, max_at, &mut _ps);
    } else {
        swap(0, max_at, &mut _ps);
        swap(1, max2_at, &mut _ps);
    }
}

pub fn new_clause(_ps: &mut Vec<Lit>, _learnt: bool, solver_state: &mut SolverState) {
    reportf("new_clause".to_string(), solver_state.verbosity);
    new_clause_pr(_ps, _learnt, false, true, solver_state);
}

fn new_clause_pr(
    _ps: &mut Vec<Lit>,
    _learnt: bool,
    _theory_clause: bool,
    _copy: bool,
    solver_state: &mut SolverState,
) {
    reportf("new_clause_pr".to_string(), solver_state.verbosity);

    if !solver_state.ok {
        return;
    };

    let ps: Vec<Lit>;

    if !_learnt {
        let qs = basic_clause_simplification(_ps.to_vec(), _copy);

        if qs == None {
            return;
        }

        let mut unqs = qs.unwrap();
        for i in 0..unqs.len() {
            if solver_state.level[var(&unqs[i]) as usize] == 0
                && value_by_lit(unqs[i], &solver_state) == Lbool::True
            {
                return;
            }
        }

        {
            let mut _i: usize = 0;
            let mut _j: usize = 0;

            for _y in 0..unqs.len() {
                if solver_state.level[var(&unqs[_i]) as usize] != 0
                    || value_by_lit(unqs[_i], &solver_state) != Lbool::False
                {
                    unqs[_j] = unqs[_i];
                    _j += 1;
                }
                _i += 1;
            }
            unqs.truncate(unqs.len() - (_i - _j) as usize);
        }
        ps = unqs;
    } else {
        ps = _ps.to_vec();
    }

    reportf(
        "check ps length ".to_string() + &ps.len().to_string(),
        solver_state.verbosity,
    );

    if ps.len() == 0 {
        solver_state.ok = false;
    } else if ps.len() == 1 {
        if _theory_clause {
            solver_state.level_to_backtrack = 0;
            cancel_until(0, solver_state);
        }

        let c: Clause = Clause::new(_learnt || _theory_clause, &ps);
        new_clause_callback(c);

        let ps_clone: &mut Vec<Lit> = &mut ps.to_vec();
        if !internal_enqueue(&ps_clone[0], solver_state) {
            solver_state.ok = false;
        }
    } else {
        if _theory_clause {
            reorder_by_level(&mut ps.to_vec(), solver_state)
        }

        let mut c: Clause = Clause::new(_learnt || _theory_clause, &ps);

        if !_learnt && !_theory_clause {
            solver_state.clauses.push(c.clone());
            solver_state.solver_stats.clauses_literals += c.size() as f64;
        } else {
            if _learnt {
                let mut max_i: i32 = 1;
                let mut max: i32 = solver_state.level[var(&ps[1]) as usize];
                for y in 2..ps.len() {
                    if solver_state.level[var(&ps[y]) as usize] > max {
                        max = solver_state.level[var(&ps[y]) as usize];
                        max_i = y as i32;
                    }
                }
                c.data[1] = ps[max_i as usize];
                c.data[max_i as usize] = ps[1];

                enqueue(&c.data[0], Some(c.clone()), solver_state);
            } else {
                move_back(c.clone().data[0], c.clone().data[1], solver_state);
            }

            solver_state.cla_bump_activity(&mut c);
            solver_state.learnts.push(c.clone());
            solver_state.solver_stats.learnts_literals += c.clone().size() as f64;
        }

        let watch_position_zero = index(!c.clone().data[0]) as usize;
        let watch_position_one = index(!c.clone().data[1]) as usize;

        solver_state.watches[watch_position_zero].push(c.clone());
        solver_state.watches[watch_position_one].push(c.clone());
    }
}

pub fn remove(c: Clause, just_dealloc: bool, solver_state: &mut SolverState) {
    reportf("remove".to_string(), solver_state.verbosity);

    if !just_dealloc {
        remove_watch(
            &mut solver_state.watches[index(!c.clone().data[0]) as usize],
            c.clone(),
        );
        remove_watch(
            &mut solver_state.watches[index(!c.clone().data[1]) as usize],
            c.clone(),
        );
    }

    if c.is_learnt {
        solver_state.solver_stats.learnts_literals -= c.size() as f64;
    } else {
        solver_state.solver_stats.clauses_literals -= c.size() as f64;
    }
}
pub fn simplify(c: Clause, solver_state: &mut SolverState) -> bool {
    reportf("simplify".to_string(), solver_state.verbosity);

    for y in 0..c.size() {
        if value_by_lit(c.data[y as usize], &solver_state) == Lbool::True {
            return true;
        }
    }
    return false;
}
pub fn remove_watch(ws: &mut Vec<Clause>, elem: Clause) -> bool {
    reportf("remove_watch".to_string(), 0);

    if ws.len() == 0 {
        return false;
    }
    let mut j: usize = 0;
    while ws[j] != elem {
        j += 1;
    }
    for _y in j..ws.len() - 1 {
        ws[j] = ws[j + 1].clone();
        j += 1;
    }
    ws.pop();
    return true;
}
pub fn new_var(solver_state: &mut SolverState) -> i32 {
    reportf("new_var".to_string(), solver_state.verbosity);

    let index: i32;
    index = solver_state.assigns.len() as i32;
    solver_state.watches.push(Vec::new());
    solver_state.watches.push(Vec::new());
    solver_state.reason.push(None);
    solver_state.assigns.push(Lbool::Undef0);
    solver_state.level.push(-1);
    solver_state.trail_pos.push(-1);
    solver_state.activity.push(0.0);
    solver_state.order.new_var(solver_state.clone());
    solver_state.analyze_seen.push(Lbool::Undef0);

    return index;
}
pub fn assume(p: Lit, solver_state: &mut SolverState) -> bool {
    reportf("assume".to_string(), solver_state.verbosity);

    solver_state.trail_lim.push(solver_state.trail.len() as i32);
    return solver_state.i_enqueue(p);
}

pub fn cancel_until(level: i32, solver_state: &mut SolverState) {
    reportf("cancel_until".to_string(), solver_state.verbosity);

    if solver_state.decision_level() > level {
        let mut c: i32 = (solver_state.trail.len() as i32 - 1) as i32;

        loop {
            let x = var(&solver_state.trail[c as usize]) as usize;
            solver_state.assigns[x] = Lbool::Undef0;
            solver_state.reason[x] = None;
            solver_state
                .order
                .clone()
                .undo(x as i32, solver_state.clone()); //revisit:: should no reason to clone here
            c -= 1;
            if c < solver_state.trail_lim[level as usize] {
                break;
            }
        }
        solver_state
            .trail
            .truncate(solver_state.trail_lim[level as usize] as usize);
        solver_state.trail_lim.truncate(level as usize);
        solver_state.qhead = solver_state.trail.len() as i32;
    }
}
pub fn new_clause_callback(_c: Clause) {}
