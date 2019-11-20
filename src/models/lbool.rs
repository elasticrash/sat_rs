use crate::models::lit::*;
use crate::models::solverstate::SolverState;
use std::ops::BitAnd;
use std::ops::Not;

pub static L_TRUE: Lbool = Lbool::True;
pub static L_FALSE: Lbool = Lbool::False;

#[derive(Copy, Clone, PartialEq)]
pub enum Lbool {
    True = 1,
    False = -2,
    Undef0 = 0,
    Undef1 = -1,
}

impl Not for Lbool {
    type Output = Lbool;
    fn not(self) -> Lbool {
        if self == Lbool::True {
            return Lbool::False;
        } else {
            return Lbool::True;
        }
    }
}

impl BitAnd for Lbool {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        let result = !(self as i8);
        match result {
            1 => Lbool::True,
            -2 => Lbool::False,
            0 => Lbool::Undef0,
            -1 => Lbool::Undef1,
            _ => Lbool::Undef1,
        }
    }
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
        // using the bitwise and to do bitwise NOT
        assign = assign & assign;
    }
    return assign;
}
