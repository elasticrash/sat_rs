use crate::models::solverstate::*;

pub trait Dpll {
    fn model_found(&mut self) -> bool;
}

impl Dpll for SolverState {
    fn model_found(&mut self) -> bool {
        self.level_to_backtrack = i32::max_value();
        let res: bool = false;

        if !self.ok {
            return false;
        }

        res
    }
}
