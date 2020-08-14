use crate::models::heap::*;
use crate::models::lbool::is_undefined;
use crate::models::lit::*;
use crate::models::random::{drand, irand};
use crate::models::solverstate::*;

#[derive(Clone)]
pub struct VarOrder {
    pub assigns: Assigns,
    pub activity: Activity,
    pub heap: Heap,
    pub random_seed: f64,
}

pub trait IVarOrder {
    fn new() -> Self;
    fn new_var(&mut self);
    fn update(&mut self, x: i32);
    fn undo(&mut self, x: i32);
    fn select(&mut self, random_var_freq: f64) -> Lit;
}

impl IVarOrder for VarOrder {
    fn new() -> Self {
        let n_heap = Heap::new();
        let _self = Self {
            assigns: Assigns { col: Vec::new() },
            activity: Activity { col: Vec::new() },
            heap: n_heap,
            random_seed: 91648253.0,
        };
        return _self;
    }

    fn new_var(&mut self) {
        self.heap.set_bounds(self.assigns.col.len() as i32);
        self.heap.insert(
            self.assigns.col.len() as i32 - 1,
            &var_lt,
            self.activity.col.to_vec(),
        );
    }
    fn update(&mut self, x: i32) {
        if self.heap.in_heap(x) {
            self.heap.increase(x, &var_lt, self.activity.col.to_vec());
        }
    }
    fn undo(&mut self, x: i32) {
        if !self.heap.in_heap(x) {
            self.heap.insert(x, &var_lt, self.activity.col.to_vec());
        }
    }
    fn select(&mut self, random_var_freq: f64) -> Lit {
        let random = drand(self.random_seed as f64);
        if random < random_var_freq as f64 && !self.heap.empty() {
            let next: i32 = irand(random, self.assigns.col.len() as i32);
            if is_undefined(self.assigns.col[next as usize]) {
                return Lit::simple(next);
            }
        }

        while !self.heap.empty() {
            let next: i32 = self.heap.getmin(&var_lt, self.activity.col.clone());

            if is_undefined(self.assigns.col[next as usize]) {
                let r = !Lit::simple(next);
                return r;
            }
        }

        return Lit::undefined();
    }
}

fn var_lt(x: f64, y: f64) -> bool {
    return x > y;
}
