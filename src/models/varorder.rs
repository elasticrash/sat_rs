use crate::models::heap::Heap;
use crate::models::heap::IHeap;
use crate::models::lbool::{is_undefined, Lbool};
use crate::models::lit::ILit;
use crate::models::lit::Lit;
use crate::models::random::{drand, irand};

pub struct VarOrder {
    pub assigns: Vec<Lbool>,
    pub activity: Vec<f64>,
    pub heap: Heap,
    pub random_seed: f64,
    pub lt: fn(x: i32, y: i32) -> bool,
}

pub trait IVarOrder {
    fn new(self, ass: Vec<Lbool>, act: Vec<f64>);
    fn lt(&self, x: i32, y: i32) -> bool;
    fn newVar(self);
    fn update(self, x: i32);
    fn undo(self, x: i32);
    fn select_default(&self) -> Lit;
    fn select(&self, random_var_freq: f64) -> Lit;
}

impl IVarOrder for VarOrder {
    fn new(mut self, ass: Vec<Lbool>, act: Vec<f64>) {
        self.assigns = ass;
        self.activity = act;
        self.heap = Heap::new(self.lt);
        self.random_seed = 91648253.0;
    }
    fn lt(&self, x: i32, y: i32) -> bool {
        return &self.activity[x as usize] > &self.activity[y as usize];
    }
    fn newVar(mut self) {}
    fn update(mut self, x: i32) {
        if self.heap.in_heap(x) {
            self.heap.increase(x);
        }
    }
    fn undo(mut self, x: i32) {
        if self.heap.in_heap(x) {
            self.heap.insert(x);
        }
    }
    fn select_default(&self) -> Lit {
        return <VarOrder as IVarOrder>::select(&self, 0.0);
    }
    fn select(&self, random_var_freq: f64) -> Lit {
        let random = drand(self.random_seed as f64);
        if random < random_var_freq as f64 && !self.heap.empty() {
            let next: i32 = irand(random, self.assigns.len() as i32);
            if is_undefined(self.assigns[next as usize]) {
                return Lit::simple(next);
            }
        }

        while !self.heap.empty() {
            let next: i32 = self.heap.getmin();
            if is_undefined(self.assigns[next as usize]) {
                return Lit::simple(next);
            }
        }

        return Lit::new(-1, true);
    }
}
