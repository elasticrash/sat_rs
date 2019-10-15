use crate::models::lit::Lit;

#[derive(Clone)]
pub struct Clause {
    pub data: Vec<Lit>,
    is_learnt: bool,
    activity: f64,
}

pub trait IClause {
    fn new(learnt: bool, ps: &Vec<Lit>) -> Self;
    fn size(&self) -> i32;
    fn learnt() -> bool;
    fn get_by_index(&self) -> Lit;
    fn to_string() -> String;
    fn get_data(&self) -> &Vec<Lit>;
}

impl IClause for Clause {
    fn new(learnt: bool, ps: &Vec<Lit>) -> Self {
        return Self {
            data: Vec::new(),
            is_learnt: true,
            activity: 0.0,
        };
    }
    fn size(&self) -> i32 {
        return 0;
    }
    fn learnt() -> bool {
        return true;
    }
    fn get_by_index(&self) -> Lit {
        return self.data[0];
    }
    fn to_string() -> String {
        return String::new();
    }
    fn get_data(&self) -> &Vec<Lit> {
        return &self.data;
    }
}
