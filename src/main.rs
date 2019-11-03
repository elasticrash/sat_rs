mod functions;
mod models;
use crate::models::lit::*;
use crate::models::solverstate::*;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {
    // let args: Vec<String> = env::args().collect();

    let mut expect: bool = false;
    let mut expect_res: bool = false;

    let mut pos: usize = 1;

    /*if &args[pos] == "-s" {
        expect = true;
        expect_res = true;
        pos += 1;
    }

    if &args[pos] == "-u" {
        expect = true;
        expect_res = false;
        pos += 1;
    }*/
    let mut file = File::open("./input.txt").unwrap();
    let mut buffer = String::new();

    let mut lits: Vec<Lit> = Vec::new();
    let s: SolverState = SolverState::new();

    file.read_to_string(&mut buffer).unwrap();

    let all_chars: Vec<char> = buffer.chars().collect();

    let mut until: i32 = 0;
    let mut w: Option<String>;
    lits.clear();
    while {
        w = ReadWord(all_chars.clone(), until);
        if w != None {
            until += w.clone().unwrap().len() as i32;
        }
        w != None
    } {
        let word = w.unwrap();
        if word == "%" {
            break;
        }
        let parsed_lit: i32 = word.parse::<i32>().unwrap();

        if parsed_lit == 0 {
            break;
        }
    }
}

fn ReadWord(a: Vec<char>, b: i32) -> Option<String> {
    let sb_array: Vec<String> = Vec::new();

    // while true {}

    return None;
}
