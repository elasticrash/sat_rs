use crate::models::lit::Lit;

pub struct Clause {
    pub data: Vec<Lit>,
    is_learnt: bool,
    activity: f32
}

pub trait IClause {
    fn new() -> Self;
    fn size() -> i32;
    fn learnt() -> bool;
}
