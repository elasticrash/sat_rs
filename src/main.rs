mod models;
use models::clause::Clause;
use models::lit::ILit;
use models::lit::Lit;

static l_True: Lbool = Lbool::True;
static l_False: Lbool = Lbool::False;
static var_undefined: i32 = -1;
pub trait Heap {}

pub enum Lbool {
    True = 1,
    False = -2,
    Undef0 = 0,
    Undef1 = -1,
}

impl PartialEq for Lbool {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
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

fn to_bool(value: bool) -> Lbool {
    if value {
        return Lbool::True;
    } else {
        return Lbool::False;
    }
}

fn is_undefined(value: Lbool) -> bool {
    return value != Lbool::True && value != Lbool::False;
}

fn main() {}

fn solver() {
    let lit_undefined: Lit = Lit::new(var_undefined, true);
}
