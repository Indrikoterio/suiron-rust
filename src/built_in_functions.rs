//! Functions to support Suiron's built-in functions (SFunction).
//!

use std::rc::Rc;
use super::s_complex::*;
use super::parse_terms::*;
use super::parse_goals::*;
use super::substitution_set::*;
use super::unifiable::Unifiable;
use super::built_in_join::*;
use super::built_in_arithmetic::*;

use crate::str_to_chars;
use crate::chars_to_string;

/// Unify an SFunction with another Unifiable term.
///
/// When the function [unify()](../unifiable/enum.Unifiable.html#method.unify)
/// tries to unify an 
/// [SFunction](../unifiable/enum.Unifiable.html#variant.SFunction)
/// with another term, it calls `unify_sfunction()`.
/// This function evaluates its arguments (`terms`), and unifies the result
/// with the `other` [Unifiable](../unifiable/enum.Unifiable.html) term.
/// 
/// # Arguments
/// * name of function
/// * terms - vector of Unifiable terms
/// * other Unifiable term
/// * [SubstitutionSet](../substitution_set/index.html)
/// # Returns
/// * [SubstitutionSet](../substitution_set/index.html) or None.
/// # Usage
/// ```
/// use std::rc::Rc;
/// use suiron::*;
///
/// // Try:  $X = add(1, 2, 3)
/// let x = logic_var!(next_id(), "$X");
/// let ss = empty_ss!();
/// let terms = vec![SInteger(1), SInteger(2), SInteger(3)];
///
/// match unify_sfunction("add", &terms, &x, &ss) {
///     Some(ss) => {
///         let res = get_ground_term(&x, &ss).unwrap();
///         println!("{}", res);
///     },
///     None => { println!("No solution."); },
/// }
/// // Should print: 6
/// ```
///
pub fn unify_sfunction<'a>(name: &str, terms: &'a Vec<Unifiable>,
                           other: &'a Unifiable, ss: &'a Rc<SubstitutionSet<'a>>)
                           -> Option<Rc<SubstitutionSet<'a>>> {

    let name = name.to_string();

    if name.eq("join") {
        let result = evaluate_join(terms, ss);
        return result.unify(other, ss);
    }
    else if name.eq("add") {
        let result = evaluate_add(terms, ss);
        return result.unify(other, ss);
    }
    else if name.eq("subtract") {
        let result = evaluate_subtract(terms, ss);
        return result.unify(other, ss);
    }
    else if name.eq("multiply") {
        let result = evaluate_multiply(terms, ss);
        return result.unify(other, ss);
    }
    else if name.eq("divide") {
        let result = evaluate_divide(terms, ss);
        return result.unify(other, ss);
    }

    return None;

} // unify_sfunction()

/// Parses a string to produce a built-in function (SFunction).
///
/// # Note
/// * parse_function() is very similar to
/// [parse_complex()](../s_complex/fn.parse_complex.html).
/// # Arguments
/// * string to parse
/// # Returns
/// * [SFunction](../unifiable/enum.Unifiable.html#variant.SFunction)
/// or error message
/// # Usage
/// ```
/// use suiron::*;
///
/// match parse_function("add(7, 9, 4)") {
///     Ok(term) => { println!("{}", term); },
///     Err(msg) => { println!("{}", msg); },
/// }
/// ```
/// Should print: add(7, 9, 4)
pub fn parse_function(to_parse: &str) -> Result<Unifiable, String> {

    let s = to_parse.trim();
    let chrs = str_to_chars!(s);

    // Built-in functions have the same form as complex terms.
    // That is: name(term1, term2...)
    match validate_complex(s, &chrs) {
        Some(error_message) => { return Err(error_message); },
        None => {},
    }

    // Get indices.
    match indices_of_parentheses(&chrs) {
        Ok(indices) => {
            match indices {
                Some((left, right)) => {

                    let name = chars_to_string!(chrs[0..left]);
                    let terms_str = chars_to_string!(chrs[left + 1..right]);
                    if terms_str.len() == 0 {
                        let err = format!("parse_function - No arguments: {}", s);
                        return Err(err);
                    }

                    let mut new_terms: Vec<Unifiable> = vec![];
                    match parse_arguments(&terms_str) {
                        Ok(terms) => {
                            // If everything parsed correctly, return SFunction.
                            for term in terms { new_terms.push(term); }
                            return Ok(Unifiable::SFunction{name, terms: new_terms});
                        },
                        Err(err) => {
                            let err = format!("{}{}", err, s);
                            return Err(err);
                        },
                    }
                },
                None => {
                    let err = format!("parse_function() - Invalid function: {}", s);
                    return Err(err);
                },
            } // match
        },
        Err(err) => { return Err(err); }
    } // match

} // parse_function()


#[cfg(test)]
mod test {

    use crate::*;
    use super::*;

    #[test]
    fn test_parse_function() {
        match parse_function("add(5, 6, 7)") {
            Ok(sf) => {
                if matches!(sf, Unifiable::SFunction{name: _, terms: _}) {
                    assert_eq!("add(5, 6, 7)", sf.to_string());
                } else {
                    panic!("parse_function() - \
                            Should create a function: {}", sf);
                }
            },
            Err(err) => { panic!("{}", err); },
        }
        match parse_function("add") {
            Ok(c) => {
                panic!("parse_function() - Should generate error: {}", c)
            },
            Err(err) => {
                assert_eq!(err, "parse_function() - Invalid function: add");
            },
        }
        match parse_function("add(, 6, 7)") {
            Ok(c) => {
                panic!("parse_function() - Should generate error: {}", c)
            },
            Err(err) => {
                assert_eq!(err, "parse_arguments() - \
                                 Missing first argument: add(, 6, 7)");
            },
        }
        match parse_function("add(5, 6, )") {
            Ok(c) => {
                panic!("parse_function() - Should generate error: {}", c)
            },
            Err(err) => {
                assert_eq!(err, "parse_arguments() - \
                                 Missing last argument: add(5, 6, )");
            },
        }
        match parse_function("add()") {
            Ok(c) => {
                panic!("parse_function() - Should generate error: {}", c)
            },
            Err(err) => {
                assert_eq!(err, "parse_function - No arguments: add()");
            },
        }
    } // test_parse_function()

} // test
