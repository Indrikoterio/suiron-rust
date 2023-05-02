//! This demo program parses simple English sentences and checks for grammatical errors.
//! It is not intended to be complete or practical.
//!
//! To run the program, open a command line interface, move to the suiron_demo
//! folder, and enter:<br>
//!
//! <pre>
//! cargo run
//! </pre>
//!
//! The program should output:
//!
//! <pre>
//! They envy us.
//! He envy us. --> 'He' and 'envy' do not agree.
//! He envies us.
//! I envied them.
//! I envies them. --> 'I' and 'envies' do not agree.
//! Cats loves me. --> 'Cats' and 'loves' do not agree.
//! Diamonds lasts. --> 'Diamonds' and 'lasts' do not agree.
//! Love lasts.
//! </pre>
//!
//! In order to understand the comments below, it is useful to have
//! a basic understanding of logic programming.<br>Here are a couple references:
//!
//! &nbsp; &nbsp; [Logic Programming](http://athena.ecs.csus.edu/~mei/logicp/prolog.html)<br>
//! &nbsp; &nbsp; [cse341](https://courses.cs.washington.edu/courses/cse341/12au/prolog/basics.html)
//!
//! # Description of Rust Code
//!
//! The program starts at [main](../suiron_demo/fn.main.html).
//! First it...
//!
//!   - creates an empty
//!   [knowledge base](../suiron/knowledge_base/index.html)
//!   - loads part-of-speech data into a hashmap, see
//!   [create_pos_map()](../suiron_demo/part_of_speech/fn.create_pos_map.html)
//!   - creates some [unifiable terms](../suiron/unifiable/enum.Unifiable.html),
//!   (atoms, logic variables, etc.), and [rules](../suiron/rule/index.html)
//!   - stores these rules into the knowledge base
//!   - loads additional facts and rules from a file, see
//!   [load_kb_from_file()](../suiron/rule_reader/fn.load_kb_from_file.html)
//!   - reads in a text file for analysis: [sentences.txt](../../../src/sentences.txt)
//!   - splits the text into sentences, see
//!   [split_into_sentences()](../suiron_demo/fn.split_into_sentences.html)
//!
//! Next, the program calls
//! [sentence_to_facts()](../suiron_demo/fn.sentence_to_facts.html).
//! This function does several things. It calls
//! [sentence_to_words()](../suiron_demo/sentence/fn.sentence_to_words.html)
//! to divide each sentence into words and punctuation. For example:
//!
//! <pre>
//!    "They envy us."
//! </pre>
//!
//! becomes...
//!
//! <pre>
//!    ["They", "envy", "us", "."]
//! </pre>
//!
//! Next it creates a Suiron list by calling
//! [make_linked_list()](../suiron/s_linked_list/fn.make_linked_list.html):
//!
//! <pre>
//!    [They, envy, us, .]
//! </pre>
//!
//! Note: In Prolog, words which begin with a capital letter are variables.
//! In Suiron, variables do not begin with a capital. The first term in the
//! list above (`They`) is an atom, a string constant. Variables, in Suiron,
//! begin with a dollar sign.
//!
//! Next, sentence_to_facts() calls the function
//! [make_facts()](../suiron_demo/part_of_speech/fn.make_facts.html).
//! This function produces facts which associate each word with a gramatical fact.
//! For example:
//!
//! <pre>
//!    word(we, pronoun(we , subject, first, plural))
//! </pre>
//!
//! Many words can have more than one part of speech. The word `envy`,
//! for example, can be a noun or a verb. In order to parse English sentences,
//! the program needs facts which identify all possible parts of speech:
//!
//! <pre>
//!     word(envy, noun(envy, singular)).
//!     word(envy, verb(envy, present, base)).
//! </pre>
//!
//! Finally, the program makes a query for each sentence
//! (eg. `parse([They, envy, us, .], $_).`)
//! and calls the method [solve()](../suiron/solutions/fn.solve.html)
//! to do the analysis.
//!
//! # Suiron Analysis
//!
//! The grammar rules for this demo are in
//! [demo_grammar.txt](../../../src/demo_grammar.txt).
//!
//! During analysis, the rule words_to_pos/2 is applied to convert the input
//! word list, created by sentence_to_facts(), into a list of terms which identify
//! part of speech.
//!
//! <pre>
//!   words_to_pos([$H1 | $T1], [$H2 | $T2]) :-
//!                          word($H1, $H2), words_to_pos($T1, $T2).
//!   words_to_pos([], []).
//! </pre>
//!
//! The sentence "They envy us." will become:
//!
//! <pre>
//! [pronoun(They, subject, third, plural), verb(envy, present, base),
//!          pronoun(us, object, first, plural), period(.)]
//! </pre>
//!
//! The predicate `sentence` identifies (unifies with) various types
//! of sentence, such as:
//!
//! <pre>
//!   subject pronoun, verb
//!   subject noun, verb
//!   subject pronoun, verb, object
//!   subject noun, verb, object
//! </pre>
//!
//! There are rules to check subject/verb agreement of these sentences:
//!
//! <pre>
//!    check_pron_verb
//!    check_noun_verb
//! </pre>
//!
//! When a mismatch is found (*He envy), these rules print out an error message:
//!
//! <pre>
//! 'He' and 'envy' do not agree.
//! </pre>
//!
// Cleve (Klivo) Lendon
// 2023

