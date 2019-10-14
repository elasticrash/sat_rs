use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::solverstate::*;
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

pub fn new_clause(_ps: Vec<Lit>, _learnt: bool) {}

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

pub fn reorder_by_level(mut _ps: Vec<Lit>, solver_state: &mut SolverState) {
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
        } else if lev > max {
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

fn new_clause_pr(_ps: Vec<Lit>, _learnt: bool, _theory_clause: bool, _copy: bool) {}

pub fn remove() {}
pub fn simplify() {}
pub fn remove_watch() {}
pub fn new_var() {}
