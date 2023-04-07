use std::env;
use std::rc::Rc;
use std::process;
use std::io;
use std::io::{stdout, Write};

use suiron::*;

fn query() {

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
} // query()


fn main() {

    query();

println!("***********************");
println!("Size of <Unifiable> is {}", std::mem::size_of::<Unifiable>());
println!("Size of <Box<Unifiable>> is {}", std::mem::size_of::<Box<Unifiable>>());
println!("Size of <Rc<Unifiable>> is {}", std::mem::size_of::<Rc<Unifiable>>());
println!("Size of SolutionNode is {}", std::mem::size_of::<SolutionNode>());
println!("***********************");


} //-------------------------------

/*
    Memory made safe,
    All our systems turn to Rust,
    Sayonara C.
*/