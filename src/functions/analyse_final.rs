use crate::models::clause::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::solverstate::*;

/*_________________________________________________________________________________________________
|
|  analyzeFinal
|
|  Description:
|    Specialized analysis procedure to express the final conflict in terms of assumptions.
|    'root_level' is allowed to point beyond end of trace (useful if called after conflict while
|    making assumptions). If 'skip_first' is TRUE, the first literal of 'confl' is  ignored (needed
|    if conflict arose before search even started).
|________________________________________________________________________________________________@*/
pub trait Final {
    fn analyse_final(&mut self, _confl: &Clause, _skip_first: bool);
}

impl Final for SolverState {
    fn analyse_final(&mut self, _confl: &Clause, _skip_first: bool) {
        trace!(
            "{}|{}|{}|{:?}",
            "analyse final".to_string(),
            file!(),
            line!(),
            _confl,
        );

        self.conflict.clear();
        if self.root_level == 0 {
            return;
        }

        let istart: i32 = match _skip_first {
            true => 1,
            false => 0,
        };

        for _y in istart.._confl.data.len() as i32 {
            let x: usize = var(&_confl.data[_y as usize]) as usize;
            if self.level[x] > 0 {
                self.analyze_seen[x] = Lbool::True;
            }
        }

        let start: i32 = match self.root_level >= self.trail_lim.len() as i32 {
            true => (self.trail.len() - 1) as i32,
            false => self.trail_lim[self.root_level as usize],
        };

        for y in (self.trail_lim[0]..=start).rev() {
            let x: usize = var(&self.trail[y as usize]) as usize;

            if self.analyze_seen[x] != Lbool::Undef0 {
                match &self.reason[x] {
                    Some(clause) => {
                        for j in 1..clause.data.len() {
                            if self.level[var(&clause.data[j as usize]) as usize] > 0 {
                                self.analyze_seen[var(&clause.data[j as usize]) as usize] =
                                    Lbool::True;
                            }
                        }
                    }
                    None => {
                        assert!(self.level[x] > 0);
                        self.conflict.push(!self.trail[y as usize]);
                    }
                }

                self.analyze_seen[x] = Lbool::Undef0;
            }
        }
    }
}
