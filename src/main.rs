mod functions;
mod models;
use crate::functions::new_clause::*;
use crate::functions::solve::*;
use crate::models::lit::*;
use crate::models::solverstate::*;
use simplelog::*;
use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate log;

fn main() {
    let _args: Vec<String> = env::args().collect();

    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            File::create("sat.log").unwrap(),
        ),
    ])
    .unwrap();

    let mut file = File::open("./input.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let state = process_problem(&buffer);

    let mut result: String = String::new();
    if state.ok {
        result.push_str("SATISFIABLE");
    } else {
        result.push_str("UNSATISFIABLE");
    }
    info!("{}|{}|{}|{}", result, file!(), line!(), 2);
    println!("{}", state.solver_stats);
}

fn process_problem(buffer: &str) -> SolverState {
    let mut state: SolverState = SolverState::new();

    let problem_lines = regex::Regex::new(r"\n|\r").unwrap();
    let mut lits: Vec<Lit> = Vec::new();
    let mut last_var_zero = false;
    for part in problem_lines.split(buffer) {
        if last_var_zero {
            lits = Vec::new();
            last_var_zero = false;
        }
        if !part.is_empty() && !part.starts_with('c') && !part.starts_with('p') {
            let line_vars = regex::Regex::new(r" ").unwrap();
            for var in line_vars.split(part) {
                if !var.is_empty() {
                    let parsed_lit: i32 = var.parse::<i32>().unwrap();
                    let zero_based_abs_var = parsed_lit.abs() - 1;
                    while zero_based_abs_var >= state.n_vars() {
                        state.new_var();
                    }
                    let solver_lit;

                    match parsed_lit.cmp(&0) {
                        Ordering::Greater => {
                            solver_lit = Lit::simple(zero_based_abs_var);
                            lits.push(solver_lit);
                        }
                        Ordering::Less => {
                            solver_lit = !Lit::simple(zero_based_abs_var);
                            lits.push(solver_lit);
                        }
                        Ordering::Equal => {
                            last_var_zero = true;
                        }
                    }
                }
            }
        }
        if last_var_zero && !lits.is_empty() {
            state.add_clause(&mut lits);
        }
    }

    state.solve_no_assumptions();
    state
}

#[test]
fn control_problem() {
    let problem = r#"
c
p cnf 3 2
1 -3 0
2 3 -1 0
"#;
    let state = process_problem(problem);

    assert_eq!(state.solver_stats.starts, 1.);
    assert_eq!(state.solver_stats.conflicts, 0.);
    assert_eq!(state.solver_stats.decisions, 3.);
    assert_eq!(state.solver_stats.propagations, 3.);
    assert_eq!(state.solver_stats.tot_literals, 0.);
}

#[test]
fn aim_50_problem() {
    let problem = r#"
c SOURCE: Kazuo Iwama, Eiji Miyano (miyano@cscu.kyushu-u.ac.jp),
c  and Yuichi Asahiro
c
c DESCRIPTION: Artifical instances from generator by source.  Generators
c  and more information in sat/contributed/iwama.
c
c NOTE: Satisfiable
c
p cnf 50 80
16 17 30 0
-17 22 30 0
-17 -22 30 0
16 -30 47 0
16 -30 -47 0
-16 -21 31 0
-16 -21 -31 0
-16 21 -28 0
-13 21 28 0
13 -16 18 0
13 -18 -38 0
13 -18 -31 0
31 38 44 0
-8 31 -44 0
8 -12 -44 0
8 12 -27 0
12 27 40 0
-4 27 -40 0
12 23 -40 0
-3 4 -23 0
3 -23 -49 0
3 -13 -49 0
-23 -26 49 0
12 -34 49 0
-12 26 -34 0
19 34 36 0
-19 26 36 0
-30 34 -36 0
24 34 -36 0
-24 -36 43 0
6 42 -43 0
-24 42 -43 0
-5 -24 -42 0
5 20 -42 0
5 -7 -20 0
4 7 10 0
-4 10 -20 0
7 -10 -41 0
-10 41 46 0
-33 41 -46 0
33 -37 -46 0
32 33 37 0
6 -32 37 0
-6 25 -32 0
-6 -25 -48 0
-9 28 48 0
-9 -25 -28 0
19 -25 48 0
2 9 -19 0
-2 -19 35 0
-2 22 -35 0
-22 -35 50 0
-17 -35 -50 0
-29 -35 -50 0
-1 29 -50 0
1 11 29 0
-11 17 -45 0
-11 39 45 0
-26 39 45 0
-3 -26 45 0
-11 15 -39 0
14 -15 -39 0
14 -15 -45 0
14 -15 -27 0
-14 -15 47 0
17 17 40 0
1 -29 -31 0
-7 32 38 0
-14 -33 -47 0
-1 2 -8 0
35 43 44 0
21 21 24 0
20 29 -48 0
23 35 -37 0
2 18 -33 0
15 25 -45 0
9 14 -38 0
-5 11 50 0
-3 -13 46 0
-13 -41 43 0
"#;
    let state = process_problem(problem);

    assert_eq!(state.solver_stats.starts, 1.);
    assert_eq!(state.solver_stats.conflicts, 11.);
    assert_eq!(state.solver_stats.decisions, 27.);
    assert_eq!(state.solver_stats.propagations, 168.);
    assert_eq!(state.solver_stats.tot_literals, 31.);
}

