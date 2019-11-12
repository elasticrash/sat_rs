use crate::models::varorder::*;
use crate::models::logger::*;

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
    fn percolate_down(&mut self, i: i32);
    fn empty(&self) -> bool;
    fn getmin(&mut self) -> i32;
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
    fn percolate_up(&mut self, mut _i: i32) {
        reportf("percolate_up".to_string());

        let x = self.heap[_i as usize];
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
    fn percolate_down(&mut self, mut _i: i32) {
        reportf("percolate_down".to_string());

        let x = self.heap[_i as usize];
        while <Heap as IHeap>::left(_i) < self.heap.len() as i32 {
            let child: i32;
            if <Heap as IHeap>::right(_i) < self.heap.len() as i32
                && self.activities[self.heap[<Heap as IHeap>::right(_i) as usize] as usize]
                    > self.activities[self.heap[<Heap as IHeap>::left(_i) as usize] as usize]
            {
                child = <Heap as IHeap>::right(_i)
            } else {
                child = <Heap as IHeap>::left(_i);
            }

            if self.activities[child as usize] > self.activities[x as usize] {
                break;
            }

            self.heap[_i as usize] = self.heap[child as usize];
            self.indices[self.heap[_i as usize] as usize] = _i;
            _i = child;
        }

        self.heap[_i as usize] = x;
        self.indices[x as usize] = _i;
    }
    fn empty(&self) -> bool {
        return self.heap.len() == 1 as usize;
    }
    fn getmin(&mut self) -> i32 {
        let r = self.heap[1];
        self.heap[1] = *self.heap.last().unwrap();
        self.indices[self.heap[1] as usize] = 1;
        self.indices[r as usize] = 0;
        self.heap.pop();
        if self.heap.len() > 1 {
            <Heap as IHeap>::percolate_down(self, 1);
        }
        return r;
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
