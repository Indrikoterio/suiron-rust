//! Functions for parsing unifiable terms and lists of terms.
//!
// Cleve Lendon 2023

use super::infix::*;
use super::parse_goals::*;
use super::s_linked_list::*;
use super::logic_var::*;
use super::s_complex::*;
use super::unifiable::{*, Unifiable::*};
use super::built_in_functions::*;

use crate::atom;
use crate::sfunction;
use crate::str_to_chars;
use crate::chars_to_string;

/// Parses a list of terms (arguments).
///
/// Parses a comma separated list of terms to produce a vector
/// of [Unifiable](../unifiable/enum.Unifiable.html) terms.
///
/// # Arguments
/// * string to parse
/// # Return
/// * vector of
/// [Unifiable](../unifiable/enum.Unifiable.html) terms or error message
/// # Usage
/// ```
/// use suiron::*;
///
/// if let Ok(terms) = parse_arguments("Argon, 18") {
///     println!("{:?}", terms);
/// }
/// // Should print: [Atom("Argon"), SInteger(18)]
/// ```
pub fn parse_arguments(to_parse: &str) -> Result<Vec<Unifiable>, String> {

    let s = to_parse.trim();
    let chrs = str_to_chars!(s);
    let length_chrs = chrs.len();

    if length_chrs == 0 {
        let err = "parse_arguments() - Empty argument list: ";
        return Err(err.to_string());
    }

    if chrs[0] == ',' {
        let err = "parse_arguments() - Missing first argument: ";
        return Err(err.to_string());
    }

    // A comma at the end probably indicates a missing argument, but...
    // make sure comma is not escaped, because this is valid: "term1, term2, \,"
    if chrs[length_chrs - 1] == ',' {
        // Length must be longer than 1, because comma
        // is not the first character.
        let prev = chrs[length_chrs - 2];
        if prev != '\\' {   // escape character
           let err = "parse_arguments() - Missing last argument: ";
           return Err(err.to_string());
        }
    }

    let mut has_digit     = false;
    let mut has_non_digit = false;
    let mut has_period    = false;
    let mut open_quote    = false;

    let mut num_quotes    = 0;
    let mut round_depth   = 0;   // depth of round parentheses (())
    let mut square_depth  = 0;   // depth of square brackets [[]]

    let mut argument = "".to_string();
    let mut term_list = Vec::<Unifiable>::new();

    let mut start = 0;

    let mut i = start;
    while i < length_chrs {

        let ch = chrs[i];

        // If this argument is between double quotes,
        // it must be an Atom.
        if open_quote {
            argument.push(ch);
            if ch == '"' {
                open_quote = false;
                num_quotes += 1;
            }
        }
        else {
            if ch == '[' {
                argument.push(ch);
                square_depth += 1;
            }
            else if ch == ']' {
                argument.push(ch);
                square_depth -= 1;
            }
            else if ch == '(' {
                argument.push(ch);
                round_depth += 1;
            }
            else if ch == ')' {
                argument.push(ch);
                round_depth -= 1
            }
            else if round_depth == 0 && square_depth == 0 {

                if ch == ',' {

                    let s2 = argument.trim();
                    match check_quotes(s2, num_quotes) {
                        Some(err) => { return Err(err); },
                        None => {},
                    }
                    num_quotes = 0;

                    let term = make_term(s2, has_digit, has_non_digit, has_period)?;
                    term_list.push(term);
                    argument    = "".to_string();
                    has_digit   = false;
                    has_non_digit = false;
                    has_period  = false;
                    start = i + 1;    // past comma
                }
                else if ch >= '0' && ch <= '9' {
                    argument.push(ch);
                    has_digit = true
                }
                else if ch == '+' || ch == '-' {
                    argument.push(ch);
                    // Plus or minus might be in front of a number: +7, -3.8
                    // In this case, it is part of the number.
                    let mut next_ch = 'x';
                    if i < length_chrs - 1 { next_ch = chrs[i + 1]; }
                    let mut prev_ch = ' ';
                    if i > 0 { prev_ch = chrs[i]; }
                    if prev_ch == ' ' && (next_ch < '0' || next_ch > '9') {
                        has_non_digit = true;
                    }
                }
                else if ch == '.' {
                    argument.push(ch);
                    has_period = true
                }
                else if ch == '\\' {  // escape character, must include next character
                    if i < length_chrs {
                        i += 1;
                        argument.push(chrs[i]);
                    }
                    else {  // must be at end of argument string
                        argument.push(ch);
                    }
                }
                else if ch == '"' {
                    argument.push(ch);
                    open_quote = true;  // first quote
                    num_quotes += 1;
                }
                else {
                    argument.push(ch);
                    if ch > ' ' { has_non_digit = true; }
                }
            }
            else {
                // Must be between () or []. Just add character.
                argument.push(ch);
            }
        } // not open_quote

        i += 1;

    } // while

    if start < length_chrs {

        let s2 = argument.trim();
        match check_quotes(s2, num_quotes) {
            Some(err) => {
                return Err(err);
            },
            None => {},
        }

        let term = make_term(s2, has_digit, has_non_digit, has_period)?;
        term_list.push(term);
    }

    if round_depth != 0 {
        let err = "parse_arguments() - Unmatched parentheses: ";
        return Err(err.to_string());
    }

    if square_depth != 0 {
        let err = "parse_arguments() - Unmatched brackets: ";
        return Err(err.to_string());
    }

    return Ok(term_list);

} // parse_arguments()