use std::fs;
use std::rc::Rc;
use std::process;
use std::collections::HashMap;

use suiron::*;

mod sentence;
use sentence::*;

mod part_of_speech;
use part_of_speech::*;

/// The demo program starts here.
fn main() {

    // The knowledge base stores facts and rules.
    let mut kb = KnowledgeBase::new();

    // Load part of speech data from a text file.
    let pos = match create_pos_map("./src/part_of_speech.txt") {
        Ok(p) => { p },
        Err(err) => {
            println!("{}", err);
            process::exit(0);
        },
    };

    // ---------------------------------------
    // Function which produce logic variables.
    fn h1() -> Unifiable { logic_var!("$H1") }
    fn h2() -> Unifiable { logic_var!("$H2") }
    fn t1() -> Unifiable { logic_var!("$T1") }
    fn t2() -> Unifiable { logic_var!("$T2") }

    /*
     words_to_pos/2 is a rule to convert a list of words into a list
     of parts of speech. For example, the atom 'the' is converted to
     the Complex term 'article(the, definite)':

         words_to_pos([$H1 | $T1], [$H2 | $T2]) :- word($H1, $H2),
                                                   words_to_pos($T1, $T2).
         words_to_pos([], []).
     */

    let head = scomplex!(atom!("words_to_pos"),
                         slist!(true, h1(), t1()), slist!(true, h2(), t2()));
    let p1 = pred!("word", h1(), h2());
    let p2 = pred!("words_to_pos", t1(), t2());
    let body = and_goal!(p1, p2);
    let rule = make_rule(head, body);

    // Note: The atom!(), slist!() and scomplex!() macros above can be
    // replaced by a single line:
    //
    // let rule = parse_rule("words_to_pos([$H1 | $T1], [$H2 | $T2]) :- \
    //                        word($H1, $H2), \
    //                        words_to_pos($T1, $T2).").unwrap();
    //
    // parse_rule() will parse the given string to produce a Rule struct.
    //
    // In Prolog, variables begin with a capital letter and atoms begin
    // with a lower case letter. Suiron is a little different. The parser
    // requires a dollar sign to identify variables. An atom can begin
    // with an upper case or lower case letter.

    add_rules!(&mut kb, rule); // Add the rule to our knowledge base.

    let wtp = scomplex!(atom!("words_to_pos"), slist!(), slist!());
    let fact = make_fact(wtp);

    add_rules!(&mut kb, fact);

    // ---------------------------------------
    // Rules for noun phrases.
    let rule = parse_rule("make_np([adjective($Adj, $_), \
                           noun($Noun, $Plur) | $T], [$NP | $Out]) :- \
                           !, $NP = np([$Adj, $Noun], $Plur), \
                           make_np($T, $Out).").unwrap();
    add_rules!(&mut kb, rule);

    let rule = parse_rule("make_np([$H | $T], [$H | $T2]) :- \
                           make_np($T, $T2).").unwrap();
    add_rules!(&mut kb, rule);

    let rule = parse_rule("make_np([], []).").unwrap();
    add_rules!(&mut kb, rule);

    // ---------------------------------------
    // Read facts and rules from file. Load knowledge base.
    if let Some(error_message) =
                load_kb_from_file(&mut kb, "./src/demo_grammar.txt") {
        println!("{}", error_message);
        process::exit(0);
    }

    let file_name = "./src/sentences.txt";
    let text = match read_file(file_name) {
        Ok(t) => t,
        Err(err) => {
            println!("{}: {}", err, file_name);
            process::exit(0);
        },
    };

    let sentences = split_into_sentences(&text);
    for sentence in sentences {

        print!("{}", sentence);

        // Delete previous 'word' facts. Don't want them to accumulate.
        kb.remove("word/2");

        let in_list = sentence_to_facts(&sentence, &mut kb, &pos);
        //print_kb(&kb);

        let query = make_query(vec![atom!("parse"), in_list, anon!()]);
        let query = Rc::new(query);
        let sn = make_base_node(query, &kb);

        solve(sn);
        print!("\n");

    } // for

} // main

