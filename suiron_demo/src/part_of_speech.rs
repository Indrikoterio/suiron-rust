//! Functions which create a part-of-speech hashmap and word-facts.
//!
//! The file [part_of_speech.txt](../../../../src/part_of_speech.txt)
//! contains part-of-speech tags for English words,<br>based on Penn State's Treebank tagset.
//! Reference: [Part-of-Speech Tutorial](https://sites.google.com/site/partofspeechhelp/home)
//!
//! ABN pre-quantifier (half, all)<br>
//! AP post-determiner (many, several, next)<br>
//! AT article (a, the, no)<br>
//! BE be<br>
//! BED were<br>
//! BEDZ was<br>
//! BEG being<br>
//! BEM am<br>
//! BEN been<br>
//! BER are, art<br>
//! BBB is<br>
//! CC coordinating conjunction<br>
//! CD cardinal digit<br>
//! DT determiner<br>
//! EX existential there (like: “there is” … think of it like “there exists”)<br>
//! FW foreign word<br>
//! IN preposition/subordinating conjunction<br>
//! JJ adjective 'big'<br>
//! JJR adjective, comparative 'bigger'<br>
//! JJS adjective, superlative 'biggest'<br>
//! LS list marker 1)<br>
//! MD modal could, will<br>
//! NN noun, singular 'desk'<br>
//! NNS noun plural 'desks'<br>
//! NNP proper noun, singular 'Harrison'<br>
//! NNPS proper noun, plural 'Americans'<br>
//! OD ordinal numeral (first, 2nd)<br>
//! NPS proper noun, plural Vikings<br>
//! PDT predeterminer 'all the kids'<br>
//! PN nominal pronoun (everybody, nothing)<br>
//! PP$ possessive personal pronoun (my, our)<br>
//! PP$$ second (nominal) personal pronoun (mine, ours)<br>
//! PPO objective personal pronoun (me, him, it, them)<br>
//! PPS 3rd. singular nominative pronoun (he, she, it, one)<br>
//! PPSS other nominative personal pronoun (I, we, they, you)<br>
//! POS possessive ending parent's<br>
//! PRP personal pronoun I, he, she<br>
//! PRP$ possessive pronoun my, his, hers<br>
//! QL qualifier (very, fairly)<br>
//! QLP post-qualifier (enough, indeed)<br>
//! RB adverb very, silently,<br>
//! RBR adverb, comparative better<br>
//! RBS adverb, superlative best<br>
//! RP particle give up<br>
//! SYM symbol<br>
//! TO to go 'to' the store.<br>
//! UH interjection errrrrm<br>
//! VB verb, base form take<br>
//! VBD verb, past tense took<br>
//! VBG verb, gerund/present participle taking<br>
//! VBN verb, past participle taken<br>
//! VBP verb, sing. present, non-3d take<br>
//! VBZ verb, 3rd person sing. present takes<br>
//! WDT wh-determiner which<br>
//! WP wh-pronoun who, what<br>
//! WP$ possessive wh-pronoun whose<br>
//! WRB wh-abverb where, when<br>
//!
// Cleve (Klivo) Lendon  2023

use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use suiron::*;

/// Reads in part-of-speech data and creates a hashmap of tags.
///
/// This function reads in part-of-speech (PoS) data from a text file.
/// Each line of the file has an English word and its tags.
/// For example, the line `name NN VB` indicates that the word
/// `name` can be a noun or a verb.
///
/// The hashmap is indexed by word; each entry is a vector of PoS tags.
///
/// # Arguments
/// * file name
/// # Return
/// * PoS hashmap or error message
/// # Usage
/// ```
/// let _pos = match create_pos_map("./src/part_of_speech.txt") {
///     Ok(p) => { p },
///     Err(err) => {
///         println!("{}", err);
///         return;
///     },
/// };
/// ```
pub fn create_pos_map(file_name: &str) -> Result<HashMap<String, Vec<String>>, String> {

    // Map: word / Part of Speech.
    let mut word_pos: HashMap<String, Vec<String>> = HashMap::new();

    match line_reader(file_name) {
        Ok(lines) => {
            for line in lines {
                if let Ok(line) = line {
                    let line = line.trim();
                    let chrs = str_to_chars!(line);
                    if let Some(index) = chrs.iter().position(|&r| r == ' ') {
                        let word = chars_to_string!(chrs[0..index]);
                        let the_rest = chars_to_string!(chrs[index + 1..]);
                        // pos is an array of strings
                        let pos = the_rest.split(" ").collect::<Vec<&str>>();
                        let mut pos2 = vec![];
                        for p in pos { pos2.push(p.to_string()); }
                        word_pos.insert(word, pos2);
                    }
                } // if let Ok(line)
            } // for
        },
        Err(msg) => {
            // Add file name to error message.
            let msg = format!("{}: {}", msg, file_name);
            return Err(msg);
        },
    } // match

    return Ok(word_pos);

} // create_pos_map

