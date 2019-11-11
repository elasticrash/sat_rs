use crate::models::lbool::*;
use crate::models::varorder::*;

#[derive(Clone)]
pub struct Heap {
    pub comp: Box<fn(&VarOrder, i32, i32) -> bool>,
    pub heap: Vec<i32>,
    pub indices: Vec<i32>,
    pub activities: Vec<f64>,
}

pub trait IHeap {
    fn new(v: fn(&VarOrder, i32, i32) -> bool, act: &Vec<f64>) -> Self;
    fn set_bounds(&mut self, n: i32);
    fn in_heap(&self, n: i32) -> bool;
    fn increase(&mut self, n: i32);
    fn insert(&mut self, n: i32);
    fn percolate_up(&mut self, i: i32);
    fn percolate_down(i: i32);
    fn empty(&self) -> bool;
    fn getmin(&self) -> i32;
    fn left(n: i32) -> i32;
    fn right(n: i32) -> i32;
    fn parent(n: i32) -> i32;
}

impl IHeap for Heap {
    fn new(v: fn(&VarOrder, i32, i32) -> bool, act: &Vec<f64>) -> Self {
        return Self {
            comp: Box::new(v),
            heap: Vec::new(),
            indices: Vec::new(),
            activities: act.clone(),
        };
    }
    fn set_bounds(&mut self, n: i32) {
        self.indices.resize(n as usize, 0);
    }
    fn in_heap(&self, n: i32) -> bool {
        return self.indices[n as usize] != 0;
    }
    fn increase(&mut self, n: i32) {
        <Heap as IHeap>::percolate_up(self, self.indices[n as usize]);
    }
    fn insert(&mut self, n: i32) {
        // this check is to stop it panicking (until the whole thing is finished)
        if n >= 0 {
            self.indices[n as usize] = self.heap.len() as i32;
            self.heap.push(n);
            <Heap as IHeap>::percolate_up(self, self.indices[n as usize]);
        }
    }
    // TODO
    fn percolate_up(&mut self, mut _i: i32) {
        let x = self.heap[_i as usize];
        //comp(x, self.heap[<Heap as IHeap>::parent(_i) as usize])
        while <Heap as IHeap>::parent(_i) != 0
            && self.activities[x as usize]
                > self.activities[self.heap[<Heap as IHeap>::parent(_i) as usize] as usize]
        {
            self.heap[_i as usize] = self.heap[<Heap as IHeap>::parent(_i) as usize];
            self.indices[self.heap[_i as usize] as usize] = _i;
            _i = <Heap as IHeap>::parent(_i);
        }

        self.heap[_i as usize] = x;
        self.indices[x as usize] = _i;
    }
    // TODO
    fn percolate_down(_i: i32) {}
    fn empty(&self) -> bool {
        return self.heap.len() == 1 as usize;
    }
    // TODO
    fn getmin(&self) -> i32 {
        return 0;
    }
    fn left(n: i32) -> i32 {
        return n + n;
    }
    fn right(n: i32) -> i32 {
        return n + n + 1;
    }
    fn parent(n: i32) -> i32 {
        return n >> 1;
    }
}
