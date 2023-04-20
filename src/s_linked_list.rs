//! Functions to support Suiron lists.
//!
//! Suiron lists ([SLinkedList](../unifiable/enum.Unifiable.html#variant.SLinkedList))
//! are similar to Prolog lists. Some examples:
//!
//! <blockquote>
//! []<br>
//! [a, b, c, d]<br>
//! [a, b | $T]<br>
//! [$H | $T]<br>
//! </blockquote>
//!
//! Suiron lists are implemented as singly linked lists. Each element
//! (or node) contains a value (a [Unifiable](../unifiable/enum.Unifiable.html)
//! term) and a link to the next element. The last element of the list links
//! to Nil.
//!
//! A vertical bar (or pipe) |, is used to divide the list between head terms
//! and the tail, which is everything left over. When two lists are unified,
//! a tail variable binds to all the left-over terms in the other list. For
//! example, in the following code,
//!
//! <blockquote>
//!    [a, b, c, d, e] = [$X, $Y | $Z]
//! </blockquote>
//!
//! Variable $X binds to a.<br>
//! Variable $Y binds to b.<br>
//! Variable $Z binds to [c, d, e].
//!
// Cleve Lendon 2023

use std::rc::Rc;

use super::unifiable::{*, Unifiable::*};
use super::logic_var::*;
use super::parse_terms::*;
use super::substitution_set::*;

use crate::cons_node;
use crate::str_to_chars;
use crate::chars_to_string;

/// Makes a Suiron list, represented in source as: [a, b, c] or [a, b | $T]
///
/// The first argument, the vertical bar flag, is set true for lists
/// which have a tail variable, such as:<br>
/// <blockquote>
/// [a, b | $Tail]
/// </blockquote>
///
/// # Note
/// The macro [slist!](../macro.slist.html) can also be used to create lists.
///
/// # Arguments
/// * `vbar` - vertical bar flag |
/// * `terms` - vector of unifiable terms
/// # Return
/// `list` - ([SLinkedList](../unifiable/enum.Unifiable.html#variant.SLinkedList))
///
/// # Usage
/// To build [a, b, c], assuming that a, b and c are atoms:
///
/// <blockquote>
/// let terms = vec![a, b, c];<br>
/// let list1 = make_linked_list(false, terms);
/// </blockquote>
///
/// To build [a, b, c | $Tail]:
///
/// <blockquote>
/// let tail = logic_var!(1, "$Tail");<br>
/// let terms = vec![a, b, c, tail];<br>
/// let list2 = make_linked_list(true, terms);
/// </blockquote>
///
pub fn make_linked_list<'a>(vbar: bool, mut terms: Vec<Unifiable>) -> Unifiable {

    let mut node: Unifiable;

    // Tail of list. Start with an empty node.
    let mut tail = cons_node!(Nil, Nil, 0, false);

    let n_terms = terms.len();
    if n_terms == 0 { return tail; }  // Return empty list.

    let mut tail_var = vbar;   // Last variable is a tail variable.
    let mut num = 1;
    let last_index = n_terms - 1;

    let mut i = last_index;
    while i > 0 {

        node = terms.remove(i);
        if i == last_index {
            if let SLinkedList{term: t, next: n, count: c, tail_var: tf} = node {
                // If the last term is empty [], there is no need
                // to add it to the tail.
                if Nil == *t { tail = Nil; }
                else {
                    tail = cons_node!(*t, *n, c, tf);
                    num = c + 1;
                }
                tail_var = false;
                i -= 1;
                continue;
            }
            if node == Nil {
                i -= 1;
                continue;
            }
        }

        tail = cons_node!(node, tail, num, tail_var);
        num += 1;

        tail_var = false;
        i -= 1;

    } // while

    return cons_node!(terms.remove(0), tail, num, tail_var);

} // make_linked_list()

