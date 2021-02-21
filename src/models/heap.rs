#[derive(Clone)]
pub struct Heap {
    pub heap: Vec<i32>,
    pub indices: Vec<i32>,
}

pub trait IHeap {
    fn new() -> Self;
    fn set_bounds(&mut self, n: i32);
    fn in_heap(&self, n: i32) -> bool;
    fn increase(&mut self, n: i32, act: Vec<f64>);
    fn insert(&mut self, n: i32, act: Vec<f64>);
    fn percolate_up(&mut self, i: i32, act: Vec<f64>);
    fn percolate_down(&mut self, i: i32, act: Vec<f64>);
    fn empty(&self) -> bool;
    fn getmin(&mut self, act: Vec<f64>) -> i32;
    fn compare(x: f64, y: f64) -> bool;
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
    fn increase(&mut self, n: i32, act: Vec<f64>) {
        assert!(self.indices[n as usize] != 0);
        <Heap as IHeap>::percolate_up(self, self.indices[n as usize], act.to_vec());
    }
    fn insert(&mut self, n: i32, act: Vec<f64>) {
        self.indices[n as usize] = self.heap.len() as i32;
        self.heap.push(n);
        <Heap as IHeap>::percolate_up(self, self.indices[n as usize], act.to_vec());
    }
    fn percolate_up(&mut self, mut _i: i32, act: Vec<f64>) {
        trace!(
            "{}|{}|{}|{}|{:?}",
            "percolate_up".to_string(),
            file!(),
            line!(),
            _i,
            act
        );

        let x = self.heap[_i as usize];
        while (_i >> 1) != 0
            && <Heap as IHeap>::compare(
                act[x as usize],
                act[self.heap[(_i >> 1) as usize] as usize],
            )
        {
            self.heap[_i as usize] = self.heap[(_i >> 1) as usize];
            self.indices[self.heap[_i as usize] as usize] = _i;
            _i = _i >> 1;
        }

        self.heap[_i as usize] = x;
        self.indices[x as usize] = _i;
    }
    fn percolate_down(&mut self, mut _i: i32, act: Vec<f64>) {
        trace!(
            "{}|{}|{}|{}|{:?}",
            "percolate_down".to_string(),
            file!(),
            line!(),
            _i,
            act
        );

        let x = self.heap[_i as usize];
        while _i + _i < self.heap.len() as i32 {
            let child: i32;
            if (_i + _i + 1) < self.heap.len() as i32
                && <Heap as IHeap>::compare(
                    act[self.heap[(_i + _i + 1) as usize] as usize],
                    act[self.heap[(_i + _i) as usize] as usize],
                )
            {
                child = _i + _i + 1;
            } else {
                child = _i + _i;
            }

            if !(<Heap as IHeap>::compare(act[self.heap[child as usize] as usize], act[x as usize]))
            {
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
    fn getmin(&mut self, act: Vec<f64>) -> i32 {
        let r = self.heap[1];
        self.heap[1] = *self.heap.last().unwrap();
        self.indices[self.heap[1] as usize] = 1;
        self.indices[r as usize] = 0;
        self.heap.pop();
        if self.heap.len() > 1 {
            <Heap as IHeap>::percolate_down(self, 1, act.to_vec());
        }
        return r;
    }
    fn compare(x: f64, y: f64) -> bool {
        return x > y;
    }
}
