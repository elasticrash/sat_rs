use std::cmp::Ordering;
use std::ops::Not;

pub trait Heap {}

pub struct Clause {}

pub struct Lit {
    pub x: i32,
}

pub trait ILit {
    fn new(v: i32, sign: bool) -> Self;
    fn simple(v: i32) -> Self;
    fn to_string() -> str;
    fn get_hash_code() -> i32;
    fn equals(object:lit) -> bool
}

impl Not for Lit {
    type Output = Lit;
    fn not(self) -> Lit {
        let mut q:Lit = Lit{x:0};
        q.x = self.x ^ 1;
        return q;
    }
}

impl PartialOrd for Lit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

impl PartialEq for Lit {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

pub enum Lbool {
    True = 1,
    False = -2,
    Undef0 = 0,
    Undef1 = -1,
}

pub struct Solver {
    clauses: Vec<Clause>,
    learnts: Vec<Clause>,
    activity: Vec<f64>,
    watches: Vec<Vec<Clause>>,
    trail_pos: Vec<i32>,
    trail: Vec<Lit>,
    trail_lim: Vec<i32>,
    reason: Vec<Clause>,
    level: Vec<i32>,
    analyze_seen: Vec<Lbool>,
    analyze_stack: Vec<Lit>,
    analyze_toclear: Vec<Lit>,
    addUnit_tmp: Vec<Lit>,
    addBinary_tmp: Vec<Lit>,
    addTernary_tmp: Vec<Lit>,
    model: Vec<Lbool>,
    conflict: Vec<Lit>,
}

fn main() {}

fn solver() {}
