use crate::models::heap::*;
use crate::models::lbool::{is_undefined, Lbool};
use crate::models::lit::*;
use crate::models::random::{drand, irand};
use crate::models::solverstate::*;

#[derive(Clone)]
pub struct VarOrder {
    pub assigns: Vec<Lbool>,
    pub activity: Vec<f64>,
    pub heap: Heap,
    pub random_seed: f64,
}

pub trait IVarOrder {
    fn new(ass: Vec<Lbool>, act: Vec<f64>) -> Self;
    fn new_var(&mut self, solver_state: SolverState);
    fn update(&mut self, x: i32);
    fn undo(&mut self, x: i32, solver_state: SolverState);
    fn select(&mut self, random_var_freq: f64, solver_state: SolverState) -> Lit;
    fn sync(&mut self, solver_state: SolverState);
}

impl IVarOrder for VarOrder {
    fn new(ass: Vec<Lbool>, act: Vec<f64>) -> Self {
        return Self {
            assigns: ass,
            activity: act,
            heap: Heap::new(),
            random_seed: 91648253.0,
        };
    }

    fn new_var(&mut self, solver_state: SolverState) {
        <VarOrder as IVarOrder>::sync(self, solver_state);

        self.heap.set_bounds(self.assigns.len() as i32);
        self.heap
            .insert(self.assigns.len() as i32 - 1, self.activity.clone());
    }
    fn update(&mut self, x: i32) {
        if self.heap.in_heap(x) {
            self.heap.increase(x);
        }
    }
    fn undo(&mut self, x: i32, solver_state: SolverState) {
        <VarOrder as IVarOrder>::sync(self, solver_state);

        if self.heap.in_heap(x) {
            self.heap.insert(x, self.activity.clone());
        }
    }
    fn select(&mut self, random_var_freq: f64, solver_state: SolverState) -> Lit {
        <VarOrder as IVarOrder>::sync(self, solver_state.clone());

        let random = drand(self.random_seed as f64);
        if random < random_var_freq as f64 && !self.heap.empty() {
            let next: i32 = irand(random, self.assigns.len() as i32);
            if is_undefined(self.assigns[next as usize]) {
                return Lit::simple(next);
            }
        }

        while !self.heap.empty() {
            let next: i32 = self.heap.getmin(solver_state.activity.to_vec());

            if is_undefined(self.assigns[next as usize]) {
                let r = !Lit::simple(next);
                return r;
            }
        }

        return Lit::new(-1, false);
    }
    fn sync(&mut self, solver_state: SolverState) {
        self.assigns = solver_state.assigns.clone();
        self.activity = solver_state.activity.clone();
    }
}
