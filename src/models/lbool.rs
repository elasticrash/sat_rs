use crate::models::lit::*;
use crate::models::solverstate::SolverState;

pub static L_TRUE: Lbool = Lbool::True;
pub static L_FALSE: Lbool = Lbool::False;

#[derive(Copy, Clone, PartialEq)]
pub enum Lbool {
    True = 1,
    False = -2,
    Undef0 = 0,
    Undef1 = -1,
}

pub fn to_bool(value: bool) -> Lbool {
    if value {
        return Lbool::True;
    } else {
        return Lbool::False;
    }
}

pub fn is_undefined(value: Lbool) -> bool {
    return value != Lbool::True && value != Lbool::False;
}

pub fn value_by_var(x: i32, y: &SolverState) -> Lbool {
    return y.assigns[x as usize];
}

pub fn value_by_lit(x: Lit, solver_state: &SolverState) -> Lbool {
    let mut assign = solver_state.assigns[var(&x) as usize];
    if sign(&x) {
        assign = bit_not(assign);
    }
    return assign;
}

// bitwise not for lbool
fn bit_not(lb: Lbool) -> Lbool {
    let result = !(lb as i8);
    return match result {
        1 => Lbool::True,
        -2 => Lbool::False,
        0 => Lbool::Undef0,
        -1 => Lbool::Undef1,
        _ => Lbool::Undef1,
    };
}
