use crate::models::lit::Lit;

pub struct Clause {
    pub data: Vec<Lit>,
    is_learnt: bool,
    activity: f64,
}

pub trait IClause {
    fn new(learnt: bool, ps: Vec<Lit>) -> Self;
    fn size() -> i32;
    fn learnt() -> bool;
    fn get_by_index() -> Lit;
    fn to_string() -> String;
    fn get_data() -> Vec<Lit>;
}
