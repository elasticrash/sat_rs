use crate::models::lit::*;
use crate::models::solverstate::SolverState;
use std::ops::Not;

pub static L_TRUE: Lbool = Lbool::True;
pub static L_FALSE: Lbool = Lbool::False;

#[derive(Copy, Clone)]
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

pub fn value_by_var(x: i32, y: SolverState) -> Lbool {
    return y.assigns[x as usize];
}

pub fn value_by_lit(x: Lit, y: &SolverState) -> Lbool {
    if sign(&x) {
        return !y.assigns[var(&x) as usize];
    } else {
        return y.assigns[var(&x) as usize];
    }
}