/// Makes a word lower case, except if it's the pronoun I.
///
/// # Arguments
/// * word
/// # Return
/// * lower case word
/// # Usage
/// ```
/// let word = lower_case_except_i("Twilight");
/// println!("{}", word);  // Prints: twilight
/// let word = lower_case_except_i("I");
/// println!("{}", word);  // Prints: I
/// let word = lower_case_except_i("I'm");
/// println!("{}", word);  // Prints: I'm
/// ```
fn lower_case_except_i(word: &str) -> String {
    if word == "I" || word.starts_with("I'") { return word.to_string(); }
    return word.to_lowercase();
}

/// Creates a pronoun term according to the given word and its tag.
///
/// Example pronoun term: `pronoun(they, subject, third, plural)`
///
/// # Note
/// * This function does not handle the pronoun `you`.
/// `You` is dealt with separately, by
/// [make_you_facts()](../part_of_speech/fn.make_you_facts.html).
///
/// # Arguments
/// * word
/// * word in lower case
/// * part of speech tag
/// # Return
/// * complex term or None
/// # Usage
/// ```
/// let term = make_pronoun_term("We", "we", "PPSS");
/// match term {
///     Some(term) => { println!("{}", term); },
///     None => { println!("Invalid"); },
/// }
/// // Prints: pronoun(We, subject, first, plural)
/// ```
fn make_pronoun_term(word: &str, lower: &str, tag: &str) -> Option<Unifiable> {

    if tag.starts_with("PPS") { // PPS or PPSS
        let term = match lower {
            "we"   => scomplex!(atom!("pronoun"), atom!(word),
                                atom!("subject"), atom!("first"), atom!("plural")),
            "they" => scomplex!(atom!("pronoun"), atom!(word),
                                atom!("subject"), atom!("third"), atom!("plural")),
            "I"    => scomplex!(atom!("pronoun"), atom!(word),
                                atom!("subject"), atom!("first"), atom!("singular")),
            _      => scomplex!(atom!("pronoun"), atom!(word),
                                atom!("subject"), atom!("third"), atom!("singular")),
        };
        return Some(term);
    } else if tag.starts_with("PPO") {
        let term = match lower {
            "us"   => scomplex!(atom!("pronoun"), atom!(word),
                                atom!("object"), atom!("first"), atom!("plural")),
            "them" => scomplex!(atom!("pronoun"), atom!(word),
                                atom!("object"), atom!("third"), atom!("plural")),
            "me"   => scomplex!(atom!("pronoun"), atom!(word),
                                atom!("object"), atom!("first"), atom!("singular")),
            _      => scomplex!(atom!("pronoun"), atom!(word),
                                atom!("object"), atom!("third"), atom!("singular")),
        };
        return Some(term);
    }

    return None;

} // make_pronoun_term


/// Creates facts for the pronoun `you`.
///
/// For example: `word(you, pronoun(You, subject, second, singular))`
///
/// # Arguments
/// * word
/// # Return
/// * vector of facts
/// # Usage
/// ```
/// let pronouns = make_you_facts("You");
/// for pronoun in pronouns {
///     println!("{}", pronoun);
/// }
/// // Prints:
/// // word(you, pronoun(You, subject, second, singular)).
/// // word(you, pronoun(You, object, second, singular)).
/// // word(you, pronoun(You, subject, second, plural)).
/// // word(you, pronoun(You, object, second, plural)).
/// ```
fn make_you_facts(word: &str) -> Vec<Rule> {

    let mut facts: Vec<Rule> = vec![];

    let pronouns = vec![
        scomplex!(atom!("pronoun"), atom!(word),
                  atom!("subject"), atom!("second"), atom!("singular")),
        scomplex!(atom!("pronoun"), atom!(word),
                  atom!("object"), atom!("second"), atom!("singular")),
        scomplex!(atom!("pronoun"), atom!(word),
                  atom!("subject"), atom!("second"), atom!("plural")),
        scomplex!(atom!("pronoun"), atom!(word),
                  atom!("object"), atom!("second"), atom!("plural")),
    ];

    for term in pronouns {
        let new_term = scomplex!(atom!("word"), atom!("you"), term);
        let fact = make_fact(new_term);
        facts.push(fact);
    }

    return facts;

} // make_you_facts

