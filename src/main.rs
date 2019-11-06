mod functions;
mod models;
use crate::functions::new_clause::*;
use crate::functions::solve::*;
use crate::functions::stats::*;
use crate::models::lit::*;
use crate::models::logger::*;
use crate::models::solverstate::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    debug("=========================================".to_string());
    let _args: Vec<String> = env::args().collect();

    let mut expect: bool = false;
    let mut expect_res: bool = false;

    // let mut pos: usize = 1;

    // if &args[pos] == "-s" {
    //     expect = true;
    //     expect_res = true;
    //     pos += 1;
    // }

    // if &args[pos] == "-u" {
    //     expect = true;
    //     expect_res = false;
    //     pos += 1;
    // }
    let mut file = File::open("./input.txt").unwrap();
    let mut buffer = String::new();

    let mut lits: Vec<Lit> = Vec::new();
    let mut s: SolverState = SolverState::new();

    file.read_to_string(&mut buffer).unwrap();

    let all_chars: Vec<char> = buffer.chars().collect();

    let mut until: i32 = 0;
    let mut w: (String, String);
    lits.clear();
    while {
        w = read_word(all_chars.clone(), until);
        until += w.0.clone().len() as i32;
        until += w.1.clone().len() as i32;
        w.0.len() != 0 && w.1.len() != 0
    } {
        let word = w.0;
        if word == "%" {
            break;
        }
        let parsed_lit: i32 = word.parse::<i32>().unwrap();

        if parsed_lit != 0 {
            let var = parsed_lit.abs() - 1;
            while var >= s.n_vars() {
                new_var(&mut s);
            }
            let solver_lit;
            if parsed_lit > 0 {
                solver_lit = Lit::simple(var);
            } else {
                solver_lit = !Lit::simple(var);
            }

            lits.push(solver_lit);
        }
    }

    s.add_clause(&mut lits);

    if expect {
        s.verbosity = 0;
        solve_no_assumptions(&mut s);
        if s.ok == expect_res {
            reportf(".".to_string());
        } else {
            reportf("problem:".to_string());
        }
    } else {
        s.verbosity = 1;
        solve_no_assumptions(&mut s);
        let mut result: String = String::new();
        if s.ok {
            result.push_str("SATISFIABLE");
        } else {
            result.push_str("UNSATISFIABLE");
        }
        reportf(result);
        print_stats(s.solver_stats);
    }
}

fn read_word(chars: Vec<char>, from: i32) -> (String, String) {
    let mut sb = String::new();
    let mut fake = String::new();
    let mut i: i32 = from;
    for _y in from..chars.len() as i32 {
        let mut ch = chars[i as usize];

        if ch == ' ' || ch == '\n' || ch == '\r' {
            fake.push_str(&ch.to_string());
            if sb.len() > 0 {
                break;
            }
        } else {
            if ch == 'p' || ch == 'c' {
                fake.push_str(&ch.to_string());
                while ch != '\n' {
                    i += 1;
                    ch = chars[i as usize];
                    fake.push_str(&ch.to_string());
                }

                if sb.len() > 0 {
                    break;
                }
            } else {
                sb.push_str(&ch.to_string());
            }
        }
        i += 1;
    }
    return (sb, fake);
}
