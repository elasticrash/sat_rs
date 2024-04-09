use crate::functions::new_clause::*;
use crate::models::clause::*;
use crate::models::solverstate::*;
use std::cmp::Ordering;

/*_________________________________________________________________________________________________
|
|  reduceDB
|
|  Description:
|    Remove half of the learnt clauses, minus the clauses locked by the current assignment. Locked
|    clauses are clauses that are reason to some assignment. Binary clauses are never removed.
|________________________________________________________________________________________________@*/
pub trait Reduce {
    fn reduce_db(&mut self);
}

impl Reduce for SolverState {
    fn reduce_db(&mut self) {
        trace!("{}|{}|{}", "reduce_db".to_string(), file!(), line!());

        let mut i: i32 = 0;
        let mut j: i32 = 0;

        let extra_lim: f64 = self.cla_inc / self.learnts.len() as f64;

        self.learnts.sort_by(|x, y| {
            if x.size() > 2 && (y.size() == 2 || x.activity < y.activity) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        for y in 0..self.learnts.len() / 2 {
            if self.learnts[y].data.len() > 2 && !self.locked(&self.learnts[y].clone()) {
                self.remove(self.learnts[y].clone(), false);
            } else {
                self.learnts[j as usize] = self.learnts[y].clone();
                j += 1;
            }
            i = y as i32;
        }

        for y in i..self.learnts.len() as i32 {
            if self.learnts[y as usize].data.len() > 2
                && !self.locked(&self.learnts[y as usize].clone())
                && self.learnts[y as usize].activity < extra_lim
            {
                self.remove(self.learnts[y as usize].clone(), false);
            } else {
                self.learnts[j as usize] = self.learnts[i as usize].clone();
                j += 1;
            }
        }
        self.learnts.truncate(self.learnts.len() - (i - j) as usize)
    }
}
