use crate::functions::enqueue::*;
use crate::models::clause::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::solverstate::*;
use crate::models::varorder::*;

/*_________________________________________________________________________________________________
|
|  newClause
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

pub trait NewClause {
    fn reorder_by_level(&mut self, _ps: &mut Vec<Lit>);
    fn new_clause(&mut self, _ps: &mut Vec<Lit>, _learnt: bool);
    fn new_clause_pr(
        &mut self,
        _ps: &mut Vec<Lit>,
        _learnt: bool,
        _theory_clause: bool,
        _copy: bool,
    );
    fn remove(&mut self, c: Clause, just_dealloc: bool);
    fn simplify(&mut self, k: i32, t: i32) -> bool;
    fn new_var(&mut self) -> i32;
    fn assume(&mut self, p: Lit) -> bool;
    fn cancel_until(&mut self, level: i32);
}

#[derive(Clone)]
struct Dict {
    index: i32,
    l: Lit,
}

impl NewClause for SolverState {
    fn reorder_by_level(&mut self, mut _ps: &mut Vec<Lit>) {
        trace!(
            "{}|{}|{}|{:?}",
            "reorder_by_level".to_string(),
            file!(),
            line!(),
            _ps,
        );

        let mut max: i32 = std::i32::MIN;
        let mut max_at: i32 = -1;
        let mut max2: i32 = std::i32::MIN;
        let mut max2_at: i32 = -1;

        for (i, lt) in _ps.iter().enumerate() {
            let mut lev: i32 = self.level[var(lt) as usize];
            if lev == -1 || self.value_by_lit(*lt) == Lbool::True {
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
            _ps.swap(1_usize, max2_at as usize);
        } else if max_at == 1 {
            _ps.swap(0_usize, max2_at as usize);
        } else if max2_at == 0 {
            _ps.swap(1_usize, max_at as usize);
        } else if max2_at == 1 {
            _ps.swap(0_usize, max_at as usize);
        } else {
            _ps.swap(0_usize, max_at as usize);
            _ps.swap(1_usize, max2_at as usize);
        }
    }

    fn new_clause(&mut self, _ps: &mut Vec<Lit>, _learnt: bool) {
        self.new_clause_pr(_ps, _learnt, false, true);
    }

    fn new_clause_pr(
        &mut self,
        _ps: &mut Vec<Lit>,
        _learnt: bool,
        _theory_clause: bool,
        _copy: bool,
    ) {
        trace!(
            "{}|{}|{}|{:?}",
            "new_clause".to_string(),
            file!(),
            line!(),
            _ps
        );

        if !self.ok {
            return;
        };

        let ps: Vec<Lit>;

        assert!(!(_learnt && _theory_clause));

        if !_learnt {
            assert!(_theory_clause || self.decision_level() == 0);

            let qs = basic_clause_simplification(_ps.to_vec(), _copy);

            let mut unqs = match qs {
                Some(v) => v,
                None => return,
            };

            for unq in &unqs {
                if self.level[var(unq) as usize] == 0 && self.value_by_lit(*unq) == Lbool::True {
                    return;
                }
            }

            let mut _i: usize = 0;
            let mut _j: usize = 0;

            for _y in 0..=unqs.len() {
                _i = _y;
                if _i == unqs.len() {
                    break;
                }
                if self.level[var(&unqs[_y]) as usize] != 0
                    || self.value_by_lit(unqs[_y]) != Lbool::False
                {
                    unqs[_j] = unqs[_y];
                    _j += 1;
                }
            }
            unqs.truncate(unqs.len() - (_i - _j));

            ps = unqs;
        } else {
            ps = _ps.to_vec();
        }

        if ps.is_empty() {
            self.ok = false;
        } else if ps.len() == 1 {
            if _theory_clause {
                self.level_to_backtrack = 0;
                self.cancel_until(0);
            }
            if !self.internal_enqueue(&ps[0]) {
                self.ok = false;
            }
        } else {
            if _theory_clause {
                self.reorder_by_level(&mut ps.to_vec())
            }

            let mut c: Clause = Clause::new(_learnt || _theory_clause, &ps);

            if !_learnt && !_theory_clause {
                self.clauses.push(c.clone());
                self.solver_stats.clauses_literals += c.size() as f64;
            } else {
                if _learnt {
                    let mut max_i: usize = 1;
                    let mut max: i32 = self.level[var(&ps[1]) as usize];
                    for (y, lt) in ps.iter().enumerate().skip(2) {
                        if self.level[var(lt) as usize] > max {
                            max = self.level[var(lt) as usize];
                            max_i = y;
                        }
                    }
                    c.data[1] = ps[max_i];
                    c.data[max_i] = ps[1];

                    assert!(self.enqueue(&c.data[0], Some(c.clone())));
                } else {
                    move_back(c.clone().data[0], c.clone().data[1], self);
                }

                self.cla_bump_activity(&mut c);
                self.learnts.push(c.clone());
                self.solver_stats.learnts_literals += c.clone().size() as f64;
            }

            let watch_position_zero = (!c.data[0]).x as usize;
            let watch_position_one = (!c.data[1]).x as usize;

            self.watches[watch_position_zero].push(c.clone());
            self.watches[watch_position_one].push(c.clone());
        }
    }

    fn remove(&mut self, c: Clause, just_dealloc: bool) {
        trace!("{}|{}|{}|{:?}", "remove".to_string(), file!(), line!(), c);

        if !just_dealloc {
            remove_watch(
                &mut self.watches[(!c.clone().data[0]).x as usize],
                c.clone(),
            );
            remove_watch(
                &mut self.watches[(!c.clone().data[1]).x as usize],
                c.clone(),
            );
        }

        if c.is_learnt {
            self.solver_stats.learnts_literals -= c.size() as f64;
        } else {
            self.solver_stats.clauses_literals -= c.size() as f64;
        }
    }
    fn simplify(&mut self, k: i32, t: i32) -> bool {
        trace!(
            "{}|{}|{}|{}|{}",
            "simplify".to_string(),
            file!(),
            line!(),
            k,
            t
        );
        assert!(self.decision_level() == 0);

        let c = if t != 0 {
            &self.learnts[k as usize]
        } else {
            &self.clauses[k as usize]
        };

        for y in 0..c.size() {
            let f = self.clone().value_by_lit(c.data[y as usize]);
            if f == Lbool::True {
                return true;
            }
        }
        false
    }

    fn new_var(&mut self) -> i32 {
        trace!("{}|{}|{}", "new_var".to_string(), file!(), line!(),);

        let index: i32 = self.assigns.col.len() as i32;
        self.watches.push(Vec::new());
        self.watches.push(Vec::new());
        self.reason.push(None);
        self.add_assigns(Lbool::Undef0);
        self.level.push(-1);
        self.trail_pos.push(-1);
        self.add_activity(0.0);
        self.order.new_var();
        self.analyze_seen.push(Lbool::Undef0);

        index
    }
    fn assume(&mut self, p: Lit) -> bool {
        trace!("{}|{}|{}|{:?}", "assume".to_string(), file!(), line!(), p);

        self.trail_lim.push(self.trail.len() as i32);
        self.i_enqueue(p)
    }

    fn cancel_until(&mut self, level: i32) {
        trace!(
            "{}|{}|{}|{}",
            "cancel_until".to_string(),
            file!(),
            line!(),
            level
        );

        if self.decision_level() > level {
            for y in (self.trail_lim[level as usize]..=(self.trail.len() as i32 - 1)).rev() {
                let x = var(&self.trail[y as usize]) as usize;
                self.update_assigns(Lbool::Undef0, x);
                self.reason[x] = None;
                self.order.undo(x as i32);
            }

            self.trail.truncate(self.trail_lim[level as usize] as usize);
            self.trail_lim.truncate(level as usize);
            self.qhead = self.trail.len() as i32;
        }
    }
}

fn basic_clause_simplification(_ps: Vec<Lit>, _copy: bool) -> Option<Vec<Lit>> {
    trace!(
        "{}|{}|{}|{:?}",
        "basic_clause_simplification".to_string(),
        file!(),
        line!(),
        _ps,
    );

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

        match dict.iter().find(|&x| x.index == v) {
            Some(d) => {
                if d.l == l {
                } else {
                    return None;
                }
            }
            None => {
                dict.push(Dict { index: v, l });
                qs[ptr as usize] = l;
                ptr += 1;
            }
        }
    }
    qs.truncate(ptr as usize);

    Some(qs)
}

fn remove_watch(ws: &mut Vec<Clause>, elem: Clause) -> bool {
    trace!(
        "{}|{}|{}|{:?}",
        "remove_watch".to_string(),
        file!(),
        line!(),
        ws
    );

    if ws.is_empty() {
        return false;
    }
    let mut j: usize = 0;
    while ws[j] != elem {
        assert!(j < ws.len() - 1);
        j += 1;
    }
    for y in j..ws.len() - 1 {
        ws[y] = ws[y + 1].clone();
    }
    ws.pop();
    true
}
