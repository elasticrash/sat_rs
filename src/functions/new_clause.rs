use crate::models::lit::Lit;

pub fn new_clause(_ps: Vec<Lit>, _learnt:bool){
    
}

pub fn basic_clause_simplification(_ps:Vec<Lit>, _copy: bool) -> Vec<Lit>{
    let qs = Vec::new();
    return qs;
}

pub fn reorder_by_level(_ps: Vec<Lit>){

}

fn new_clause_pr(_ps: Vec<Lit>, _learnt:bool, _theory_clause:bool, _copy:bool){

}

pub fn remove(){}
pub fn simplify(){}
pub fn remove_watch(){}
pub fn new_var(){}