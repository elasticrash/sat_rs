use crate::models::lit::Lit;

pub struct Clause {
    pub data: Vec<Lit>,
    is_learnt: bool,
}