#[test]
fn quinn_problem() {
    let problem = r#"
c
p cnf 16 18
1 2  0
-2 -4  0
3 4  0
-4 -5  0
5 -6  0
6 -7  0
6 7  0
7 -16  0
8 -9  0
-8 -14  0
9 10  0
9 -10  0
-10 -11  0
10 12  0
11 12  0
13 14  0
14 -15  0
15 16  0
"#;
    let state = process_problem(problem);

    assert_eq!(state.solver_stats.starts, 1.);
    assert_eq!(state.solver_stats.conflicts, 1.);
    assert_eq!(state.solver_stats.decisions, 6.);
    assert_eq!(state.solver_stats.propagations, 25.);
    assert_eq!(state.solver_stats.tot_literals, 1.);
}

#[test]
fn aim_100_problem() {
    let problem = r#"
c SOURCE: Kazuo Iwama, Eiji Miyano (miyano@cscu.kyushu-u.ac.jp),
c          and Yuichi Asahiro
c
c DESCRIPTION: Artifical instances from generator by source.  Generators
c              and more information in sat/contributed/iwama.
c
c NOTE: Not Satisfiable
c
p cnf 100 160
16 30 95 0
-16 30 95 0
-30 35 78 0
-30 -78 85 0
-78 -85 95 0
8 55 100 0
8 55 -95 0
9 52 100 0
9 73 -100 0
-8 -9 52 0
38 66 83 0
-38 83 87 0
-52 83 -87 0
66 74 -83 0
-52 -66 89 0
-52 73 -89 0
-52 73 -74 0
-8 -73 -95 0
40 -55 90 0
-40 -55 90 0
25 35 82 0
-25 82 -90 0
-55 -82 -90 0
11 75 84 0
11 -75 96 0
23 -75 -96 0
-11 23 -35 0
-23 29 65 0
29 -35 -65 0
-23 -29 84 0
-35 54 70 0
-54 70 77 0
19 -77 -84 0
-19 -54 70 0
22 68 81 0
-22 48 81 0
-22 -48 93 0
3 -48 -93 0
7 18 -81 0
-7 56 -81 0
3 18 -56 0
-18 47 68 0
-18 -47 -81 0
-3 68 77 0
-3 -77 -84 0
19 -68 -70 0
-19 -68 74 0
-68 -70 -74 0
54 61 -62 0
50 53 -62 0
-50 61 -62 0
-27 56 93 0
4 14 76 0
4 -76 96 0
-4 14 80 0
-14 -68 80 0
-10 -39 -89 0
1 49 -81 0
1 26 -49 0
17 -26 -49 0
-1 17 -40 0
16 51 -89 0
-9 57 60 0
12 45 -51 0
2 12 69 0
2 -12 40 0
-12 -51 69 0
-33 60 -98 0
5 -32 -66 0
2 -47 -100 0
-42 64 83 0
20 -42 -64 0
20 -48 98 0
-20 50 98 0
-32 -50 98 0
-24 37 -73 0
-24 -37 -100 0
-57 71 81 0
-37 40 -91 0
31 42 81 0
-31 42 72 0
-31 42 -72 0
7 -19 25 0
-1 -25 -94 0
-15 -44 79 0
-6 31 46 0
-39 41 88 0
28 -39 43 0
28 -43 -88 0
-4 -28 -88 0
-30 -39 -41 0
-29 33 88 0
-16 21 94 0
-10 26 62 0
-11 -64 86 0
-6 -41 76 0
38 -46 93 0
26 -37 94 0
-26 53 -79 0
78 87 -94 0
65 76 -87 0
23 51 -62 0
-11 -36 57 0
41 59 -65 0
-56 72 -91 0
13 -20 -46 0
-13 15 79 0
-17 47 -60 0
-13 -44 99 0
-7 -38 67 0
37 -49 62 0
-14 -17 -79 0
-13 -15 -22 0
32 -33 -34 0
24 45 48 0
21 24 -48 0
-36 64 -85 0
10 -61 67 0
-5 44 59 0
-80 -85 -99 0
6 37 -97 0
-21 -34 64 0
-5 44 46 0
58 -76 97 0
-21 -36 75 0
-15 58 -59 0
-58 -76 -99 0
-2 15 33 0
-26 34 -57 0
-18 -82 -92 0
27 -80 -97 0
6 32 63 0
-34 -86 92 0
13 -61 97 0
-28 43 -98 0
5 39 -86 0
39 -45 92 0
27 -43 97 0
13 -58 -86 0
-28 -67 -93 0
-69 85 99 0
42 71 -72 0
10 -27 -63 0
-59 63 -83 0
36 86 -96 0
-2 36 75 0
-59 -71 89 0
36 -67 91 0
36 -60 63 0
-63 91 -93 0
25 87 92 0
-21 49 -71 0
-2 10 22 0
6 -18 41 0
6 71 -92 0
-53 -69 -71 0
-2 -53 -58 0
43 -45 -96 0
34 -45 -69 0
63 -86 -98 0
"#;
    let state = process_problem(problem);

    assert_eq!(state.solver_stats.starts, 1.);
    assert_eq!(state.solver_stats.conflicts, 30.);
    assert_eq!(state.solver_stats.decisions, 147.);
    assert_eq!(state.solver_stats.propagations, 402.);
    assert_eq!(state.solver_stats.tot_literals, 77.);
}
