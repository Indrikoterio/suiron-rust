//! This crate demonstrates the use of the Suiron library crate.
//!
//! Suiron implements a source code parser, a knowledge base, and an inference engine.<br>
//! Query uses Suiron to load a program and evaluate queries to the knowledge base.
//!
use std::env;
use std::rc::Rc;
use std::process;
use std::io;
use std::io::{stdout, Write};

use suiron::*;

/// The `query` binary loads a Suiron program and prompts for queries.
///
/// In addition to this program, there is a demo program located
/// [here](../../../suiron_demo/target/doc/suiron_demo/index.html).
///
/// # Usage
/// Open a command line interface, and navigate to the `suiron-rust` folder.
///
/// Enter the command `cargo run`, followed two dashes and the path of a source file.
/// The source file shown here is: [tests/kings.txt](../../../tests/kings.txt)
/// ```
/// cargo run -- tests/kings.txt
/// ```
/// The program will prompt for a query:
/// ```
/// Loading file: tests/kings.txt
/// ?-
/// ```
/// Enter a query, such as:
/// ```
/// ?- grandfather($X, Harold).
/// ```
/// The program should respond with:
/// ```
/// $X = Godwin
/// No more.
/// ?-
/// ```
///
/// # Tutorial
/// An on-line tutorial can be found [here](https://klivo.net/suiron/).
///
fn main() {

    let args: Vec<String> = env::args().collect();

    // Read file, if a file name was given.
    if args.len() > 1 {

        let file_path = &args[1];
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

        loop {

            // Get a query from stdin.
            print!("?- ");
            let _ = stdout().flush();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Enter a query.");

            input = input.trim().to_string();
            if input.len() == 0 { break; }

            let query = parse_query(&input);
            match query {
                Ok(q) => {
                    let sn = make_base_node(Rc::new(q), &kb); // solution node
                    loop {
                        let result = solve(Rc::clone(&sn));
                        print!("{} ", result);
                        let _ = stdout().flush();
                        io::stdin().read_line(&mut input).expect("");
                        if result.eq("No more.") { break; }
                    } // loop
                },
                Err(err) => { println!("{}", err); },
            } // match
        } // loop
    } // if args.len() > 1
    else {
        println!("\nSuiron - A fast inference engine, by Cleve Lendon, 2023\n");
        println!("Usage:");
        println!("cargo run -- tests/kings.txt\n");
    }

} // main()

/*
    Memory made safe,
    All our systems turn to Rust,
    Sayonara C.
*/