/// Creates a verb term, eg. `verb(listen, present, base)`.
///
/// # Arguments
/// * word
/// * part of speech tag
/// # Return
/// * term or None
/// # Unify
/// ```
/// let verb = make_verb_term("listen", "VB");
/// match verb {
///     Some(verb) => { println!("{}", verb); },
///     None => { println!("Invalid"); },
/// }
/// // Prints: verb(listen, present, base)
/// ```
fn make_verb_term(word: &str, tag: &str) -> Option<Unifiable> {

    if tag == "VB" {
        return Some(scomplex!(atom!("verb"), atom!(word), atom!("present"), atom!("base")));
    } else if tag == "VBZ" {
        return Some(scomplex!(atom!("verb"), atom!(word), atom!("present"), atom!("third_sing")));
    } else if tag == "VBD" {
        return Some(scomplex!(atom!("verb"), atom!(word), atom!("past"), atom!("past")));
    } else if tag == "VBG" {
        return Some(scomplex!(atom!("participle"), atom!(word), atom!("active")));
    } else if tag == "VBN" {
        return Some(scomplex!(atom!("participle"), atom!(word), atom!("passive")));
    }

    return None;

} // make_verb_term


/// Creates a noun term, eg. `noun(speaker, singular)`.
///
/// # Arguments
/// * word
/// * part of speech tag
/// # Return
/// * term or None
/// # Usage
/// ```
/// let noun = make_noun_term("speaker", "NN");
/// match noun {
///     Some(noun) => { println!("{}", noun); },
///     None => { println!("Invalid"); },
/// }
/// // Prints: noun(speaker, singular)
/// ```
fn make_noun_term(word: &str, tag: &str) -> Option<Unifiable> {

    if tag == "NN" {
        return Some(scomplex!(atom!("noun"), atom!(word), atom!("singular")));
    } else if tag == "NNS" {
        return Some(scomplex!(atom!("noun"), atom!(word), atom!("plural")));
    } else if tag == "NNP" {
        return Some(scomplex!(atom!("noun"), atom!(word), atom!("singular")));
    }

    return None;

} // make_noun_term


/// Creates an adjective term, eg. `adjective(happy, positive)`.
///
/// # Arguments
/// * word
/// * part of speech tag
/// # Return
/// * term or None
/// # Usage
/// ```
/// let term = make_adjective_term("happy", "JJ");
/// match term {
///     Some(term) => { println!("{}", term); },
///     None => { println!("Invalid"); },
/// }
/// // Prints: adjective(happy, positive)
/// ```
fn make_adjective_term(word: &str, tag: &str) -> Option<Unifiable> {

    if tag == "JJ" {
        return Some(scomplex!(atom!("adjective"), atom!(word), atom!("positive")));
    } else if tag == "JJR" {
        return Some(scomplex!(atom!("adjective"), atom!(word), atom!("comparative")));
    } else if tag == "JJS" {
        return Some(scomplex!(atom!("adjective"), atom!(word), atom!("superlative")));
    }

    return None;

} // make_adjective_term

/// Creates terms for articles, eg. `article(a, indefinite)`.
///
/// # Arguments
/// * word
/// # Return
/// * term
/// # Usage
/// ```
/// let term = make_article_term("a");
/// if let Some(term) = term {
///     println!("{}", term);
/// }
/// // article(a, indefinite)
/// ```
fn make_article_term(word: &str) -> Option<Unifiable> {

    let lower = word.to_lowercase();
    if lower == "the" {
        let term = scomplex!(atom!("article"), atom!(word), atom!("definite"));
        return Some(term);
    }
    let term = scomplex!(atom!("article"), atom!(word), atom!("indefinite"));
    return Some(term);

} // make_article_term

/// Creates adverb terms, eg. `adverb(happily)`.
///
/// # Arguments
/// * word
/// # Return
/// * term or None
/// # Usage
/// ```
/// let term = make_adverb_term("happily");
/// if let Some(term) = term { println!("{}", term); }
/// // Prints: adverb(happily)
/// ```
fn make_adverb_term(word: &str) -> Option<Unifiable> {
    let term = scomplex!(atom!("adverb"), atom!(word));
    return Some(term);
} // make_adverb_term

