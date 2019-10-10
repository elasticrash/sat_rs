// Literals
use std::cmp::Ordering;
use std::ops::Not;

pub struct Lit {
    pub x: i32,
}

pub trait ILit {
    fn new(v: i32, sign: bool) -> Self;
    fn simple(v: i32) -> Self;
    fn to_string() -> str;
    fn get_hash_code() -> i32;
    fn equals(object: Lit) -> bool;
}

impl Not for Lit {
    type Output = Lit;
    fn not(self) -> Lit {
        let mut q: Lit = Lit { x: 0 };
        q.x = self.x ^ 1;
        return q;
    }
}

impl PartialOrd for Lit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

impl PartialEq for Lit {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}
