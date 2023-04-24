//! # Suiron
//!
//! Suiron is a fast inference engine.
//! Its fact/rule definition language is similar to Prolog, but there are some differences.
//!
//! To understand how to use Suiron, a basic understanding of
//! [Prolog](https://en.wikipedia.org/wiki/Prolog) is required.
//! There is an online test site and tutorial at:
//! [klivo.net/suiron](https://klivo.net/suiron)
//!
//! ## Briefly
//!
//! An inference engines analyzes facts and rules which are stored in a knowledge base.
//! Suiron has a parser which loads these facts and rules from a text-format source file.
//!
//! Below is an example of a fact, which means "June is the mother of Theodore":
//!
//!```bash
//! mother(June, Theodore).
//!```
//!
//! Here we see the main difference between Suiron and Prolog.
//! In Prolog, lower case words are 'atoms' (that is, string constants) and upper case words are variables.
//! In Suiron, atoms can be lower case or upper case. Thus 'mother', 'June' and 'Theodore' are all atoms.
//! Suiron's atoms can even contain spaces.
//!
//!```bash
//! mother(June, The Beaver).
//!```
//!
//! Suiron's variables are defined by putting a dollar sign in front of the variable name,
//! for example, $Child. A query to determine June's children would be written:
//!
//!```bash
//! mother(June, $Child).
//!```
//!
//! Please refer to [LogicVar](../suiron/unifiable/enum.Unifiable.html#variant.LogicVar).
//!
//! The anonymous variable must also begin with a dollar sign: $\_ .
//! A simple underscore '\_' is treated as an atom.
//! Below is an example of a rule which contains an anonymous variable:
//!
//!```bash
//! voter($P) :- $P = person($_, $Age), $Age >= 18.
//!```
//!
//!<hr>
//!
//! Facts and rules can also be created dynamically within a Rust application program.
//! The fact mother(June, Theodore) could be created by calling the function parse_complex().
//!
//!```bash
//! let fact = parse_complex("mother(June, Theodore).");
//!```
//!
//! Please refer to [SComplex](../suiron/unifiable/enum.Unifiable.html#variant.SComplex).
//!
//! The query mother(June, $Child) could be created in Go as follows:
//!
//!```bash
//! let mother = Atom("mother");
//! let June   = Atom("June");
//! let child  = LogicVar("$Child");
//! let query  = MakeGoal(mother, June, child);
//!```
//!
//! Please refer to [variable.rs](suiron/variable.rs) and [goal.rs](suiron/goal.rs) for more details.
//!
//! Suiron also supports integer and floating point numbers, which are implemented as 64-bit ints and floats.
//! These are parsed by Go's strconv package:
//!
//!```bash
//!    f, err := strconv.ParseFloat(str, 64)
//!    i, err := strconv.ParseInt(str, 10, 64)
//!```
//!
//! If a Float and an Integer are compared, the Integer will be converted to a Float for the comparison.
//!
//! Please refer to [constants.rs](suiron/constants.rs).
//!
//! Of course, Suiron supports linked lists, which work the same way as Prolog lists.
//! A linked list can be loaded from a file:
//!
//!```bash
//!   ..., [a, b, c, d] = [$Head | $Tail], ...
//!```
//!
//! or created dynamically:
//!
//!```bash
//! let X = ParseLinkedList("[a, b, c, d]");
//! let Y = MakeLinkedList(true, $Head, $Tail);
//!```
//!
//! Please refer to [linkedlist.rs](suiron/linkedlist.rs).
//!
//! ## Requirements
//!
//! Suiron was developed and tested with Rust/Cargo version 1.65.0.
//!
//! [https://www.rust-lang.org/](https://www.rust-lang.org/)
//!
//! ## Cloning
//!
//! To clone the repository, run the following command in a terminal window:
//!
//!```bash
//! git clone git@github.com:Indrikoterio/suiron-rust.git
//!```
//!
//!The repository has three folders:
//!
//!```bash
//! suiron/suiron
//! suiron/test
//! suiron/demo
//!```
//!
//! The code for the inference engine itself is in the subfolder /suiron.
//!
//! The subfolder /test contains Go programs which test the basic functionality of Suiron.
//!
//! The subfolder /demo contains a simple demo program which parses English sentences.
//!
//! ## Usage
//!
//! In the top folder is a program called 'query', which loads facts and rules
//! from a file, and allows the user to query the knowledge base.
//! Query can be run in a terminal window as follows:
//!
//!```bash
//! ./query test/kings.txt
//!```
//!
//! The user will be prompted for a query with this prompt: ?-
//!
//! The query below will print out all father/child relationships.
//!
//!```bash
//! ?- father($F, $C).
//!```
//!
//! After typing enter, the program will print out solutions, one after each press
//! of Enter, until there are no more solutions, as indicated by 'No'.
//!
//!```bash
//! ./query. test/kings.txt
//! ?- father($F, $C).
//! $F = Godwin, $C = Harold II
//! $F = Godwin, $C = Tostig
//! $F = Godwin, $C = Edith
//! $F = Tostig, $C = Skule
//! $F = Harold II, $C = Harold
//! No
//! ?-
//!```
//!
//! To use Suiron in your own project, copy the subfolder 'suiron' to your project
//! folder. You will have to include:
//!
//!```bash
//! import (
//!    . "github.com/indrikoterio/suiron/suiron"
//! )
//!```
//!
//! in your source file.
//!
//! The program [parse_demo.go](demo/parse_demo.go) demonstrates how to set up
//! a knowledge base and make queries.
//! If you intend to incorporate Suiron into your own project, this is a good
//! reference. There are detailed comments in the header.
//!
//! To run parse_demo, move to the demo folder and execute the batch file 'run'.
//!
//!```bash
//! cd demo
//! ./run
//!```
//!
//! Suiron doesn't have a lot of built-in predicates, but it does have: [append.go](suiron/append.go),
//! [functor.go](suiron/functor.go), [print.go](suiron/print.go), [new_line.go](suiron/new_line.go),
//! [include.go](suiron/include.go), [exclude.go](suiron/exclude.go), greater_than (etc.)
//!
//! ...and some arithmetic functions: [add.go](suiron/add.go), [subtract.go](suiron/subtract.go),
//! [multiply.go](suiron/multiply.go), [divide.go](suiron/divide.go)
//!
//! Please refer to the test programs for examples of how to use these.
//!
//! To run the tests, open a terminal window, go to the test folder, and execute 'run'.
//!
//!```bash
//! cd test
//! ./run
//!```
//!
//! Suiron allows you to write your own built-in predicates and functions.
//! The files [bip_template.go](suiron/bip_template.go) and [bif_template.go](suiron/bif_template.go)
//! can be used as templates. Please read the comments in the headers of these files.
//!
//! The files [hyphenate.go](test/hyphenate.go) and [capitalize.go](test/capitalize.go) in the test
//! directory can also be used for reference.
//!
//! ## Developer
//!
//! Suiron was developed by Cleve (Klivo) Lendon.
//!
//! ## Contact
//!
//! To contact the developer, send email to indriko@yahoo.com .
//! Comments, suggestions and criticism are welcomed.
//!
//! ## History
//!
//! First release, April 2023.
//!
//! ## Reference
//!
//! This inference engine is inspired by the Predicate Calculus Problem Solver
//! presented in chapters 23 and 24 of 'AI Algorithms...' by Luger and Stubblefield.
//! I highly recommend this book.
//!
//!<blockquote>
//! AI Algorithms, Data Structures, and Idioms in Prolog, Lisp, and Java<br>
//! George F. Luger, William A. Stubblefield, ©2009 | Pearson Education, Inc.<br>
//! ISBN-13: 978-0-13-607047-4<br>
//! ISBN-10: 0-13-607047-7<br>
//!</blockquote>
//!
//! ## License
//!
//! The source code for Suiron is licensed under the MIT license,
//! which you can find here:  [LICENSE](./LICENSE).
//!

