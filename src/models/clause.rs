use crate::models::lit::Lit;

#[derive(Clone)]
pub struct Clause {
    pub data: Vec<Lit>,
    pub is_learnt: bool,
    pub activity: f64,
}

pub trait IClause {
    fn new(learnt: bool, ps: &Vec<Lit>) -> Self;
    fn size(&self) -> i32;
    fn learnt(&self) -> bool;
    fn get_by_index(&self, i: usize) -> Lit;
    fn to_string(&self) -> String;
    fn get_data(&self) -> &Vec<Lit>;
}

impl IClause for Clause {
    fn new(_learnt: bool, _ps: &Vec<Lit>) -> Self {
        return Self {
            data: _ps.to_vec(),
            is_learnt: true,
            activity: 0.0,
        };
    }
    fn size(&self) -> i32 {
        return self.data.len() as i32;
    }
    fn learnt(&self) -> bool {
        return self.is_learnt;
    }
    fn get_by_index(&self, i: usize) -> Lit {
        return self.data[i];
    }
    fn to_string(&self) -> String {
        let mut sb = String::new();
        sb.push('[');
        for y in self.data.clone() {
            sb.push_str(&y.x.to_string());
            sb.push_str(&String::from(", ".to_string()));
        }
        sb.push(']');
        return sb;
    }
    fn get_data(&self) -> &Vec<Lit> {
        return &self.data;
    }
}

impl PartialEq for Clause {
    fn eq(&self, other: &Self) -> bool {
        if self.activity == other.activity {
            if self.data.len() == other.data.len() {
                if self.is_learnt == other.is_learnt {
                    let mut f: String = String::new();
                    for y in self.data.clone() {
                        f.push_str(&y.x.to_string())
                    }

                    let mut s: String = String::new();
                    for y in other.data.clone() {
                        s.push_str(&y.x.to_string())
                    }

                    if f == s {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}