/// Compares two characters. Checks for backslash escapes: \\
///
/// If the character indexed in the vector of characters is the same as
/// the given character, the function will return true, except if the
/// indexed character is preceded by a backslash.
///
/// This function is necessary because characters which are escaped
/// by a preceding backslash should not be interpreted by the parser;
/// they need to be included unchanged, as they are.
///
/// For example, in Suiron source code, the complex term `msg(Hello, world!)`,
/// has two arguments, 'Hello' and 'world!', because the parser interprets the
/// comma as a separator. The complex term `msg(Hello\, world!)` has only
/// one argument: Hello, world!.
///
/// # Arguments
/// * vector of characters
/// * index of character
/// * character to compare
/// # Return
/// boolean - True if characters are the same. False otherwise
///
/// # Usage
/// <blockquote>
/// let text = str_to_chars!("OK, sure.");<br>
/// if equal_escape(text, 2, ',') { &nbsp;&nbsp;// returns true<br>
/// …
/// </blockquote>
///
/// <blockquote>
/// let text2 = str_to_chars!("OK\\, sure.");
/// &nbsp;&nbsp;// In Rust source, \\ represents one slash: \<br>
///
/// if equal_escape(text2, 3, ',') { &nbsp;&nbsp;// returns false<br>
/// …
/// </blockquote>
///
pub fn equal_escape(vec_chars: &Vec<char>, index: usize, ch: char) -> bool {
    if vec_chars[index] == ch {
        if index > 0 {
            if vec_chars[index - 1] == '\\' {
                return false;
            }
        }
        return true;
    }
    false
} // equal_escape()


/// Parses a string to create a linked list.
///
/// Returns an error if the string is invalid.
/// # Arguments
/// * string to parse
/// # Return
/// * [list](../unifiable/enum.Unifiable.html#variant.SLinkedList))
/// or error message
/// # Usage
/// <blockquote>
/// let list = parse_linked_list("[a, b, c | $X]");
/// </blockquote>
pub fn parse_linked_list(to_parse: &str) -> Result<Unifiable, String> {

    let s = to_parse.trim();

    let the_chars = str_to_chars!(s);
    let length_chars = the_chars.len();

    if length_chars < 2 {
        let err = pll_error("String is too short", s);
        return Err(err);
    }

    let first = the_chars[0];
    if first != '[' {
        let err = pll_error("Missing opening bracket [", s);
        return Err(err);
    }

    let last = the_chars[length_chars - 1];
    if last != ']' {
        let err = pll_error("Missing closing bracket ]", s);
        return Err(err);
    }

    let mut list = cons_node!(Nil, Nil, 0, false);  // Make an empty list.
    if length_chars == 2 { return Ok(list) }    // Return empty list.

    // remove brackets
    let arguments_chars: Vec<char> = the_chars[1..length_chars - 1].to_vec();
    let length_args = arguments_chars.len();

    let mut vbar = false;
    let mut end_index = length_args;

    let mut open_quote   = false;
    let mut num_quotes   = 0;
    let mut round_depth  = 0;   // depth of round parentheses (())
    let mut square_depth = 0;   // depth of square brackets [[]]

    let mut i = (length_args - 1) as i32;
    loop {

        let ind = i as usize;

        if open_quote {
            if equal_escape(&arguments_chars, ind, '"') {
                open_quote = false;
                num_quotes += 1;
            }
        }
        else {
            if equal_escape(&arguments_chars, ind, ']') {
                square_depth += 1;
            } else if equal_escape(&arguments_chars, ind, '[') {
                square_depth -= 1;
            } else if equal_escape(&arguments_chars, ind, ')') {
                 round_depth += 1;
            } else if equal_escape(&arguments_chars, ind, '(') {
                 round_depth -= 1;
            } else if round_depth == 0 && square_depth == 0 {
                if equal_escape(&arguments_chars, ind, '"') {
                    open_quote = true;  // first quote
                    num_quotes += 1;
                } else if equal_escape(&arguments_chars, ind, ',') {

                    let s2 = chars_to_string!(&arguments_chars[ind + 1..end_index]);
                    let s2 = s2.trim();

                    if s2.len() == 0 {
                        let err = pll_error("Missing argument", s);
                        return Err(err);
                    }
                    if let Some(error_message) = check_quotes(&s2, num_quotes) {
                        return Err(error_message);
                    }
                    match parse_term(s2) {
                        Ok(term) => {
                            list = link_front(term, false, list);
                            end_index = ind;
                        },
                        Err(err) => {
                            return Err(err);
                        }
                    }
                    num_quotes = 0;

                } else  // Must be a tail variable.
                if equal_escape(&arguments_chars, ind, '|') {
                    if vbar {
                        let err = pll_error("Too many vertical bars", s);
                        return Err(err);
                    }
                    let term_str =
                            chars_to_string!(&arguments_chars[ind + 1..end_index]);
                    let term_str2 = term_str.trim();
                    if term_str2.len() == 0 {
                        let err = pll_error("Missing argument", s);
                        return Err(err);
                    }

                    match make_logic_var(term_str2.to_string()) {
                        Err(_) => {
                            let err = pll_error(
                                     "Require variable after vertical bar", s);
                            return Err(err);
                        },
                        Ok(var) => {
                            list = link_front(var, true, list);
                        }
                    }
                    vbar = true;
                    end_index = ind;
                }  // if |
            }
        }  // else

        if ind == 0 {
            let s2 = chars_to_string!(&arguments_chars[0..end_index]);
            let s2 = s2.trim();

            if s2.len() == 0 {
                let err = pll_error("Missing argument", s);
                return Err(err);
            }
            if let Some(error_message) = check_quotes(&s2, num_quotes) {
                return Err(error_message);
            }
            match parse_term(s2) {
                Ok(term) => { return Ok(link_front(term, false, list)); },
                Err(err) => { return Err(err); }
            }
        } // if ind == 0 ...

        i -= 1;

    } // loop

} // parse_linked_list()

