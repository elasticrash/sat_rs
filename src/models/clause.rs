use crate::models::lit::Lit;

#[derive(Clone, Debug)]
pub struct Clause {
    pub data: Vec<Lit>,
    pub is_learnt: bool,
    pub id: u32,
    pub activity: f64,
}

pub trait IClause {
    fn new(learnt: bool, ps: &[Lit], id: u32) -> Self;
    fn size(&self) -> i32;
    fn learnt(&self) -> bool;
}

impl IClause for Clause {
    fn new(_learnt: bool, _ps: &[Lit], id: u32) -> Self {
        Self {
            data: _ps.to_vec(),
            is_learnt: _learnt,
            activity: 0.0,
            id,
        }
    }
    fn size(&self) -> i32 {
        self.data.len() as i32
    }
    fn learnt(&self) -> bool {
        self.is_learnt
    }
}

impl PartialEq for Clause {
    fn eq(&self, other: &Self) -> bool {
        if self.activity == other.activity
            && self.data.len() == other.data.len()
            && self.is_learnt == other.is_learnt
        {
            let mut f: String = String::new();
            for y in &self.data {
                f.push_str(&y.x.to_string())
            }

            let mut s: String = String::new();
            for y in &other.data {
                s.push_str(&y.x.to_string())
            }

            if f == s {
                return true;
            }
        }
        false
    }
}
