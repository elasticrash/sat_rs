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
    fn new_var(&mut self);
    fn update(&mut self, x: i32);
    fn undo(&mut self, x: i32);
    fn select(&mut self, random_var_freq: f64) -> Lit;
}

impl Default for VarOrder {
    fn default() -> VarOrder {
        VarOrder {
            assigns: Assigns { col: Vec::new() },
            activity: Activity { col: Vec::new() },
            heap: Heap::new(),
            random_seed: 91648253.0,
        }
    }
}

impl IVarOrder for VarOrder {
    fn new_var(&mut self) {
        self.heap.set_bounds(self.assigns.col.len() as i32);
        self.heap.insert(
            self.assigns.col.len() as i32 - 1,
            self.activity.col.to_vec(),
        );
    }
    fn update(&mut self, x: i32) {
        if self.heap.in_heap(x) {
            self.heap.increase(x, self.activity.col.to_vec());
        }
    }
    fn undo(&mut self, x: i32) {
        if !self.heap.in_heap(x) {
            self.heap.insert(x, self.activity.col.to_vec());
        }
    }
    fn select(&mut self, random_var_freq: f64) -> Lit {
        if !self.heap.empty() && drand(&mut self.random_seed) < random_var_freq {
            let next: i32 = irand(&mut self.random_seed, self.assigns.col.len() as i32);
            if is_undefined(self.assigns.col[next as usize]) {
                return !Lit::simple(next);
            }
        }

        while !self.heap.empty() {
            let next: i32 = self.heap.getmin(self.activity.col.clone());

            if is_undefined(self.assigns.col[next as usize]) {
                return !Lit::simple(next);
            }
        }

        Lit::undefined()
    }
}

#[cfg(test)]
mod tests {
    use crate::models::lbool::*;
    use crate::models::varorder::*;

    #[test]
    fn new_var_order() {
        let vo = VarOrder::default();
        assert_eq!(vo.assigns.col.len(), 0);
        assert_eq!(vo.activity.col.len(), 0);
        assert_eq!(vo.heap.heap.len(), 1);
        assert_eq!(vo.heap.heap[0], -1);
        assert_eq!(vo.heap.indices.len(), 0);
    }

    #[test]

    fn insert_new_var() {
        let mut vo = VarOrder::default();
        vo.assigns.col.push(Lbool::True);
        vo.new_var();
        assert_eq!(vo.assigns.col.len(), 1);
        assert_eq!(vo.heap.indices.len(), 1);
        assert_eq!(vo.heap.heap.len() as i32, 2);
        assert_eq!(vo.heap.indices[0] as i32, 1);
    }
}
