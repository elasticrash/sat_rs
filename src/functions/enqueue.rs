use crate::models::clause::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::logger::*;
use crate::models::solverstate::*;

/*_________________________________________________________________________________________________
|
|  enqueue
|
|  Description:
|    Puts a new fact on the propagation queue as well as immediately updating the variable's value.
|    Should a conflict arise, FALSE is returned.
|
|  Input:
|    p    - The fact to enqueue
|    from - [Optional] Fact propagated from this (currently) unit clause. Stored in 'reason[]'.
|           Default value is null (no reason).
|
|  Output:
|    TRUE if fact was enqueued without conflict, FALSE otherwise.
|________________________________________________________________________________________________@*/
pub trait NQueue {
    fn enqueue(&mut self, p: &Lit, from: Option<Clause>) -> bool;
    fn internal_enqueue(&mut self, _fact: &Lit) -> bool;
}

impl NQueue for SolverState {
    fn enqueue(&mut self, p: &Lit, from: Option<Clause>) -> bool {
        reportf("enqueue".to_string(), file!(), line!(), self.verbosity);

        if !is_undefined(value_by_lit(*p, self)) {
            return value_by_lit(*p, self) != L_FALSE;
        } else {
            let x: usize = var(&p) as usize;
            self.assigns[x] = to_bool(!sign(p));
            self.level[x] = self.decision_level();
            self.trail_pos[x] = self.trail.len() as i32;
            self.reason[x] = from;
            self.trail.push(*p);
            return true;
        }
    }

    fn internal_enqueue(&mut self, _fact: &Lit) -> bool {
        reportf(
            "internal_enqueue".to_string(),
            file!(),
            line!(),
            self.verbosity,
        );

        return self.enqueue(&_fact, None);
    }
}