/// Creates preposition terms, eg. `preposition(from)`.
///
/// # Arguments
/// * word
/// # Return
/// * term or None
/// # Usage
/// ```
/// let term = make_preposition_term("from");
/// if let Some(term) = term { println!("{}", term); }
/// // Prints: preposition(from)
/// ```
fn make_preposition_term(word: &str) -> Option<Unifiable> {
    let term = scomplex!(atom!("preposition"), atom!(word));
    return Some(term);
} // make_preposition_term

/// Creates a complex term for an English word.
///
/// The third argument is a part of speech tag, such as NNS or VBD.<br>
/// Reference: [Part-of-Speech Tutorial](https://sites.google.com/site/partofspeechhelp/home)
///
/// # Arguments
/// * word
/// * lower case word
/// * part of speech tag
/// # Return
/// * complex term or None
/// # Usage
/// ```
/// let term = make_term("Dancing", "dancing", "VBG");
/// match term {
///     Some(term) => { println!("{}", term); },
///     None => { println!("Invalid"); },
/// }
/// // Prints: participle(Dancing, active)
/// ```
fn make_term(word: &str, lower: &str, tag: &str) -> Option<Unifiable> {
    if tag.starts_with("VB") {
        return make_verb_term(word, tag);
    } else if tag.starts_with("NN") {
        return make_noun_term(word, tag);
    } else if tag.starts_with("PP") {
        return make_pronoun_term(word, lower, tag);
    } else if tag.starts_with("JJ") {
        return make_adjective_term(word, tag);
    } else if tag.starts_with("AT") {
        return make_article_term(word);
    } else if tag.starts_with("IN") {
        return make_preposition_term(word);
    } else if tag.starts_with("RB") {
        return make_adverb_term(word);
    }
    return None;
} // make_term

/// Takes a word string and produces facts for the knowledge base.
///
/// Some words produce only one fact. For example, 'the' can only
/// be a definite article:
///
/// <pre>
///    article(the, definite)
/// </pre>
///
/// Other words can have more than one part of speech. The word
/// 'envy', for example, might be a noun or a verb.
///
/// <pre>
///    noun(envy, singular)
///    verb(envy, present, base)
/// </pre>
///
/// For 'envy', a parsing algorithm must be able to test both
/// possibilities. Therefore, the inference engine will need two
/// facts for the knowledge base:
///
/// <pre>
///    word(envy, noun(envy, singular)).
///    word(envy, verb(envy, present, base)).
/// </pre>
///
/// # Argument
/// * word
/// * part-of-speech (hashmap)
/// # Return
/// * list of facts (= rules)
/// # Unify
/// ```
/// // Get part-of-speech tags.
/// let pos = match create_pos_map("./src/part_of_speech.txt") {
///     Ok(p) => { p },
///     Err(err) => { println!("{}", err); return; },
/// };
/// let facts = word_to_facts("envy", &pos);
/// for fact in facts { println!("{}", fact); }
/// // Prints:
/// // word(envy, noun(envy, singular)).
/// // word(envy, verb(envy, present, base)).
/// ```
fn word_to_facts(word: &str, pos: &HashMap<String, Vec<String>>) -> Vec<Rule> {

    let lower = &lower_case_except_i(word);

    // Handle pronoun 'you', which is very ambiguous.
    if lower == "you" { return make_you_facts(word); }

    let length = word.len();
    if length == 1 { // Maybe this is punctuation.
        if let Some(term) = make_punctuation_term(word) {
            let punc_term = scomplex!(atom!("word"), atom!(word), term);
            let fact = make_fact(punc_term);
            return vec![fact];
        }
    }

    let mut facts: Vec<Rule> = vec![];

    let mut pos_data = pos.get(word);
    if pos_data.is_none() {
        pos_data = pos.get(lower);
    }

    if let Some(pos_data) = pos_data {
        if pos_data.len() > 0 {
            for pos in pos_data {
                if let Some(term) = make_term(word, lower, pos) {
                    let word_term = scomplex!(atom!("word"), atom!(word), term);
                    let fact = make_fact(word_term);
                    facts.push(fact);
                }
            } // for
        }
    }

    if facts.len() < 1 {
        let term = scomplex!(atom!("unknown"), atom!(word));
        let word_term = scomplex!(atom!("word"), atom!(word), term);
        let fact = make_fact(word_term);
        facts.push(fact);
    }

    return facts;

} // word_to_facts