/// Reads a file into a string.
///
/// # Argument
/// * file name
/// # Return
/// * file contents or error message
/// # Usage
/// ```
/// match read_file("./src/sentences.txt") {
///     Ok(contents) => { println!("{}", contents); },
///     Err(err) => { println!("{}", err); }
/// }
/// ```
fn read_file(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(file_name)?;
    Ok(data)
} // read_file()

/// Determines whether the given character is a punctuation mark.
///
/// Tests for: !, ?, .
///
/// # Arguments
/// * character to test
/// # Return
/// * true if punctuation
fn is_punc(c: char) -> bool {
    if c == '!' || c == '?' || c == '.' { return true; }
    return false;
} // is_punc

/// Determines whether the given character ends a word.
///
/// If the character is a space or new-line character,
/// it ends the previous word. Return true.
///
/// # Arguments
/// * character to test
/// # Return
/// * true if end of word
fn end_of_word(c: char) -> bool {
    if c == ' ' || c == '\n' { return true; }
    return false;
}  // end_of_word


/// Splits a text into sentences, by searching for punctuation.
///
/// The punctuation must be followed by a space.
/// (The period in 3.14 does not mark the end of a sentence.)
///
/// # Argument
/// * text string
/// # Return
/// * vector of sentences
/// # Usage
/// ```
/// let text = "The value of π is approximately 3.14. But π is an irrational number.";
/// let sentences = split_into_sentences(text);
/// println!("{:?}", sentences);
/// // Prints:
/// // ["The value of π is approximately 3.14.", "But π is an irrational number."]
/// ```
fn split_into_sentences(text: &str) -> Vec<String> {

    let mut sentences: Vec<String> = vec![];
    let mut previous_index = 0;
    let mut previous3 = vec!['a', 'a', 'a'];

    let chrs = str_to_chars!(text);
    for (i, c) in chrs.iter().enumerate() {

        if end_of_word(*c) && is_punc(previous3[2]) {
            if previous3[2] == '.' {
               // Check for H.G. Wells or H. G. Wells
               if previous3[0] != '.' && previous3[0] != ' ' {
                   let s = chars_to_string!(chrs[previous_index..i]);
                   let sentence = s.trim().to_string();
                   sentences.push(sentence);
                   previous_index = i;
               }
            } else {
                let s = chars_to_string!(chrs[previous_index..i]);
                let sentence = s.trim().to_string();
                sentences.push(sentence);
                previous_index = i;
            }
        }
        previous3[0] = previous3[1];
        previous3[1] = previous3[2];
        previous3[2] = *c;

    } // for

    let length = chrs.len();

    let s = chars_to_string!(chrs[previous_index..length]);
    let s = s.trim();
    if s.len() > 0 { sentences.push(s.to_string()); }

    return sentences;

} // split_into_sentences

/// Creates a word list
/// ([SLinkedList](../suiron/unifiable/enum.Unifiable.html#variant.SLinkedList)),
/// and word facts, which are written to the knowledge base.
///
/// # Argument
/// * sentence (string)
/// * knowledge base
/// * part of speech (hashmap)
/// # Return
/// * word list (SLinkedList)
/// # Usage
/// ```
/// // Get part-of-speech tags.
/// let pos = match create_pos_map("./src/part_of_speech.txt") {
///     Ok(p) => { p },
///     Err(err) => { println!("{}", err); return; },
/// };
/// // Knowledge Base
/// let mut kb = KnowledgeBase::new();
/// let sentence = "I know it.";
/// let word_list = sentence_to_facts(sentence, &mut kb, &pos);
/// println!("{}", word_list);
/// print_kb(&kb);
/// ```
/// The above will print.
/// <pre>
/// [I, know, it, .]
/// _____ Contents of Knowledge Base _____
/// word/2
///    word(I, pronoun(I, subject, first, singular)).
///    word(know, verb(know, present, base)).
///    word(it, pronoun(it, subject, third, singular)).
///    word(it, pronoun(it, object, third, singular)).
///    word(., period(.)).
/// </pre>
fn sentence_to_facts(sentence: &str, kb: &mut KnowledgeBase,
                     pos: &HashMap<String, Vec<String>>) -> Unifiable {

    let words: Vec<String> = sentence_to_words(sentence);

    // Create a word list.
    let mut terms: Vec<Unifiable> = vec![];
    for word in &words { terms.push(atom!(word)); }
    let word_list = make_linked_list(false, terms);

    // Make word facts, eg.: word(envy, noun(envy, singular)).
    let facts = make_facts(&words, &pos);
    for fact in facts { add_rules!(kb, fact); }

    return word_list;

} // sentence_to_facts
