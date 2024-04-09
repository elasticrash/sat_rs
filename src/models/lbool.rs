pub static L_TRUE: Lbool = Lbool::True;
pub static L_FALSE: Lbool = Lbool::False;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Lbool {
    True = 1,
    False = -2,
    Undef0 = 0,
    Undef1 = -1,
}

pub fn to_bool(value: bool) -> Lbool {
    match value {
        true => Lbool::True,
        false => Lbool::False,
    }
}

pub fn is_undefined(value: Lbool) -> bool {
    value != Lbool::True && value != Lbool::False
}

// bitwise not for lbool
pub fn bit_not(lb: Lbool) -> Lbool {
    let result = !(lb as i8);
    match result {
        1 => Lbool::True,
        -2 => Lbool::False,
        0 => Lbool::Undef0,
        -1 => Lbool::Undef1,
        _ => Lbool::Undef1,
    }
}