/// Adds a term to the front of the given linked list.
/// # Note
/// The tail variable flag is true when the term is tail variable.
/// # Arguments
/// * `new_term`
/// * `tail` - tail variable flag
/// * `list`
/// # Returns
/// * `new list`
fn link_front(new_term: Unifiable, tail: bool, list: Unifiable) -> Unifiable {

    if let SLinkedList{term: _, next: _, count, tail_var: _} = list {
        cons_node!(new_term, list, count + 1, tail)
    }
    else {
        panic!("link_front() - Third argument must be an SLinkedList.");
    }

} // link_front()

/// Counts the number of terms in a linked list.
///
/// The last term may be a tail variable, so this method requires the
/// substitution set.
///
/// Consider:
/// <pre>
/// $X = [a, b, c, d], $Y = [$H | $T], $Y = $X.
/// </pre>
///
/// After the above code runs, what is the number of terms in the list
/// assigned to $Y? There are two terms, $H and $T, so two may be a valid
/// answer. But $T is a tail variable, which unifies with the list [b, c, d].
/// In some contexts, it's more useful to count four.
///
/// This function will count terms of a linked list, including the terms
/// which the tail variable is bound to.
///
/// # Arguments
/// * [Unifiable](../unifiable/enum.Unifiable.html)
///   The Unifiable should be SLinkedList or a LogicVar linked to SLinkedList.
/// * [SubstitutionSet](../substitution_set/index.html)
/// # Return
/// * number of terms
pub fn count_terms(uni: &Unifiable, ss: &Rc<SubstitutionSet>) -> i64 {

    let mut count: i64 = 0;

    // If the first argument is a logic variable, get the ground term.
    let mut uni = uni;
    if let LogicVar{id: _, name: _} = uni {
        match get_ground_term(&uni, &ss) {
            Some(uni2) => { uni = uni2; },
            None => { return 1; },
        }
    }

    if let SLinkedList{term: t, next: n, count: _, tail_var: _} = uni {

        let mut head: &Unifiable = &*t;
        let mut slist: &Unifiable = &*n;

        while *head != Unifiable::Nil {
            count += 1;
            match get_list_data(slist) {
                Some((t, n, tv)) => {
                    head = t;
                    slist = n;
                    if tv && *head != Unifiable::Anonymous {
                        let list = get_list(&head, &ss);
                        match list {
                            Some(list) => {
                                if let SLinkedList{term, next,
                                        count: _, tail_var: _} = list {
                                    head = term;
                                    slist = next;
                                }
                            },
                            None => { return count; },
                        }
                    }
                }
                None => { return count; },
            } // match
        } // while

        return count;
    }
    else { return 1; }

} // count_terms()

