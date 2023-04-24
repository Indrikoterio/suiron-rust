//! Module for benchmarking.
//!
//! This module performs a Q sort on a list of numbers, to measure
//! the speed of the inference engine.
//!
// Cleve Lendon 2023

use std::process;
use std::rc::Rc;

use super::goal::*;
use super::s_complex::*;
use super::solutions::*;
use super::rule_reader::*;
use super::knowledge_base::*;

pub fn benchmark() {

    let file_path = "./tests/qsort.txt";
    println!("Loading file: {}", file_path);

    let mut kb = KnowledgeBase::new();
    let result = load_kb_from_file(&mut kb, &file_path);
    match result {
        Some(err) => {
            println!("{}", err);
            process::exit(0);
        },
        None => {}, // All OK.
    }
    //print_kb(&kb); // For debugging.

    let input = "m";
    let query = parse_query(&input);
    match query {
        Ok(q) => {
            let sn = make_base_node(Rc::new(q), &kb); // solution node
            let result = solve(Rc::clone(&sn));
            print!("{} ", result);
        },
        Err(err) => { println!("{}", err); },
    } // match

}  // benchmark
