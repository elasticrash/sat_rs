use crate::models::lit::*;
use std::collections::HashMap;

pub fn new_clause(_ps: Vec<Lit>, _learnt: bool) {}

pub fn basic_clause_simplification(_ps: Vec<Lit>, _copy: bool) -> Option<Vec<Lit>> {
    let mut qs: Vec<Lit>;

    if _copy {
        qs = Vec::new();
    } else {
        qs = _ps;
    }

    let mut dict: HashMap<i32, Lit> = HashMap::new();
    let ptr: i32 = 0;

    for i in 0..qs.len() {
        let l: Lit = qs[i];
        let v: i32 = var(&l);
        let other = Some(dict.get(&v).unwrap());
        if other != None {
            if other.unwrap() == &l {
            } else {
                return None;
            }
        } else {
            dict.insert(v, l);
            qs.push(l);
        }
    }
    qs.truncate(ptr as usize);

    return Some(qs);
}

pub fn reorder_by_level(_ps: Vec<Lit>) {}

fn new_clause_pr(_ps: Vec<Lit>, _learnt: bool, _theory_clause: bool, _copy: bool) {}

pub fn remove() {}
pub fn simplify() {}
pub fn remove_watch() {}
pub fn new_var() {}
