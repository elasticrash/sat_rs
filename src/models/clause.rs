use crate::models::lit::Lit;

#[derive(Clone, Debug)]
pub struct Clause {
    pub data: Vec<Lit>,
    pub is_learnt: bool,
    pub activity: f64,
}

pub trait IClause {
    fn new(learnt: bool, ps: &[Lit]) -> Self;
    fn size(&self) -> i32;
    fn learnt(&self) -> bool;
    fn get_by_index(&self, i: usize) -> Lit;
    fn to_string(&self) -> String;
    fn get_data(&self) -> &Vec<Lit>;
}

impl IClause for Clause {
    fn new(_learnt: bool, _ps: &[Lit]) -> Self {
        Self {
            data: _ps.to_vec(),
            is_learnt: _learnt,
            activity: 0.0,
        }
    }
    fn size(&self) -> i32 {
        self.data.len() as i32
    }
    fn learnt(&self) -> bool {
        self.is_learnt
    }
    fn get_by_index(&self, i: usize) -> Lit {
        self.data[i]
    }
    fn to_string(&self) -> String {
        let mut sb = String::new();
        sb.push('[');
        for y in &self.data {
            sb.push_str(&y.x.to_string());
            sb.push_str(", ");
        }
        sb.push(']');
        sb
    }
    fn get_data(&self) -> &Vec<Lit> {
        &self.data
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
