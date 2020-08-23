use crate::models::logger::*;

#[derive(Clone)]
pub struct Heap {
    pub heap: Vec<i32>,
    pub indices: Vec<i32>,
}

pub trait IHeap {
    fn new() -> Self;
    fn set_bounds(&mut self, n: i32);
    fn in_heap(&self, n: i32) -> bool;
    fn increase(&mut self, n: i32, comp: &dyn Fn(f64, f64) -> bool, act: Vec<f64>);
    fn insert(&mut self, n: i32, comp: &dyn Fn(f64, f64) -> bool, act: Vec<f64>);
    fn percolate_up(&mut self, i: i32, comp: &dyn Fn(f64, f64) -> bool, act: Vec<f64>);
    fn percolate_down(&mut self, i: i32, comp: &dyn Fn(f64, f64) -> bool, act: Vec<f64>);
    fn empty(&self) -> bool;
    fn getmin(&mut self, comp: &dyn Fn(f64, f64) -> bool, act: Vec<f64>) -> i32;
}

impl IHeap for Heap {
    fn new() -> Self {
        let mut h = Vec::new();
        h.push(-1);
        return Self {
            heap: h,
            indices: Vec::new(),
        };
    }
    fn set_bounds(&mut self, n: i32) {
        self.indices.resize(n as usize, 0);
    }
    fn in_heap(&self, n: i32) -> bool {
        return self.indices[n as usize] != 0;
    }
    fn increase(&mut self, n: i32, comp: &dyn Fn(f64, f64) -> bool, act: Vec<f64>) {
        <Heap as IHeap>::percolate_up(self, self.indices[n as usize], comp, act.to_vec());
    }
    fn insert(&mut self, n: i32, comp: &dyn Fn(f64, f64) -> bool, act: Vec<f64>) {
        self.indices[n as usize] = self.heap.len() as i32;
        self.heap.push(n);
        <Heap as IHeap>::percolate_up(self, self.indices[n as usize], comp, act.to_vec());
    }
    fn percolate_up(&mut self, mut _i: i32, comp: &dyn Fn(f64, f64) -> bool, act: Vec<f64>) {
        reportf("percolate_up".to_string(), file!(), line!(), 0);

        let x = self.heap[_i as usize];
        while (_i >> 1) != 0 && comp(act[x as usize], act[self.heap[(_i >> 1) as usize] as usize]) {
            self.heap[_i as usize] = self.heap[(_i >> 1) as usize];
            self.indices[self.heap[_i as usize] as usize] = _i;
            _i = _i >> 1;
        }

        self.heap[_i as usize] = x;
        self.indices[x as usize] = _i;
    }
    fn percolate_down(&mut self, mut _i: i32, comp: &dyn Fn(f64, f64) -> bool, act: Vec<f64>) {
        reportf("percolate_down".to_string(), file!(), line!(), 0);

        let x = self.heap[_i as usize];
        while _i + _i < self.heap.len() as i32 {
            let child: i32;
            if (_i + _i + 1) < self.heap.len() as i32
                && comp(
                    act[self.heap[(_i + _i + 1) as usize] as usize],
                    act[self.heap[(_i + _i) as usize] as usize],
                )
            {
                child = _i + _i + 1;
            } else {
                child = _i + _i;
            }

            if !(comp(act[self.heap[child as usize] as usize], act[x as usize])) {
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
    fn getmin(&mut self, comp: &dyn Fn(f64, f64) -> bool, act: Vec<f64>) -> i32 {
        let r = self.heap[1];
        self.heap[1] = *self.heap.last().unwrap();
        self.indices[self.heap[1] as usize] = 1;
        self.indices[r as usize] = 0;
        self.heap.pop();
        if self.heap.len() > 1 {
            <Heap as IHeap>::percolate_down(self, 1, comp, act.to_vec());
        }
        return r;
    }
}
