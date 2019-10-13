// Literals
use std::cmp::Ordering;
use std::ops::Not;

pub struct Lit {
    pub x: i32,
}

pub trait ILit {
    fn new(v: i32, sign: bool) -> Self;
    fn simple(v: i32) -> Self;
    fn to_string(lit: &Lit) -> String;
    fn get_hash_code(lit: &Lit) -> i32;
    fn equals(&self, lit: &Lit) -> bool;
}

impl ILit for Lit {
    fn new(v: i32, sign: bool) -> Self {
        Self {
            x: v + v + (if sign { 1 } else { 0 }),
        }
    }
    fn simple(v: i32) -> Self {
        Self { x: v + v }
    }
    fn to_string(lit: &Lit) -> String {
        let temp_string = var(&lit).to_string();
        return (if sign(&lit) { "-" } else { "" }).to_owned() + "x" + &temp_string;
    }
    fn get_hash_code(lit: &Lit) -> i32 {
        return lit.x;
    }
    fn equals(&self, lit: &Lit) -> bool {
        if self.x == lit.x {
            return true;
        }
        return false;
    }
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
        return self.x == other.x;
    }
}

pub fn sign(lit: &Lit) -> bool {
    return (&lit.x & 1) != 0;
}

pub fn var(lit: &Lit) -> i32 {
    return lit.x >> 1;
}

pub fn index(lit: &Lit) -> i32 {
    return lit.x;
}