/// Takes a list of words, and creates a list of facts.
///
/// The word 'envy', for example, should produce two facts.
///
/// <pre>
///    word(envy, noun(envy, singular)).
///    word(envy, verb(envy, present, base)).
/// </pre>
///
/// # Note
/// * A Fact is the same as a
/// [Rule](../../../doc/suiron/rule/struct.Rule.html)
/// without a body.
///
/// # Argument
/// * list of words
/// * hashmap of `part-of-speech` data
/// # Return
/// * list of facts (= rules)
/// # Usage
/// ```
/// // Get part-of-speech tags.
/// let pos = match create_pos_map("./src/part_of_speech.txt") {
///     Ok(p) => { p },
///     Err(err) => { println!("{}", err); return; },
/// };
/// let words = vec!["He".to_string(), "envies".to_string()];
/// let facts = make_facts(&words, &pos);
/// for fact in facts { println!("{}", fact); }
/// // Prints:
/// // word(He, pronoun(He, subject, third, singular)).
/// // word(envies, noun(envies, plural)).
/// // word(envies, verb(envies, present, third_sing)).
/// ```
pub fn make_facts(words: &Vec<String>, pos: &HashMap<String, Vec<String>>) -> Vec<Rule> {
    let mut facts: Vec<Rule> = vec![];
    for word in words {
        let word_facts = word_to_facts(&word, pos);
        for word_fact in word_facts { facts.push(word_fact); }
    }
    return facts;
} // make_facts

/// Creates an iterator which reads lines from a file.
///
/// # Arguments
/// * file name
/// # Return
/// * line reader
/// # Reference
/// [read_lines](https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html)
/// # Usage
/// ```
/// let file_name = "./src/sentences.txt";
/// match line_reader(file_name) {
///     Ok(lines) => {
///         for line in lines {
///             if let Ok(line) = line { println!("{}", line); }
///         }
///     },
///     Err(err) => { println!("{}", err); },
/// } // match
/// ```
///
fn line_reader<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Makes complex terms which represent punctuation.
///
/// For example, `;` produces the term `semicolon(;)`.
///
/// Some terms have two values. The second is used for comparison.
/// For example, <code>quote_mark(‘, ‘)</code> and <code>quote_mark(’, ‘)</code>
/// represent an opening and a closing quote mark. The second term is used to
/// ensure that these punctuation marks are of the same type.
///
/// # Arguments
/// * symbol (string)
/// # Return
/// * [SComplex](../../../doc/suiron/unifiable/enum.Unifiable.html#SComplex) or None
/// # Usage
/// ```
/// let term = make_punctuation_term(";");
/// if let Some(term) = term { println!("{}", term); };
/// // Prints: semicolon(;)
/// ```
pub fn make_punctuation_term(symbol: &str) -> Option<Unifiable> {

    let symbol = str_to_chars!(symbol);
    if symbol.len() != 1 { return None; }

    let punc = match symbol[0] {
        '.' => { parse_complex("period(.)") },
        // Must escape the comma with backslash.
        ',' => { parse_complex("comma(\\,)") },
        '?' => { parse_complex("question_mark(?)") },
        '!' => { parse_complex("exclamation_mark(!)") },
        ':' => { parse_complex("colon(:)") },
        ';' => { parse_complex("semicolon(;)") },
        '-' => { parse_complex("dash(-)") },
        // The second argument is for comparisons.
        '\"' => { parse_complex("quote_mark(\", \")") },
        '\'' => { parse_complex("quote_mark(', ')") },
        '«' => { parse_complex("quote_mark(«, «)") },
        '»' => { parse_complex("quote_mark(», «)") },
        '‘' => { parse_complex("quote_mark(‘, ‘)") },
        '’' => { parse_complex("quote_mark(’, ‘)") },
        '“' => { parse_complex("quote_mark(“, “)") },
        '”' => { parse_complex("quote_mark(”, “)") },
        '(' => { parse_complex("bracket((, ()") },
        ')' => { parse_complex("bracket(), ()") },
        '[' => { parse_complex("bracket([, [)") },
        ']' => { parse_complex("bracket(], [)") },
        '<' => { parse_complex("bracket(<, <)") },
        '>' => { parse_complex("bracket(>, <)") },
        _   => { Err("Not punctuation.".to_string()) },
    };

    match punc {
        Ok(punctuation_term) => Some(punctuation_term),
        Err(_) => None,
    }

} // make_punctuation_term()
