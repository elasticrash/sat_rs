use crate::models::clause::Clause;
use crate::models::lit::Lit;

/*_________________________________________________________________________________________________
|
|  enqueue : (p : Lit) (from : Clause*)  .  [bool]
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

pub fn enqueue(_fact: Lit, _from: Option<Clause>) -> bool {
    return true;
}
