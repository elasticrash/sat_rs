// Literals
use std::cmp::Ordering;
use std::ops::Not;

pub static VAR_UNDEFINED: i32 = -1;

#[derive(Copy, Clone, Debug)]
pub struct Lit {
    pub x: i32,
}

pub trait ILit {
    fn new(v: i32, sign: bool) -> Self;
    fn simple(v: i32) -> Self;
    fn empty() -> Self;
    fn to_string(lit: &Lit) -> String;
    fn get_hash_code(lit: &Lit) -> i32;
    fn equals(&self, lit: &Lit) -> bool;
    fn undefined() -> Self;
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
    fn empty() -> Self {
        Self { x: 0 }
    }
    fn to_string(lit: &Lit) -> String {
        let temp_string = var(lit).to_string();
        (if sign(lit) { "-" } else { "" }).to_owned() + "x" + &temp_string
    }
    fn get_hash_code(lit: &Lit) -> i32 {
        lit.x
    }
    fn equals(&self, lit: &Lit) -> bool {
        match self.x == lit.x {
            true => true,
            false => false,
        }
    }
    fn undefined() -> Self {
        Self::new(VAR_UNDEFINED, false)
    }
}

// using the Not operator as a bitwise operator
// might need changing in the future as it is
// slightly confusing
impl Not for Lit {
    type Output = Lit;
    fn not(self) -> Lit {
        let mut q: Lit = Lit { x: 0 };
        q.x = self.x ^ 1;
        q
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

pub fn sign(lit: &Lit) -> bool {
    let t = &lit.x & 1;
    t != 0
}

pub fn var(lit: &Lit) -> i32 {
    lit.x >> 1
}

/*pub fn swap(i: i32, j: i32, data: &mut Vec<Lit>) {
    assert!((i as usize) < data.len() && (j as usize) < data.len());
    data.swap(i as usize, j as usize);

    //let tmp = data[i as usize];
    //data[i as usize] = data[j as usize];
    //data[j as usize] = tmp;
}*/
