mod functions;
mod models;
use models::clause::Clause;
use models::lbool::Lbool;
use models::lit::ILit;
use models::lit::Lit;
use models::solverstate::*;

static l_True: Lbool = Lbool::True;
static l_False: Lbool = Lbool::False;
static var_undefined: i32 = -1;

fn main() {}

fn solver() {
    let lit_undefined: Lit = Lit::new(var_undefined, true);
    // let solver = SolverState{

    // }
}