/// Filters a Suiron list, to include or exclude terms which match a pattern.
///
/// # Arguments
/// * filter term - to match against
/// * term to filter - should be SLinkedList or a LogicVar bound to SLinkedList.
/// * [SubstitutionSet](../substitution_set/index.html)
/// * include flag - true = include, false = exclude
/// # Return
/// * new list
///
pub fn filter(filter: &Unifiable,
              uni: &Unifiable, ss: &Rc<SubstitutionSet>,
              include: bool) -> Option<Unifiable> {

    // If the first argument is a logic variable, get the ground term.
    let uni = get_ground_term(uni, ss)?;

    // If the unifiable is a linked list, filter it.
    if let SLinkedList{term: t, next: n, count: _, tail_var: _} = uni {

        let mut filtered_terms: Vec<Unifiable> = vec![];

        let mut head: &Unifiable = &*t;
        let mut slist: &Unifiable = &*n;

        while *head != Unifiable::Nil {

            if include {  // Include terms which match.
                if pass_filter(filter, head, ss) {
                    filtered_terms.push(head.clone());
                }
            }
            else {   // Must be exclude filter.
                // Opposite of above.
                if pass_filter(filter, head, ss) == false {
                    filtered_terms.push(head.clone());
                }
            }

            match get_list_data(slist) {
                Some((t, n, tv)) => {
                    head = t;
                    slist = n;
                    if tv && *head != Unifiable::Anonymous {
                        let list = get_list(&head, &ss);
                        match list {
                            Some(list) => {
                                if let SLinkedList{term, next,
                                       count: _, tail_var: _} = list {
                                    head = term;
                                    slist = next;
                                }
                            },
                            None => {},
                        } // match
                    }
                }
                None => { break; },
            } // match
        } // while

        let new_list = make_linked_list(false, filtered_terms);
        return Some(new_list);
    }
    return None;

} // filter

// Determines whether a term should pass the filter or be discarded.
//
// The function tests to see if the given term can be unified with
// the given filter term. If it can, the function returns true.
// False otherwise.
//
// # Arguments
// * filter term
// * term to test
// * substitution set
// # Return
// * bool - true if term can pass filter
fn pass_filter(filter: &Unifiable, term: &Unifiable, 
               ss: &Rc<SubstitutionSet>) -> bool {
    match filter.unify(term, ss) {
        Some(_) => { true },
        None => { false },
    }
}

// Gets the term, next link, and tail-variable flag from a list.
// # Arguments
// * list term
// # Return
// * (term, next, tail-var flag) or None
fn get_list_data(list: &Unifiable) -> Option<(&Unifiable, &Unifiable, bool)> {

    if let SLinkedList{term: t, next: n, count: _, tail_var: tv} = list {
        return Some((&*t, &*n, *tv));
    }
    return None;

} // get_list_data()

// Creates an error message for parse_linked_list() function.
// Arguments:
//   err - error description
//   bad - string which caused the error
// Return:
//   error message (String)
fn pll_error(err: &str, bad: &str) -> String {
    format!("parse_linked_list() - {}: {}", err, bad)
}

#[cfg(test)]
mod test {

    use crate::*;

    // pll - Formats an error message with prefix.
    // Reason: To shorten lines in the source.
    fn pll(err: &str) -> String { format!("parse_linked_list() - {}", err) }

