mod functions;
mod models;
use models::clause::Clause;
use models::lbool::Lbool;
use models::lit::ILit;
use models::lit::Lit;
use models::solverstate::*;

static L_TRUE: Lbool = Lbool::True;
static L_FALSE: Lbool = Lbool::False;
static VAR_UNDEFINED: i32 = -1;

fn main() {}

fn solver() {
    let lit_undefined: Lit = Lit::new(VAR_UNDEFINED, true);
    // let solver = SolverState{

    // }
}
