mod functions;
mod models;
use crate::functions::new_clause::*;
use crate::functions::solve::*;
use crate::functions::stats::*;
use crate::models::input_arguments::*;
use crate::models::lit::*;
use crate::models::logger::*;
use crate::models::solverstate::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let _args: Vec<String> = env::args().collect();

    let arguments: InputArguments = read_input_arguments(_args);
    let mut file = File::open("./input.txt").unwrap();
    let mut buffer = String::new();
    let mut state: SolverState = SolverState::new();
    state.verbosity = arguments.verbosity as i32;

    file.read_to_string(&mut buffer).unwrap();

    let problem_lines = regex::Regex::new(r" 0|\n|\r").unwrap();
    for part in problem_lines.split(&buffer) {
        let mut lits: Vec<Lit> = Vec::new();
        if part != "" && !part.starts_with('c') && !part.starts_with('p') {
            let line_vars = regex::Regex::new(r" ").unwrap();
            for var in line_vars.split(&part) {
                if var != "" {
                    let parsed_lit: i32 = var.parse::<i32>().unwrap();
                    let zero_based_abs_var = parsed_lit.abs() - 1;
                    while zero_based_abs_var >= state.n_vars() {
                        new_var(&mut state);
                    }
                    let solver_lit;
                    if parsed_lit > 0 {
                        solver_lit = Lit::simple(zero_based_abs_var);
                    } else {
                        solver_lit = !Lit::simple(zero_based_abs_var);
                    }
                    lits.push(solver_lit);
                }
            }
        }
        if lits.len() > 0 {
            state.add_clause(&mut lits);
        }
    }

    if state.verbosity == 0 {
        solve_no_assumptions(&mut state);
    } else {
        solve_no_assumptions(&mut state);
        let mut result: String = String::new();
        if state.ok {
            result.push_str("SATISFIABLE");
        } else {
            result.push_str("UNSATISFIABLE");
        }
        reportf(result, file!(), line!(), 2);
        print_stats(state.solver_stats);
    }
}

fn read_input_arguments(_args: Vec<String>) -> InputArguments {
    println!("{:?}", _args);
    let mut arguments = InputArguments {
        pre: "".to_string(),
        grow: 1,
        polarity_mode: "true".to_string(),
        decay: 0,
        rnd_freq: 0,
        verbosity: 1,
    };

    for arg in _args {
        let arg_value: Vec<&str> = arg.split('=').collect();

        if arg.starts_with("-pre") {
            arguments.pre = String::from(arg_value[1]);
        }

        if arg.starts_with("-grow") {
            arguments.grow = String::from(arg_value[1]).parse::<i32>().unwrap();
        }

        if arg.starts_with("-polarity_mode") {
            arguments.polarity_mode = String::from(arg_value[1]);
        }

        if arg.starts_with("-decay") {
            arguments.decay = String::from(arg_value[1]).parse::<i32>().unwrap();
        }

        if arg.starts_with("-rnd_freq") {
            arguments.rnd_freq = String::from(arg_value[1]).parse::<i32>().unwrap();
        }

        if arg.starts_with("-verbosity") {
            arguments.verbosity = String::from(arg_value[1]).parse::<i16>().unwrap();
        }
    }
    return arguments;
}