    #[test]
    fn test_equal_escape() {
        let text = str_to_chars!("OK, sure.");
        if equal_escape(&text, 2, ',') {} // should return true
        else { panic!("Should match with comma."); }

        let text = str_to_chars!("OK\\, sure.");  // double backslash for Rust
        if equal_escape(&text, 3, ',') {  // should return false
            panic!("Should not match with comma, because comma is escaped.");
        }
    } // test_equal_escape()

    #[test]
    fn test_make_linked_list() {

        let a = SInteger(1);
        let b = SInteger(2);
        let c = SInteger(3);

        // Make empty list.
        let empty = make_linked_list(false, vec![]);
        if let SLinkedList{term: t, next: _, count: c, tail_var: _ } = empty {
            assert_eq!(c, 0, "Test 1 - Count should be 0 for empty list.");
            match *t {
                Nil => {},
                _ => { panic!("Test 1 - Term should be Nil for empty list."); },
            }
        }
        else { panic!("Test 1 - Did not produce list."); }

        // Make list no tail variable.
        let terms = vec![a, b, c];
        let list1 = make_linked_list(false, terms);
        if let SLinkedList{term: _, next: _, count: c, tail_var: _ } = list1 {
            assert_eq!(c, 3, "Test 2 - Count should be 3.");
            assert_eq!(list1.to_string(), "[1, 2, 3]", "Test 2 - Unexpected list.");
        }
        else { panic!("Test 2 - Did not produce list."); }

        // Make list with tail variable.
        let a = SInteger(1);
        let b = SInteger(2);
        let c = SInteger(3);
        let t = logic_var!("$T");
        let terms = vec![a, b, c, t];
        let list1 = make_linked_list(true, terms);
        if let SLinkedList{term: _, next: _, count: c, tail_var: _ } = list1 {
            assert_eq!(c, 4, "Test 3 - Count should be 4.");
            assert_eq!(list1.to_string(), "[1, 2, 3 | $T]",
                                          "Test 3 - Unexpected list.");
        }
        else { panic!("Test 3 - Did not produce list."); }
    }

    #[test]
    fn test_parse_linked_list() {

        // Check parsing errors.
        match parse_linked_list("a, b, c") {
            Err(msg) => {
                assert_eq!(msg, pll("Missing opening bracket [: a, b, c"));
            },
            _ => {},
        }

        match parse_linked_list("[a, b, c") {
            Err(msg) => {
                assert_eq!(msg, pll("Missing closing bracket ]: [a, b, c"));
            },
            _ => {},
        }

        match parse_linked_list("[a | b | $T]") {
            Err(msg) => {
                assert_eq!(msg, pll("Too many vertical bars: [a | b | $T]"));
            },
            _ => {},
        }

        match parse_linked_list("[a, b, ]") {
            Err(msg) => {
                assert_eq!(msg, pll("Missing argument: [a, b, ]"));
            },
            _ => {},
        }

        match parse_linked_list("[a, b | ]") {
            Err(msg) => {
                assert_eq!(msg, pll("Missing argument: [a, b | ]"));
            },
            _ => {},
        }

        match parse_linked_list("[, b | $T]") {
            Err(msg) => {
                assert_eq!(msg, pll("Missing argument: [, b | $T]"));
            },
            _ => {},
        }

        match parse_linked_list("[a, b | $T]") {
            Ok(list) => {
                if let Unifiable::SLinkedList{term: _, next: _,
                                              count: c, tail_var: _} = list {
                    assert_eq!(c, 3, "{}", pll("Count should be 3."));
                }
                else {
                    panic!("{}", pll("Should produce a list."));
                }
            },
            Err(msg) => {
                assert_eq!(msg, pll("Missing argument: [, b | $T]"));
            },
        }

    } // test_parse_linked_list()
}


/*
    Scan the list from head to tail,
    Curse recursion, force a fail.
    Hold your chin, hypothesize.
    Predicate logic never lies.
*/