pub mod unifiable;
pub mod substitution_set;
pub mod logic_var;
pub mod s_linked_list;
pub mod s_complex;
pub mod solutions;
pub mod parse_terms;
pub mod parse_goals;
pub mod rule_reader;
pub mod token;
pub mod tokenizer;
pub mod parse_stack;
pub mod operator;
pub mod goal;
pub mod rule;
pub mod knowledge_base;
pub mod solution_node;
pub mod solution_node_and_or;
pub mod built_in_print;
pub mod built_in_append;
pub mod built_in_filter;
pub mod built_in_predicates;
pub mod built_in_functions;
pub mod built_in_comparison;
pub mod built_in_arithmetic;
pub mod built_in_print_list;
pub mod built_in_count;
pub mod built_in_join;
pub mod time_out;
pub mod infix;
pub mod benchmark;

#[macro_use]
pub mod macros;

pub use unifiable::*;
pub use unifiable::Unifiable::*;
pub use substitution_set::*;
pub use logic_var::*;
pub use s_linked_list::*;
pub use s_complex::*;
pub use solutions::*;
pub use parse_terms::*;
pub use parse_goals::*;
pub use rule_reader::*;
pub use token::*;
pub use tokenizer::*;
pub use parse_stack::*;
pub use operator::*;
pub use goal::*;
pub use rule::*;
pub use knowledge_base::*;
pub use solution_node::*;
pub use solution_node_and_or::*;
pub use built_in_print::*;
pub use built_in_append::*;
pub use built_in_filter::*;
pub use built_in_predicates::*;
pub use built_in_functions::*;
pub use built_in_comparison::*;
pub use built_in_arithmetic::*;
pub use built_in_print_list::*;
pub use built_in_count::*;
pub use built_in_join::*;
pub use time_out::*;
pub use infix::*;
pub use benchmark::*;
