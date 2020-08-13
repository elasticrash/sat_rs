use crate::functions::enqueue::*;
use crate::models::clause::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::logger::*;
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
        reportf(
            "reorder_by_level".to_string(),
            file!(),
            line!(),
            self.verbosity,
        );

        let mut max: i32 = std::i32::MIN;
        let mut max_at: i32 = -1;
        let mut max2: i32 = std::i32::MIN;
        let mut max2_at: i32 = -1;

        for i in 0.._ps.len() {
            let mut lev: i32 = self.level[var(&_ps[i]) as usize];
            if lev == -1 {
                lev = std::i32::MAX;
            } else if value_by_lit(_ps[i], &self) == Lbool::True {
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
        reportf("new_clause".to_string(), file!(), line!(), self.verbosity);

        if !self.ok {
            return;
        };

        let ps: Vec<Lit>;

        assert!(!(_learnt && _theory_clause));

        if !_learnt {
            assert!(_theory_clause || self.decision_level() == 0);

            let qs = basic_clause_simplification(_ps.to_vec(), _copy);

            if qs == None {
                return;
            }

            let mut unqs = qs.unwrap();
            for i in 0..unqs.len() {
                if self.level[var(&unqs[i]) as usize] == 0
                    && value_by_lit(unqs[i], &self) == Lbool::True
                {
                    return;
                }
            }

            {
                let mut _i: usize = 0;
                let mut _j: usize = 0;

                for _y in 0..unqs.len() {
                    if self.level[var(&unqs[_i]) as usize] != 0
                        || value_by_lit(unqs[_i], &self) != Lbool::False
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

        if ps.len() == 0 {
            self.ok = false;
        } else if ps.len() == 1 {
            if _theory_clause {
                self.level_to_backtrack = 0;
                self.cancel_until(0);
            }
            let ps_clone: &mut Vec<Lit> = &mut ps.to_vec();
            if !self.internal_enqueue(&ps_clone[0]) {
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
                    let mut max_i: i32 = 1;
                    let mut max: i32 = self.level[var(&ps[1]) as usize];
                    for y in 2..ps.len() {
                        if self.level[var(&ps[y]) as usize] > max {
                            max = self.level[var(&ps[y]) as usize];
                            max_i = y as i32;
                        }
                    }
                    c.data[1] = ps[max_i as usize];
                    c.data[max_i as usize] = ps[1];

                    assert!(self.enqueue(&c.data[0], Some(c.clone())));
                } else {
                    move_back(c.clone().data[0], c.clone().data[1], self);
                }

                self.cla_bump_activity(&mut c);
                self.learnts.push(c.clone());
                self.solver_stats.learnts_literals += c.clone().size() as f64;
            }

            let watch_position_zero = (!c.clone().data[0]).x as usize;
            let watch_position_one = (!c.clone().data[1]).x as usize;

            self.watches[watch_position_zero].push(c.clone());
            self.watches[watch_position_one].push(c.clone());
        }
    }

    fn remove(&mut self, c: Clause, just_dealloc: bool) {
        reportf("remove".to_string(), file!(), line!(), self.verbosity);

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
        reportf("simplify".to_string(), file!(), line!(), self.verbosity);
        assert!(self.decision_level() == 0);

        let c;
        if t != 0 {
            c = &self.learnts[k as usize];
        } else {
            c = &self.clauses[k as usize];
        }

        for y in 0..c.size() {
            let f = value_by_lit(c.data[y as usize], &self);
            if f == Lbool::True {
                return true;
            }
        }
        return false;
    }

    fn new_var(&mut self) -> i32 {
        reportf("new_var".to_string(), file!(), line!(), self.verbosity);

        let index: i32;
        index = self.assigns.len() as i32;
        self.watches.push(Vec::new());
        self.watches.push(Vec::new());
        self.reason.push(None);
        self.assigns.push(Lbool::Undef0);
        self.level.push(-1);
        self.trail_pos.push(-1);
        self.activity.push(0.0);
        self.order.new_var(self.clone());
        self.analyze_seen.push(Lbool::Undef0);

        return index;
    }
    fn assume(&mut self, p: Lit) -> bool {
        reportf("assume".to_string(), file!(), line!(), self.verbosity);

        self.trail_lim.push(self.trail.len() as i32);
        return self.i_enqueue(p);
    }

    fn cancel_until(&mut self, level: i32) {
        reportf("cancel_until".to_string(), file!(), line!(), self.verbosity);

        if self.decision_level() > level {
            let mut c: i32 = (self.trail.len() as i32 - 1) as i32;

            loop {
                let x = var(&self.trail[c as usize]) as usize;
                self.assigns[x] = Lbool::Undef0;
                self.reason[x] = None;
                self.order.clone().undo(x as i32, self.clone()); //revisit:: should no reason to clone here
                c -= 1;
                if c < self.trail_lim[level as usize] {
                    break;
                }
            }
            self.trail.truncate(self.trail_lim[level as usize] as usize);
            self.trail_lim.truncate(level as usize);
            self.qhead = self.trail.len() as i32;
        }
    }
}

fn basic_clause_simplification(_ps: Vec<Lit>, _copy: bool) -> Option<Vec<Lit>> {
    reportf(
        "basic_clause_simplification".to_string(),
        file!(),
        line!(),
        0,
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
                dict.push(Dict { index: v, l: l });
                qs[ptr as usize] = l;
                ptr += 1;
            }
        }
    }
    qs.truncate(ptr as usize);

    return Some(qs);
}

fn remove_watch(ws: &mut Vec<Clause>, elem: Clause) -> bool {
    reportf("remove_watch".to_string(), file!(), line!(), 0);

    if ws.len() == 0 {
        return false;
    }
    let mut j: usize = 0;
    while ws[j] != elem {
        assert!(j < ws.len() - 1);
        j += 1;
    }
    for _y in j..ws.len() - 1 {
        ws[j] = ws[j + 1].clone();
        j += 1;
    }
    ws.pop();
    return true;
}
