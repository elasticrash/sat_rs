mod lit;
use lit::Lit;

pub trait Heap {}

pub struct Clause {}

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
