use crate::models::solverstate::*;

pub trait DPLL {
    fn model_found(&mut self) -> bool;
}

impl DPLL for SolverState {
    fn model_found(&mut self) -> bool {
        self.level_to_backtrack = i32::max_value();
        let res: bool = false;

        if !self.ok {
            return false;
        }
        return res;
    }
}
