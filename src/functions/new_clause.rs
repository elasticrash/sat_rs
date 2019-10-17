use crate::functions::enqueue::*;
use crate::models::clause::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::solverstate::*;
use crate::models::varorder::*;
use std::collections::HashMap;

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

pub fn basic_clause_simplification(_ps: Vec<Lit>, _copy: bool) -> Option<Vec<Lit>> {
    let mut qs: Vec<Lit>;

    if _copy {
        qs = Vec::new();
    } else {
        qs = _ps;
    }

    let mut dict: HashMap<i32, Lit> = HashMap::new();
    let ptr: i32 = 0;

    for i in 0..qs.len() {
        let l: Lit = qs[i];
        let v: i32 = var(&l);
        let other = Some(dict.get(&v).unwrap());
        if other != None {
            if other.unwrap() == &l {
            } else {
                return None;
            }
        } else {
            dict.insert(v, l);
            qs.push(l);
        }
    }
    qs.truncate(ptr as usize);

    return Some(qs);
}

pub fn reorder_by_level(mut _ps: &mut Vec<Lit>, solver_state: &mut SolverState) {
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
    new_clause_pr(_ps, _learnt, false, true, solver_state);
}

fn new_clause_pr(
    _ps: &mut Vec<Lit>,
    _learnt: bool,
    _theory_clause: bool,
    _copy: bool,
    solver_state: &mut SolverState,
) {
    if !solver_state.ok {
        return;
    };

    let mut ps: Vec<Lit> = Vec::new();

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

            for y in 0..unqs.len() {
                if solver_state.level[var(&unqs[_i]) as usize] != 0
                    || value_by_lit(unqs[_i], &solver_state) != Lbool::False
                {
                    _j += 1;
                    unqs[_j] = unqs[_i];
                }
                unqs.truncate(_i - _j);

                _i += 1;
            }
        }
        ps = unqs;
    } else {
        ps = _ps.to_vec();
    }

    if ps.len() == 0 {
        solver_state.ok = false;
    } else if ps.len() == 1 {
        if _theory_clause {
            solver_state.level_to_backtrack = 0;
            cancel_util(0, solver_state);
        }

        let c: Clause = Clause::new(_learnt || _theory_clause, &ps);
        new_clause_callback(c);

        let ps_clone: &mut Vec<Lit> = &mut ps.to_vec();
        if !internal_enqueue(&ps_clone[0]) {
            solver_state.ok = false;
        } else {
            if _theory_clause {
                reorder_by_level(ps_clone, solver_state)
            }

            let c: Clause = Clause::new(_learnt || _theory_clause, &ps);
            let c_clone: &mut Clause = &mut c.clone();

            if !_learnt && !_theory_clause {
                solver_state.clauses.push(c.clone());
                solver_state.solver_stats.clauses_literals += c_clone.size() as f64;
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
                    c_clone.data[1] = ps[max_i as usize];
                    c_clone.data[max_i as usize] = ps[1];
                } else {
                    move_back(c.clone().data[0], c.clone().data[1]);
                }

                solver_state.cla_bump_activity(c.clone());
                solver_state.learnts.push(c.clone());
                solver_state.solver_stats.learnts_literals += c.clone().size() as f64;
            }

            solver_state.watches[index(!c.clone().data[0]) as usize].push(c.clone());
            solver_state.watches[index(!c.clone().data[1]) as usize].push(c.clone());
            new_clause_callback(c);
        }
    }
}

pub fn remove(c: Clause, just_dealloc: bool, solver_state: &mut SolverState) {
    if !just_dealloc {
        solver_state.watches[index(!c.clone().data[0]) as usize].push(c.clone());
        solver_state.watches[index(!c.clone().data[1]) as usize].push(c.clone());
    }

    if c.is_learnt {
        solver_state.solver_stats.learnts_literals -= c.size() as f64;
    } else {
        solver_state.solver_stats.clauses_literals -= c.size() as f64;
    }
}
pub fn simplify(c: Clause, solver_state: &mut SolverState) -> bool {
    for y in 0..c.size() {
        if value_by_lit(c.data[y as usize], &solver_state) == Lbool::True {
            return true;
        }
    }
    return false;
}
pub fn remove_watch(ws: &mut Vec<Clause>, elem: Clause) -> bool {
    if ws.len() == 0 {
        return false;
    }
    for y in 0..ws.len() - 1 {
        ws[y] = ws[y + 1].clone();
    }
    ws.pop();
    return true;
}
pub fn new_var(solver_state: &mut SolverState) -> i32 {
    let index: i32;
    index = solver_state.assigns.len() as i32;
    solver_state.watches.push(Vec::new());
    solver_state.watches.push(Vec::new());
    solver_state.reason.push(None);
    solver_state.assigns.push(Lbool::Undef0);
    solver_state.level.push(-1);
    solver_state.trail_pos.push(-1);
    solver_state.activity.push(0.0);
    solver_state.analyze_seen.push(Lbool::Undef0);

    return index;
}
pub fn assume(p: Lit, solver_state: &mut SolverState) -> bool {
    solver_state.trail_lim.push(solver_state.trail.len() as i32);
    return solver_state.i_enqueue(p);
}

pub fn cancel_util(level: i32, solver_state: &mut SolverState) {
    if solver_state.decision_level() > level {
        for y in (solver_state.trail_lim[level as usize])..(solver_state.trail.len() as i32) {
            let x = var(&solver_state.trail[y as usize]) as usize;
            solver_state.assigns[x] = Lbool::Undef0;
            solver_state.reason[x] = None;
            solver_state.order.clone().undo(x as i32); //revisit:: should no reason to clone here
        }
    }
}
pub fn new_clause_callback(c: Clause) {}