// make_term()
// Creates a Unifiable term from the given string.
//
// Arguments
//    string to parse
//    has_digit     - boolean, true if to_parse has digit
//    has_non_digit - boolean, true if to_parse has non-digit
//    has_period    - boolean, true if to_parse has period
// Return
//    unifiable term or erro message
fn make_term(to_parse: &str,
             has_digit: bool,
             has_non_digit: bool,
             has_period: bool) -> Result<Unifiable, String> {

    let s = to_parse.trim();

    let term_chars = str_to_chars!(s);
    let length_term = term_chars.len();

    if length_term == 0 {
        let err = mt_error("Length of term is 0", s);
        return Err(err);
    }

    let first: char = term_chars[0];
    if first == '$' {

        // Anonymous variable.
        if s == "$_" { return Ok(Anonymous); }

        // If the string is not a valid LogicVar
        // (perhaps $ or $10), make it an Atom.
        match make_logic_var(s.to_string()) {
            Ok(var) => { return Ok(var); },
            Err(_)  => { return Ok(atom!(s)); },
        }
    }

    // If the argument begins and ends with a quotation mark,
    // the argument is an Atom. Strip off quotation marks.
    if length_term >= 2 {
        let last = term_chars[length_term - 1];
        if first == '"' {
            if last == '"' {
                let chars2: Vec<char> = term_chars[1..length_term - 1].to_vec();
                if chars2.len() == 0 {
                    let err = mt_error("Invalid term. Length is 0", s);
                    return Err(err);
                }
                let s2 = chars_to_string!(chars2);
                return Ok(Atom(s2.to_string()));
            } else {
                let err = mt_error("Invalid term. Unmatched quote mark", s);
                return Err(err)
            }
        } else if first == '[' && last == ']' {
            return parse_linked_list(s);
        }
        // Try complex terms, eg.:  job(programmer)
        else if first != '(' && last == ')' {
            // Check for built-in functions.
            if s.starts_with("join(")     { return parse_function(s); }
            if s.starts_with("add(")      { return parse_function(s); }
            if s.starts_with("subtract(") { return parse_function(s); }
            if s.starts_with("multiply(") { return parse_function(s); }
            if s.starts_with("divide(")   { return parse_function(s); }
            return parse_complex(s);
        }
    } // length >= 2

    if has_digit && !has_non_digit { // Must be Integer or Float.
        if has_period {
            match s.parse::<f64>() {
                Ok(fl) => { return Ok(SFloat(fl)); },
                Err(_) => { return Err("Invalid float.".to_string()) },
            }
        } else {
            match s.parse::<i64>() {
                Ok(i) => { return Ok(SInteger(i)); },
                Err(_) => { return Err("Invalid integer".to_string()); },
            }
        }
    }
    return Ok(atom!(s));

}  // make_term

/// Checks validity of double quote marks in a string.
///
/// An argument may be enclosed in double quotation marks, eg. `"Sophie"`.
/// If there are unpaired quotation marks, such as in `""Sophie"`, an error
/// message will be returned. Otherwise, None is returned.
/// # Arguments
/// * string to check
/// * number of double quotes in string (previously counted)
/// # Return
/// * error message or None
/// # Usage
/// Note: In Rust source, double quotes are escaped with a backslash: \\
/// ```
/// use suiron::*;
///
/// if let Some(error_message) = check_quotes("\"\"Sophie\"", 3) {
///     println!("{}", error_message);
/// }
/// // Should print: check_quotes() - Unmatched quotes: ""Sophie"
/// ```
pub fn check_quotes(to_check: &str, count: usize) -> Option<String> {
    if count == 0 { return None; }
    if count != 2 {
        return Some(cq_error("Unmatched quotes", to_check));
    }
    let chrs = str_to_chars!(to_check);
    let first = chrs[0];
    if first != '"' {
        return Some(cq_error("Text before opening quote", to_check));
    }
    let last = chrs[chrs.len() - 1];
    if last != '"' {
        return Some(cq_error("Text after closing quote", to_check));
    }
    None
} // check_quotes()

