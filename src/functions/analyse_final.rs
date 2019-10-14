use crate::models::clause::Clause;

/*_________________________________________________________________________________________________
|
|  analyzeFinal : (confl : Clause*) (skip_first : bool)  .  [void]
|
|  Description:
|    Specialized analysis procedure to express the final conflict in terms of assumptions.
|    'root_level' is allowed to point beyond end of trace (useful if called after conflict while
|    making assumptions). If 'skip_first' is TRUE, the first literal of 'confl' is  ignored (needed
|    if conflict arose before search even started).
|________________________________________________________________________________________________@*/

pub fn analyse_final(_confl: Clause, _skip_first: bool) {}
