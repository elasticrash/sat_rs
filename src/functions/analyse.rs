use crate::models::clause::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::solverstate::*;
use std::cmp::max;

/*_________________________________________________________________________________________________
|
|  analyze
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
pub trait Analyze {
    fn analyze(&mut self, confl: Option<Clause>, out_learnt: &mut Vec<Lit>) -> i32;
    fn analyze_removeable(&mut self, _p: Lit, min_level: u32) -> bool;
}

impl Analyze for SolverState {
    fn analyze(&mut self, mut confl: Option<Clause>, out_learnt: &mut Vec<Lit>) -> i32 {
        trace!(
            "{}|{}|{}|{:?}",
            "analyse".to_string(),
            file!(),
            line!(),
            confl
        );

        let mut out_btlevel: i32 = 0;
        let mut path_c: i32 = 0;
        let mut p: Lit = Lit::undefined();
        out_learnt.push(Lit::empty()); // (leave room for the asserting literal)
        let mut index: i32 = (self.trail.len() - 1) as i32;

        while {
            {
                assert!(confl.is_some());
                let c: &Clause = &confl.unwrap();

                if c.is_learnt {
                    self.cla_bump_activity(&mut c.clone());
                }

                let start: usize = if p == Lit::undefined() { 0 } else { 1 };

                for idata in start..c.data.len() {
                    let q: Lit = c.data[idata];
                    if self.analyze_seen[var(&q) as usize] == Lbool::Undef0
                        && self.level[var(&q) as usize] > 0
                    {
                        self.var_bump_activity(q);
                        self.analyze_seen[var(&q) as usize] = Lbool::True;
                        if self.level[var(&q) as usize] == self.decision_level() {
                            path_c += 1;
                        } else {
                            out_learnt.push(q);
                            out_btlevel = max(out_btlevel, self.level[var(&q) as usize])
                        }
                    }
                }
                loop {
                    if self.analyze_seen[var(&self.trail[index as usize]) as usize] == Lbool::Undef0
                    {
                        index -= 1;
                    } else {
                        index -= 1;
                        break;
                    }
                }
                p = self.trail[(index + 1) as usize];
                confl = self.reason[var(&p) as usize].clone();
                self.analyze_seen[var(&p) as usize] = Lbool::Undef0;
                path_c -= 1;
            }
            path_c > 0
        } {}
        out_learnt[0] = !p;

        {
            let mut i: usize = 1;
            let mut j;

            if self.expensive_ccmin {
                let mut min_level: u32 = 0;
                for _y in 1..out_learnt.len() {
                    let v = var(&out_learnt[i]);
                    let l = self.level[v as usize];
                    min_level |= 1 << (l & 31);
                    i += 1;
                }
                self.analyze_toclear.clear();
                i = 1;
                j = 1;
                for _y in 1..out_learnt.len() {
                    match self.reason[var(&out_learnt[i]) as usize] {
                        None => {
                            out_learnt[j] = out_learnt[i];
                            j += 1;
                        }
                        _ => {
                            if !self.analyze_removeable(out_learnt[i], min_level) {
                                out_learnt[j] = out_learnt[i];
                                j += 1;
                            }
                        }
                    }
                    i += 1;
                }
            } else {
                self.analyze_toclear.clear();
                i = 1;
                j = 1;
                let mut keep: bool = false;
                for _y in 1..out_learnt.len() {
                    match self.reason[var(&out_learnt[i]) as usize] {
                        Some(ref p) => {
                            let c: &Clause = p;
                            for k in 1..c.data.len() {
                                if self.analyze_seen[var(&c.data[k]) as usize] == Lbool::Undef0
                                    && self.level[var(&c.data[k]) as usize] != 0
                                {
                                    j += 1;
                                    out_learnt[j] = out_learnt[i];
                                    keep = true;
                                    break;
                                }
                            }
                        }
                        None => {
                            out_learnt[j] = out_learnt[i];
                        }
                    }

                    if !keep {
                        self.analyze_toclear.push(out_learnt[i]);
                    }
                    i += 1;
                }
            }
            {
                for learnt in out_learnt.iter_mut() {
                    self.analyze_seen[var(learnt) as usize] = Lbool::Undef0;
                }

                for y in 0..self.analyze_toclear.len() {
                    self.analyze_seen[var(&self.analyze_toclear[y]) as usize] = Lbool::Undef0;
                }
            }

            self.solver_stats.max_literals += out_learnt.len() as f64;
            out_learnt.truncate(out_learnt.len() - (i - j));
            self.solver_stats.tot_literals += out_learnt.len() as f64;
        }

        out_btlevel
    }

    fn analyze_removeable(&mut self, _p: Lit, min_level: u32) -> bool {
        trace!(
            "{}|{}|{}|{:?}",
            "analyze removeable".to_string(),
            file!(),
            line!(),
            _p,
        );
        assert!(self.reason[var(&_p) as usize].is_some());

        self.analyze_stack.clear();
        self.analyze_stack.push(_p);
        let top: i32 = self.analyze_toclear.len() as i32;

        while !self.analyze_stack.is_empty() {
            assert!(self.reason[var(self.analyze_stack.last().unwrap()) as usize].is_some());
            let c: &Clause;
            match &self.reason[var(self.analyze_stack.last().unwrap()) as usize] {
                Some(clause) => {
                    c = &clause;
                    self.analyze_stack.pop();
                    for i in 1..c.data.len() {
                        let p: Lit = c.data[i];
                        if self.analyze_seen[var(&p) as usize] == Lbool::Undef0
                            && self.level[var(&p) as usize] != 0
                        {
                            if self.reason[var(&p) as usize].is_some()
                                && ((1 << self.level[var(&p) as usize] & 31) & min_level) != 0
                            {
                                self.analyze_seen[var(&p) as usize] = Lbool::True;
                                self.analyze_stack.push(p);
                                self.analyze_toclear.push(p);
                            } else {
                                for j in top..self.analyze_toclear.len() as i32 {
                                    self.analyze_seen
                                        [var(&self.analyze_toclear[j as usize]) as usize] =
                                        Lbool::Undef0;
                                }
                                self.analyze_toclear.truncate(top as usize);
                                return false;
                            }
                        }
                    }
                }
                None => {
                    self.analyze_stack.pop();
                }
            }
        }
        self.analyze_toclear.push(_p);
        true
    }
}