/// Parses a string to produce a [Unifiable](../unifiable/enum.Unifiable.html) term.
///
/// parse_term(\"$_\") ➔ [Anonymous](../unifiable/enum.Unifiable.html#variant.Anonymous)<br>
/// parse_term(\"verb\") ➔ [Atom](../unifiable/enum.Unifiable.html#variant.Atom)<br>
/// parse_term(\"1.7\") ➔ [SFloat](../unifiable/enum.Unifiable.html#variant.SFloat)<br>
/// parse_term(\"46\") ➔ [SInteger](../unifiable/enum.Unifiable.html#variant.SInteger)<br>
/// parse_term(\"$X\") ➔ [LogicVar](../unifiable/enum.Unifiable.html#variant.LogicVar)<br>
/// parse_term(\"animal(horse, mammal)\") ➔
/// [SComplex](../unifiable/enum.Unifiable.html#variant.SComplex)<br>
/// parse_term(\"[a, b, c]\") ➔
/// [SLinkedList](../unifiable/enum.Unifiable.html#variant.SLinkedList)<br>
/// parse_term(\"$X + 6\") ➔
/// [SFunction](../unifiable/enum.Unifiable.html#variant.SFunction)
///
/// # Arguments
/// * string to parse
/// # Return
/// * [Unifiable](../unifiable/enum.Unifiable.html) term or error message
/// # Usage
/// ```
/// use suiron::*;
///
/// match parse_term(" animal(horse, mammal) ") {
///     Ok(term) => { println!("{}", term); },
///     Err(msg) => { println!("{}", msg); },
/// }
/// // Should print: animal(horse, mammal)
/// ```
pub fn parse_term(to_parse: &str) -> Result<Unifiable, String> {

    let mut s = to_parse.trim();

    let mut has_digit     = false;
    let mut has_non_digit = false;
    let mut has_period    = false;

    let chrs = str_to_chars!(&s);

    // First, let's check for an arithmetic function with an infix,
    // such as $X + 100 or $X / 100.
    let (infix, index) = check_arithmetic_infix(&chrs);

    if infix == Infix::Plus || infix == Infix::Minus ||
       infix == Infix::Multiply || infix == Infix::Divide {

        let (left, right) = get_left_and_right(chrs, index, 1)?;

        let sfunc = match infix {
            Infix::Plus     => { sfunction!("add", left, right) },
            Infix::Minus    => { sfunction!("subtract", left, right) },
            Infix::Multiply => { sfunction!("multiply", left, right) },
            Infix::Divide   => { sfunction!("divide", left, right) },
            _ => {
                let s = format!("parse_term() - Invalid infix {}", infix);
                return Err(s);
            },
        };
        return Ok(sfunc);
    }

    for ch in &chrs {
        if *ch >= '0' && *ch <= '9' {
            has_digit = true;
        } else if *ch == '.' {
            has_period = true;
        } else {
            has_non_digit = true;
        }
    }

    // Check for escaped characters, eg: \,
    if chrs.len() == 2 && chrs[0] == '\\' { s = &s[1..]; }

    return make_term(s, has_digit, has_non_digit, has_period);

}  // parse_term

// Formats an error message for make_term().
// Arguments:
//   err - error description
//   bad - string which caused the error
// Return:
//   error message (String)
fn mt_error(err: &str, bad: &str) -> String {
    format!("make_term() - {}: {}", err, bad)
}

