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
