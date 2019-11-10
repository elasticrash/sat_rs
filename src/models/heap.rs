use crate::models::varorder::*;

#[derive(Clone)]
pub struct Heap {
    pub comp: fn(&VarOrder, i32, i32) -> bool,
    pub heap: Vec<i32>,
    pub indices: Vec<i32>,
}

pub trait IHeap {
    fn new(v: fn(&VarOrder, i32, i32) -> bool) -> Self;
    fn set_bounds(&mut self, n: i32);
    fn in_heap(&self, n: i32) -> bool;
    fn increase(&self, n: i32);
    fn insert(&mut self, n: i32);
    fn percolate_up(i: i32);
    fn percolate_down(i: i32);
    fn empty(&self) -> bool;
    fn getmin(&self) -> i32;
}

impl IHeap for Heap {
    fn new(v: fn(&VarOrder, i32, i32) -> bool) -> Self {
        return Self {
            comp: v,
            heap: Vec::new(),
            indices: Vec::new(),
        };
    }
    fn set_bounds(&mut self, n: i32) {
        self.indices.resize(n as usize, 0);
    }
    fn in_heap(&self, n: i32) -> bool {
        return self.indices[n as usize] != 0;
    }
    fn increase(&self, n: i32) {
        <Heap as IHeap>::percolate_up(self.indices[n as usize]);
    }
    fn insert(&mut self, n: i32) {
        // this check is to stop it panicking (until the whole thing is finished)
        if n >= 0 {
            self.indices[n as usize] = self.heap.len() as i32;
            self.heap.push(n);
            <Heap as IHeap>::percolate_up(self.indices[n as usize]);
        }
    }
    // TODO
    fn percolate_up(_i: i32) {}
    // TODO
    fn percolate_down(_i: i32) {}
    fn empty(&self) -> bool {
        return self.heap.len() == 1 as usize;
    }
    // TODO
    fn getmin(&self) -> i32 {
        return 0;
    }
}
