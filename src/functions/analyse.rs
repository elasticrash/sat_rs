use crate::models::clause::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::solverstate::*;
use std::cmp::max;

/*_________________________________________________________________________________________________
|
|  analyze : (confl : Clause*) (out_learnt : vec<Lit>&) (out_btlevel : int&)  .  [void]
|
|  Description:
|    Analyze conflict and produce a reason clause.
|
|    Pre-conditions:
|      * 'out_learnt' is assumed to be cleared.
|      * Current decision level must be greater than root level.
|
|    Post-conditions:
|      * 'out_learnt[0]' is the asserting literal at level 'out_btlevel'.
|
|  Effect:
|    Will undo part of the trail, upto but not beyond the assumption of the current decision level.
|________________________________________________________________________________________________@*/

pub fn analyze(
    mut confl: Option<Clause>,
    out_learnt: &mut Vec<Lit>,
    solver_state: &mut SolverState,
) -> i32 {
    let mut out_btlevel: i32 = 0;
    let mut seen: Vec<Lbool> = solver_state.analyze_seen.clone();
    let mut path_c: i32 = 0;
    let mut p: Lit = Lit::new(VAR_UNDEFINED, true);

    out_learnt.push(Lit::new(VAR_UNDEFINED, true)); // (leave room for the asserting literal)
    let mut index: i32 = (solver_state.trail.len() - 1) as i32;

    while {
        {
            let c: Clause = confl.clone().unwrap();

            if c.is_learnt {
                solver_state.cla_bump_activity(&mut c.clone());
            }

            let mut start: usize = 1;
            if p.x == VAR_UNDEFINED {
                start = 0;
            }
            for j in start..c.clone().data.len() {
                let q: Lit = c.data[j];
                if seen[var(&q) as usize] == Lbool::Undef0
                    && solver_state.level[var(&q) as usize] > 0
                {
                    solver_state.var_bump_activity(q);
                    seen[var(&q) as usize] = Lbool::True;
                    if solver_state.level[var(&q) as usize] == solver_state.decision_level() {
                        path_c += 1;
                    } else {
                        out_learnt.push(q);
                        out_btlevel = max(out_btlevel, solver_state.level[var(&q) as usize])
                    }
                }
            }
            while { seen[var(&solver_state.trail[index as usize]) as usize] == Lbool::Undef0 } {
                index -= 1;
                p = solver_state.trail[(index + 1) as usize];
                confl = solver_state.reason[var(&p) as usize].clone();
                seen[var(&p) as usize] = Lbool::Undef0;
                path_c -= 1;
            }
        }
        path_c > 0
    } {}
    out_learnt[0] = !p;

    {
        let mut i: usize = 1;
        let mut j: usize;

        if solver_state.expensive_ccmin {
            let mut min_level: u32 = 0;
            for y in (i as usize)..out_learnt.len() {
                i = y;
                min_level |= 1 << (solver_state.level[var(&out_learnt[i]) as usize] & 31);
            }
            solver_state.analyze_toclear.clear();
            i = 1;
            j = 1;
            for y in (i as usize)..out_learnt.len() {
                match solver_state.reason[var(&out_learnt[y]) as usize] {
                    None => {
                        j += 1;
                        out_learnt[j as usize] = out_learnt[y];
                    }
                    _ => {
                        if !analyze_removeable(out_learnt[y], min_level, solver_state) {
                            j += 1;
                            out_learnt[j as usize] = out_learnt[y];
                        }
                    }
                }
            }
        } else {
            solver_state.analyze_toclear.clear();
            i = 1;
            j = 1;
            let mut keep: bool = false;
            for y in (i as usize)..out_learnt.len() {
                match solver_state.reason[var(&out_learnt[y]) as usize] {
                    Some(ref p) => {
                        let c: Clause = p.clone();
                        for k in 1..c.data.len() {
                            if seen[var(&c.data[k]) as usize] == Lbool::Undef0
                                && solver_state.level[var(&c.data[k]) as usize] != 0
                            {
                                j += 1;
                                out_learnt[j as usize] = out_learnt[y];
                                keep = true;
                                break;
                            }
                        }
                    }
                    None => {
                        out_learnt[j as usize] = out_learnt[y];
                    }
                }

                if !keep {
                    solver_state.analyze_toclear.push(out_learnt[y]);
                }
            }
        }
        {
            for y in 0..out_learnt.len() {
                seen[var(&out_learnt[y]) as usize] = Lbool::Undef0;
            }

            for y in 0..solver_state.analyze_toclear.len() {
                seen[var(&solver_state.analyze_toclear[y]) as usize] = Lbool::Undef0;
            }
        }

        solver_state.solver_stats.max_literals += out_learnt.len() as f64;
        out_learnt.truncate((i - j) as usize);
        solver_state.solver_stats.tot_literals += out_learnt.len() as f64;
    }

    return out_btlevel;
}

fn analyze_removeable(_p: Lit, min_level: u32, solver_state: &mut SolverState) -> bool {
    solver_state.analyze_stack.clear();
    solver_state.analyze_stack.push(_p.clone());
    let top: i32 = solver_state.analyze_toclear.len() as i32;

    while solver_state.analyze_stack.len() > 0 {
        let c: Clause;
        if solver_state.analyze_stack.last() == None {
            match &solver_state.reason[var(&_p) as usize] {
                Some(clause) => {
                    c = clause.clone();
                    solver_state.analyze_stack.pop();
                    for i in 1..c.clone().data.len() {
                        let p: Lit = c.clone().data[i];
                        if solver_state.analyze_seen[var(&p) as usize] == Lbool::Undef0
                            && solver_state.level[var(&p) as usize] != 0
                        {
                            match &solver_state.reason[var(&p) as usize] {
                                None => {}
                                _ => {
                                    if ((1 << solver_state.level[var(&p) as usize] & 31)
                                        & min_level)
                                        != 0
                                    {
                                        solver_state.analyze_seen[var(&p) as usize] = Lbool::True;
                                        solver_state.analyze_stack.push(p);
                                        solver_state.analyze_toclear.push(p);
                                    } else {
                                        for j in top..solver_state.analyze_toclear.len() as i32 {
                                            solver_state.analyze_seen[var(&solver_state
                                                .analyze_toclear
                                                [j as usize])
                                                as usize] = Lbool::Undef0;
                                            solver_state.analyze_toclear.truncate(
                                                solver_state.analyze_toclear.len() - top as usize,
                                            );
                                            return false;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                None => {}
            }
        }
    }
    return true;
}
