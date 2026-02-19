use crate::functions::enqueue::*;
use crate::models::clause::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::solverstate::*;

/*_________________________________________________________________________________________________
|
|  propagate
|
|  Description:
|    Propagates all enqueued facts. If a conflict arises, the conflicting clause is returned,
|    otherwise null. NOTE! This method has been optimized for speed rather than readability.
|
|    Post-conditions:
|      * the propagation queue is empty, even if there was a conflict.
|________________________________________________________________________________________________@*/
pub trait Prop {
    fn propagate(&mut self) -> Option<Clause>;
}

impl Prop for SolverState {
    fn propagate(&mut self) -> Option<Clause> {
        trace!("{}|{}|{}", "propagate".to_string(), file!(), line!(),);

        let mut confl: Option<Clause> = None;

        while self.qhead < self.trail.len() as i32 {
            self.solver_stats.propagations += 1.0;
            self.simp_db_props -= 1.0;

            let p: Lit = self.trail[self.qhead as usize];
            self.qhead += 1;
            let mut ws: Vec<Clause> = self.watches[p.x as usize].clone();

            //log p
            let mut i: usize = 0;
            let mut j: usize = 0;
            let end: usize = i + ws.len();
            while i != end {
                let mut c: Clause = ws[i].clone();

                i += 1;
                let false_lit: Lit = !p;

                if c.data[0] == false_lit {
                    c.data[0] = c.data[1];
                    c.data[1] = false_lit;
                }

                assert!(c.data[1] == false_lit);

                let first: Lit = c.data[0];
                let val: Lbool = self.value_by_lit(first);
                if val == L_TRUE {
                    ws[j] = c.clone();
                    j += 1;
                } else {
                    let mut foundwatch: bool = false;
                    for k in 2..c.data.len() {
                        if self.value_by_lit(c.data[k]) != L_FALSE {
                            c.data[1] = c.data[k];
                            c.data[k] = false_lit;
                            self.watches[(!c.data[1]).x as usize].push(c.clone());
                            let mut c_sorted: Vec<i32> = c.data.iter().map(|l| l.x).collect();
                            c_sorted.sort_unstable();
                            let other_ws = &mut self.watches[(!first).x as usize];
                            for entry in other_ws.iter_mut() {
                                let watches_pair = (entry.data[0] == first
                                    && entry.data[1] == false_lit)
                                    || (entry.data[0] == false_lit && entry.data[1] == first);
                                if watches_pair {
                                    let mut e_sorted: Vec<i32> =
                                        entry.data.iter().map(|l| l.x).collect();
                                    e_sorted.sort_unstable();
                                    if c_sorted == e_sorted {
                                        *entry = c.clone();
                                        break;
                                    }
                                }
                            }
                            foundwatch = true;
                            break;
                        }
                    }
                    if !foundwatch {
                        ws[j] = c.clone();
                        j += 1;
                        if !self.enqueue(&first, Some(c.clone())) {
                            if self.decision_level() == 0 {
                                self.ok = false;
                            }
                            confl = Some(c.clone());
                            self.qhead = self.trail.len() as i32;

                            while i < end {
                                ws[j] = ws[i].clone();
                                j += 1;
                                i += 1;
                            }
                        }
                    }
                }
            }
            ws.truncate(ws.len() - (i - j));
            self.watches[p.x as usize] = ws;
        }
        confl
    }
}
