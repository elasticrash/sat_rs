/*_________________________________________________________________________________________________
|
|  propagate : [void]  .  [Clause*]
|
|  Description:
|    Propagates all enqueued facts. If a conflict arises, the conflicting clause is returned,
|    otherwise null. NOTE! This method has been optimized for speed rather than readability.
|
|    Post-conditions:
|      * the propagation queue is empty, even if there was a conflict.
|________________________________________________________________________________________________@*/

pub fn propagate() {}
