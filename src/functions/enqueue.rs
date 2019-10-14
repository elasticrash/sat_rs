use crate::models::clause::Clause;
use crate::models::lit::Lit;

pub fn enqueue(_fact: Lit, _from: Option<Clause>) -> bool {
    return true;
}