// Formats an error message for check_quotes().
// Arguments:
//   err - error description
//   bad - string which caused the error
// Return:
//   error message (String)
fn cq_error(err: &str, bad: &str) -> String {
    format!("check_quotes() - {}: {}", err, bad)
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::unifiable::Unifiable;

    #[test]
    fn test_check_quotes() {

        // Check: ""Hello"
        if let Some(error_message) = check_quotes("\"\"Hello\"", 3) {
            assert_eq!(error_message,
                       "check_quotes() - Unmatched quotes: \"\"Hello\"");
        }
        // Check: x"Hello"
        if let Some(error_message) = check_quotes("x\"Hello\"", 2) {
            assert_eq!(error_message,
                       "check_quotes() - Text before opening quote: x\"Hello\"");
        }
        // Check: "Hello"x
        if let Some(error_message) = check_quotes("\"Hello\"x", 2) {
            assert_eq!(error_message,
                       "check_quotes() - Text after closing quote: \"Hello\"x");
        }
        // Check: "Hello"
        if let Some(error_message) = check_quotes("\"Hello\"", 2) {
            panic!("The string should be OK: {}", error_message);
        }
        // Check: Hello
        if let Some(error_message) = check_quotes("Hello", 0) {
            panic!("The string should be OK: {}", error_message);
        }
    } // test_check_quotes()

    #[test]
    fn test_parse_term() {
        match parse_term(" $_ ") {
            Ok(term) => {
                if matches!(term, Unifiable::Anonymous) {
                    assert_eq!("$_", term.to_string());
                } else {
                    panic!("Should create an Anonymous variable: {}", term)
                }
            },
            Err(msg) => { panic!("{}", msg); },
        }
        match parse_term(" $X ") {
            Ok(term) => {
                if let Unifiable::LogicVar{id, name} = term {
                    assert_eq!(0, id);
                    assert_eq!("$X", name);
                } else {
                    panic!("Should create a LogicVar: {}", term)
                }
            },
            Err(msg) => { panic!("{}", msg); },
        }
        match parse_term(" $10 ") {
            Ok(term) => {
                if let Unifiable::Atom(s) = term {
                    assert_eq!("$10", s);
                } else {
                    panic!("Should create an Atom: {}", term)
                }
            },
            Err(msg) => { panic!("{}", msg); },
        }
        match parse_term(" verb ") {
            Ok(term) => {
                if matches!(term, Unifiable::Atom(_)) {
                    assert_eq!("verb", term.to_string());
                } else {
                    panic!("Should create an Atom: {}", term)
                }
            },
            Err(msg) => { panic!("{}", msg); },
        }
        match parse_term(" 1.7 ") {
            Ok(term) => {
                if matches!(term, Unifiable::SFloat(_)) {
                    assert_eq!("1.7", term.to_string());
                } else {
                    panic!("Should create an SFloat: {}", term)
                }
            },
            Err(msg) => { panic!("{}", msg); },
        }
        match parse_term(" 46 ") {
            Ok(term) => {
                if matches!(term, Unifiable::SInteger(_)) {
                    assert_eq!("46", term.to_string());
                } else {
                    panic!("Should create an SInteger: {}", term)
                }
            },
            Err(msg) => { panic!("{}", msg); },
        }
        match parse_term(" animal(horse, mammal) ") {
            Ok(term) => {
                if let Unifiable::SComplex(terms) = term {
                    assert_eq!("mammal", terms[2].to_string());
                } else {
                    panic!("Should create an SComplex: {}", term)
                }
            },
            Err(msg) => { panic!("{}", msg); },
        }
    } // test_parse_term()

    #[test]
    fn test_parse_arguments() {

        let terms_str = "8, 5.9, symptom, [], [a, b | $T], city(Toronto, 2.79)";

        match parse_arguments(terms_str) {
            Ok(terms) => {
                match terms[0] {
                    SInteger(i) => { assert_eq!(i, 8, "Incorrect integer."); },
                    _ => { panic!("Should create SInteger: {}", terms[0]); },
                };
                match terms[1] {
                    SFloat(f) => { assert_eq!(f, 5.9, "Incorrect float."); },
                    _ => { panic!("Should create SFloat: {}", terms[1]); },
                };
                match &terms[2] {
                    Atom(a) => { assert_eq!(a, "symptom", "Incorrect atom"); },
                    _ => { panic!("Should create an Atom: {}", terms[2]); },
                };
                match &terms[3] {
                    SLinkedList{term: _, next: _, count: c, tail_var: _} => {
                        let size: usize = 0;
                        assert_eq!(*c, size, "Should be empty list.");
                    },
                    _ => { panic!("Should create an empty list: {}", terms[3]); },
                };
                match &terms[4] {
                    SLinkedList{term: _, next: _, count: c, tail_var: _} => {
                        let size: usize = 3;
                        assert_eq!(*c, size, "Incorrect list size.");
                        let s = terms[4].to_string();
                        assert_eq!(s, "[a, b | $T]", "Incorrect list.");
                    },
                    _ => { panic!("Should create a list: {}", terms[4]); },
                };
                match &terms[5] {
                    SComplex(args) => {
                        let arity = args.len() - 1;  // exclude functor
                        assert_eq!(arity, 2, "Complex term, incorrect arity.");
                        let s = terms[5].to_string();
                        assert_eq!(s, "city(Toronto, 2.79)", "Incorrect complex term.");
                    },
                    _ => { panic!("Should create a complex term: {}", terms[5]); },
                };
            },
            Err(err) => { panic!("{}", err); },
        }

    } // test_parse_arguments()

} // test
