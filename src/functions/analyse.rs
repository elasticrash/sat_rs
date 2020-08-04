use crate::models::clause::*;
use crate::models::lbool::*;
use crate::models::lit::*;
use crate::models::logger::*;
use crate::models::solverstate::*;
use std::cmp::max;

/*_________________________________________________________________________________________________
|
|  analyze
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
    reportf(
        "analyse".to_string(),
        file!(),
        line!(),
        solver_state.verbosity,
    );
    let mut out_btlevel: i32 = 0;
    let mut seen: Vec<Lbool> = solver_state.analyze_seen.clone();
    let mut path_c: i32 = 0;
    let mut p: Lit = Lit::new(VAR_UNDEFINED, true);

    out_learnt.push(Lit::empty()); // (leave room for the asserting literal)
    let mut index: i32 = (solver_state.trail.len() - 1) as i32;

    while {
        {
            assert!(confl != None);
            let c: Clause = confl.clone().unwrap();

            if c.is_learnt {
                solver_state.cla_bump_activity(&mut c.clone());
            }

            let mut start: usize;
            if p.x == VAR_UNDEFINED {
                start = 0;
            } else {
                start = 1;
            }

            for _y in start..c.clone().data.len() {
                let q: Lit = c.data[start];
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
                start += 1;
            }
            loop {
                if seen[var(&solver_state.trail[index as usize]) as usize] == Lbool::Undef0 {
                    index -= 1;
                } else {
                    index -= 1;
                    break;
                }
            }
            p = solver_state.trail[(index + 1) as usize];
            confl = solver_state.reason[var(&p) as usize].clone();
            seen[var(&p) as usize] = Lbool::Undef0;
            path_c -= 1;
        }
        path_c > 0
    } {}
    out_learnt[0] = !p;

    {
        let mut i: usize = 1;
        let mut j;

        if solver_state.expensive_ccmin {
            let mut min_level: u32 = 0;
            for _y in 1..out_learnt.len() {
                let v = var(&out_learnt[i]);
                let l = solver_state.level[v as usize];
                min_level |= 1 << (l & 31);
                i += 1;
            }
            solver_state.analyze_toclear.clear();
            i = 1;
            j = 1;
            for _y in 1..out_learnt.len() {
                match solver_state.reason[var(&out_learnt[i]) as usize] {
                    None => {
                        out_learnt[j as usize] = out_learnt[i];
                        j += 1;
                    }
                    _ => {
                        if !analyze_removeable(out_learnt[i], min_level, solver_state) {
                            out_learnt[j as usize] = out_learnt[i];
                            j += 1;
                        }
                    }
                }
                i += 1;
            }
        } else {
            solver_state.analyze_toclear.clear();
            i = 1;
            j = 1;
            let mut keep: bool = false;
            for _y in 1..out_learnt.len() {
                match solver_state.reason[var(&out_learnt[i]) as usize] {
                    Some(ref p) => {
                        let c: Clause = p.clone();
                        for k in 1..c.data.len() {
                            if seen[var(&c.data[k]) as usize] == Lbool::Undef0
                                && solver_state.level[var(&c.data[k]) as usize] != 0
                            {
                                j += 1;
                                out_learnt[j as usize] = out_learnt[i];
                                keep = true;
                                break;
                            }
                        }
                    }
                    None => {
                        out_learnt[j as usize] = out_learnt[i];
                    }
                }

                if !keep {
                    solver_state.analyze_toclear.push(out_learnt[i]);
                }
                i += 1;
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
        out_learnt.truncate(out_learnt.len() - (i - j) as usize);
        solver_state.solver_stats.tot_literals += out_learnt.len() as f64;
    }

    return out_btlevel;
}

fn analyze_removeable(_p: Lit, min_level: u32, solver_state: &mut SolverState) -> bool {
    reportf(
        "analyze removeable".to_string(),
        file!(),
        line!(),
        solver_state.verbosity,
    );
    assert!(solver_state.reason[var(&_p) as usize] != None);

    solver_state.analyze_stack.clear();
    solver_state.analyze_stack.push(_p.clone());
    let top: i32 = solver_state.analyze_toclear.len() as i32;

    while solver_state.analyze_stack.len() > 0 {
        let p_last = &solver_state.analyze_stack.last();
        assert!(solver_state.reason[var(&p_last.unwrap()) as usize] != None);
        let c: Clause;
        match &solver_state.reason[var(&p_last.unwrap()) as usize] {
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
                                if ((1 << solver_state.level[var(&p) as usize] & 31) & min_level)
                                    != 0
                                {
                                    solver_state.analyze_seen[var(&p) as usize] = Lbool::True;
                                    solver_state.analyze_stack.push(p);
                                    solver_state.analyze_toclear.push(p);
                                } else {
                                    for j in top..solver_state.analyze_toclear.len() as i32 {
                                        solver_state.analyze_seen[var(
                                            &solver_state.analyze_toclear[j as usize]
                                        )
                                            as usize] = Lbool::Undef0;
                                        solver_state.analyze_toclear.truncate(top as usize);
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
    solver_state.analyze_toclear.push(_p);
    return true;
}
