# Suiron - An inference engine written in Rust.

Suiron is a fast inference engine.
Its fact/rule definition language is similar to Prolog, but there are some differences.

To understand how to use Suiron, a basic understanding of
[Prolog](https://en.wikipedia.org/wiki/Prolog) is helpful.
Here are a couple of useful links:

[Logic Programming](http://athena.ecs.csus.edu/~mei/logicp/prolog.html)<br>
[cse341](https://courses.cs.washington.edu/courses/cse341/12au/prolog/basics.html)

There is an on-line tutorial and test site for Suiron at:
[klivo.net/suiron](https://klivo.net/suiron)

## Briefly

An inference engines analyzes facts and rules which are stored in a knowledge base.

Suiron loads facts and rules from a text-format source file, parses them,
and writes them to the knowledge base.

Below is an example of a fact, which means "June is the mother of Theodore":

<pre>mother(June, Theodore).</pre>

Here we see the main difference between Suiron and Prolog.
In Prolog, lower case words are `atoms` (that is, string constants) and upper case
words are variables. In Suiron, atoms can be lower case or upper case. Thus `mother`,
`June` and `Theodore` are all atoms. Suiron's atoms can even contain spaces.

<pre>mother(June, The Beaver).</pre>

Suiron's variables are defined by putting a dollar sign in front of the variable name,
for example, $Child. A query to determine June's children would be written:

<pre>mother(June, $Child).</pre>

The anonymous variable must also begin with a dollar sign: $\_ .
A simple underscore `_` is treated as an atom.
Below is an example of a rule which contains an anonymous variable:

<pre>voter($P) :- $P = person($_, $Age), $Age >= 18.</pre>

<br><hr><br>

Facts and rules can also be created dynamically within a Rust application program.
The fact mother(June,&nbsp;Theodore) could be created by calling the functions
parse\_complex() and make\_fact().

<pre>
let term = parse_complex("mother(June, Theodore).");
let fact = make_fact(term);</pre>

The query mother(June, $Child) could be created in Rust source as follows:

<pre>
let mother = atom!("mother");
let june   = atom!("June");
let child  = logic_var!("$Child");
let query  = query!(mother, june, child);</pre>

Suiron also supports integer and floating point numbers, which are
implemented as 64-bit ints and floats.

<pre>
let pi = SFloat(3.14159);
let year = SInteger(2023);</pre>

If a float and an integer are compared, the integer will be converted to
a float for the comparison.

Of course, Suiron supports linked lists, which work the same way as Prolog lists.
A linked list can be loaded from a source file:

<pre>
  …, [a, b, c, d] = [$Head | $Tail], …</pre>

or created dynamically in Rust:

<pre>
let list1 = parse_linked_list("[a, b, c | $X]");
let list2 = make_linked_list(false, terms);</pre>

## Requirements

Suiron was developed and tested with Rust/Cargo version 1.65.0.

[https://www.rust-lang.org/](https://www.rust-lang.org/)

## Cloning

To clone the repository, run the following command in a terminal window:

<pre>
git clone git@github.com:Indrikoterio/suiron-rust.git</pre>

The repository has the following subfolders:

- suiron-rust/benches
- suiron-rust/src
- suiron-rust/suiron\_demo
- suiron-rust/target
- suiron-rust/tests

The source code for Suiron itself is under /src.

The subfolder /tests has programs which test the basic functionality of the
inference engine. Tests can be run by opening a command line interface, moving
to the suiron-rust folder and running the following command.

<pre>cargo test</pre>

The program under /benches (suiron_benchmark.rs) uses the Criterion crate
to run a qsort algorithm. On a MacBook Pro, with a 2.8 GHz dual core Intel Core i5
processor, this benchmark runs in about 32 milliseconds. The program
can be run with the command `cargo bench`.

The subfolder /suiron_demo contains a simple demo program which parses
English sentences. If you intend to incorporate Suiron into your own project,
this is a good reference.
See: [Suiron Demo](./suiron_demo/src/suiron_demo.rs)

The /target folder holds build results.

## Usage

The crate `query` uses `suiron` library crate to loads facts and rules
from a file, and allows the user to query the knowledge base.
Query can be run in a terminal window as follows:

<pre>
cargo run -- test/kings.txt</pre>

The user will be prompted for a query with this prompt: ?-

The query below will print out all father/child relationships.

<pre>
?- father($F, $C).</pre>

After typing &lt;enter&gt;, the program will print out solutions,
one after each press of &lt;enter&gt;, until there are no more solutions.

<pre>
cargo run -- test/kings.txt
?- father($F, $C).
$F = Godwin, $C = Harold II
$F = Godwin, $C = Tostig
$F = Godwin, $C = Edith
$F = Tostig, $C = Skule
$F = Harold II, $C = Harold
No more.
?- </pre>

Suiron doesn't have a lot of built-in predicates, but it does have:

- append
- functor
- print
- print\_list
- nl (new line)
- include, exclude
- greater\_than, less\_than, etc.
- arithmatic functions: +, -, *, /

Please refer to the test programs for examples of how to use these.

## Developer

Suiron was developed by Cleve (Klivo) Lendon.

## Contact

To contact the developer, send email to indriko@yahoo.com .<br>
Comments, suggestions and criticism are welcomed.

## History

First release, May 2023.

## Reference

This inference engine was inspired by the Predicate Calculus Problem Solver
presented in chapters 23 and 24 of 'AI Algorithms...' by Luger and Stubblefield.
I highly recommend this book.

<blockquote>
AI Algorithms, Data Structures, and Idioms in Prolog, Lisp, and Java<br>
George F. Luger, William A. Stubblefield, ©2009 | Pearson Education, Inc.<br>
ISBN-13: 978-0-13-607047-4<br>
ISBN-10: 0-13-607047-7<br>
</blockquote>

## License

The source code for Suiron is licensed under the MIT license,
which you can find here: [LICENSE](./LICENSE.txt).

