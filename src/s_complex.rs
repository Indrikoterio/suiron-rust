//! Functions to support complex terms.
//!
//! Suiron's complex terms
//! ([SComplex](../unifiable/enum.Unifiable.html#variant.SComplex))
//! are similar to Prolog's complex terms. They consist of a functor,
//! followed by a list of terms between parentheses. Some examples:
//!
//! <blockquote>
//! element(Argon, 18)<br>
//! father($F, Luke)<br>
//! pronoun(I, subject, first, singular)<br>
//! </blockquote>
//!
//! In Rust, complex terms are implemented as a vector of unifiable terms.
//!
//! # Note
//! Unlike Prolog, Suiron's
//! [atoms](../unifiable/enum.Unifiable.html#variant.Atom)
//! (string constants) can be capitalized or lower case.<br>
//! [Logic variables](../unifiable/enum.Unifiable.html#variant.LogicVar)
//! start with a dollar sign and a letter, eg. $F.<br>
//!
use super::unifiable::{*, Unifiable::*};
use super::goal::*;
use super::logic_var::*;
use super::parse_terms::*;
use super::parse_goals::*;

use crate::atom;
use crate::str_to_chars;
use crate::chars_to_string;

/// Produces a complex term from a vector of terms.
///
/// This function does validity checking. The first term must be an
/// [atom](../unifiable/enum.Unifiable.html#variant.Atom).
///
/// See also [scomplex!](../macro.scomplex.html).
///
/// # Arguments
/// * `terms` - vector of
/// [Unifiable](../unifiable/enum.Unifiable.html) terms
/// # Return
/// * [SComplex](../unifiable/enum.Unifiable.html#variant.SComplex)
/// # Panics
/// * If vector is empty.
/// * If first term (functor) is not an atom.
/// # Usage
/// ```
/// use suiron::*;
///
/// let d = atom!("drinks");
/// let p = atom!("Picard");
/// let t = atom!("Earl Grey");
///
/// let terms = vec![d, p, t];
/// let cmplx = make_complex(terms);
/// println!("{}", cmplx);  // Prints: drinks(Picard, Earl Grey)
/// ```
pub fn make_complex(terms: Vec<Unifiable>) -> Unifiable {
    if terms.len() == 0 { panic!("make_complex() - Vector is empty."); }
    let first = &terms[0];
    match first {
        Unifiable::Atom(_) => {},
        _ => { panic!("make_complex() - First term must be an Atom."); },
    }
    return Unifiable::SComplex(terms);
} // make_complex()


/// Produces a query from a vector of terms.
///
/// A query is the same as a complex term, but before it can be
/// submitted to a solver, its logic variables must be given unique IDs.
///
/// This function resets the global LOGIC_VAR_ID to 0, and recreates the
/// logic variable terms of the query.
///
/// # Arguments
/// * `terms` - vector of
/// [Unifiable](../unifiable/enum.Unifiable.html) terms
/// # Return
/// * [Goal](../goal/enum.Goal.html)
/// # Panics
/// * If vector is empty.
/// * If first term (functor) is not an atom.
/// # Usage
/// ```
/// use suiron::*;
///
/// let functor = atom!("loves");
/// let x = logic_var!("$X");
/// let y = logic_var!("$Y");
///
/// let terms = vec![functor, x, y];
/// let qry = make_query(terms);
/// println!("{}", qry);  // Prints: loves($X_1, $Y_2)
/// ```
pub fn make_query(terms: Vec<Unifiable>) -> Goal {

    // The main bottleneck in Suiron is the time it takes to copy
    // the substitution set. The substitution set is as large as
    // the highest variable ID (LOGIC_VAR_ID). Therefore LOGIC_VAR_ID
    // should be set to 0 for every query.
    clear_id();  // Reset LOGIC_VAR_ID.

    let mut new_terms: Vec<Unifiable> = vec![];
    let mut vars = VarMap::new();

    for term in terms {
        new_terms.push(term.recreate_variables(&mut vars));
    }

    let c = make_complex(new_terms);
    Goal::ComplexGoal(c)

} // make_query()

/// Parses a string to produce a complex term.
///
/// # Arguments
/// * `to_parse` - &str
/// # Return
/// * `Result` -
/// Ok([SComplex](../unifiable/enum.Unifiable.html#variant.SComplex))
/// or Err(message)
///
/// # Usage
/// ```
/// use suiron::*;
///
/// let cmplx = parse_complex("symptom(Covid, fever)");
/// match cmplx {
///    Ok(c) => { println!("{}", c); },
///    Err(err) => { println!("{}", err); },
/// }
/// // Prints: symptom(Covid, fever)
/// ```
///
/// # Note
/// Backslash is used to escape characters, such as the comma.
/// For example:<br>
/// ```
/// use suiron::*;
///
/// let cmplx = parse_complex("punctuation(comma, \\,)");
/// match cmplx {
///     Ok(c) => { println!("{}", c); },
///     Err(err) => { println!("{}", err); },
/// }
/// // Prints: punctuation(comma, ,)
/// ```
///
/// The backslash is doubled because the Rust compiler also
/// interprets the backslash.
///
pub fn parse_complex(to_parse: &str) -> Result<Unifiable, String> {

    let s = to_parse.trim();
    let chrs = str_to_chars!(s);

    match validate_complex(s, &chrs) {
        Some(msg) => { return Err(msg); },
        None => {},
    }

    // Get indices, if any.
    match indices_of_parentheses(&chrs) {
        Ok(indices) => {
            match indices {
                Some((left, right)) => {
                    let functor = chars_to_string!(chrs[0..left]);
                    let args    = chars_to_string!(chrs[left + 1..right]);
                    match parse_functor_terms(&functor, &args) {
                        Ok(cmplx) => { return Ok(cmplx); }, // OK, return.
                        Err(err) => { // Adjust error message. Add original string.
                            let err = format!("{}{}", err, to_parse);
                            return Err(err);
                        },
                    }
                },
                None => {
                    match parse_functor_terms(s, "") {
                        Ok(cmplx) => { return Ok(cmplx); }, // OK, return.
                        Err(err) => { // Adjust error message. Add original string.
                            let err = format!("{} {}", err, to_parse);
                            return Err(err);
                        },
                    }
                },
            }
        },
        Err(err) => { return Err(err); }
    }
} // parse_complex


/// Validates the string to be parsed.
///
/// If the string to parse is empty or too long, this function
/// will return an error message. Also, the string must not
/// begin with a dollar sign or a parenthesis.
///
/// # Argument
/// * `to_parse` - &str
/// * `chars` - vector of chars
/// # Return
/// * `Option` - Some(error_message) or None
pub fn validate_complex(to_parse: &str, chrs: &Vec<char>) -> Option<String> {

    let length = chrs.len();
    if length == 0 {
        let err = format!("Parsing error - Length of string is 0.");
        return Some(err);
    }

    if length > 1000 {
        let err = format!("Parsing error - String is too long.\n{}", to_parse);
        return Some(err);
    }

    let first = chrs[0];
    if first == '$' || first == '(' {
        let err = format!("Parsing error - First character is invalid: {}", to_parse);
        return Some(err);
    }
    return None;

} // validate_complex()


/// Parses a string to produce a query.
///
/// Queries are complex terms entered after the query prompt, ?-. For example:<br>
///
/// ?- `symptom(flu, $Sym).`<br>
///
/// A query is the same as a complex term, but before it can be
/// submitted to a solver, its logic variables must be given unique IDs.
///
/// This function simply calls parse_complex(), then recreates the
/// logic variables.
///
/// # Arguments
/// * `to_parse` - string to parse
/// # Return
/// * `Result` -
/// Ok([Goals](../goal/enum.Goal.html))
/// or Err(message)
///
/// # Usage
/// ```
/// use suiron::*;
///
/// match parse_query("loves($X, $Y)") {
///     Ok(q) => { println!("{}", q); },
///     Err(err) => { println!("{}", err); },
/// }
/// // Prints: loves($X_1, $Y_2)
/// ```
///
/// # Note
/// Backslash is used to escape characters, such as the comma.
/// For example:<br>
/// ```
/// use suiron::*;
///
/// let cmplx = parse_complex("punctuation(comma, \\,)");
/// match cmplx {
///     Ok(c) => { println!("{}", c); },
///     Err(err) => { println!("{}", err); },
/// }
/// // Prints: punctuation(comma, ,)
/// ```
///
/// The backslash is doubled because the Rust compiler also
/// interprets the backslash.
///
pub fn parse_query(to_parse: &str) -> Result<Goal, String> {

    // Clean up query.
    // Perhaps there is an unnecessary period at the end.
    let mut parse2 = to_parse.to_string();
    let ch = parse2.chars().last().unwrap();
    if ch == '.' { parse2.pop(); }

    match parse_complex(&parse2) {
        Ok(q) => {
            match q {
                Unifiable::SComplex(terms) => {
                    let query = make_query(terms);
                    return Ok(query);
                },
                _ => { panic!("parse_query() - Cannot happen.") },
            }
        },
        Err(err) => { Err(err) },
    }
} // parse_query()

/// Parses two string arguments to produce a complex term.
///
/// # Arguments
/// * `functor` - string
/// * `list of terms` - string
/// # Return
/// * `Result` -
/// Ok([SComplex](../unifiable/enum.Unifiable.html#variant.SComplex))
/// or Err(message)
/// # Usage
/// ```
/// use suiron::*;
///
/// let cmplx = parse_functor_terms("father", "Anakin, Luke");
/// match cmplx {
///     Ok(c) => { println!("{}", c); },
///     Err(err) => { println!("{}", err); },
/// }
/// // Prints: Anakin, Luke
/// ```
pub fn parse_functor_terms(functor: &str, terms: &str) -> Result<Unifiable, String> {

    let mut new_terms = vec![atom!(functor.trim())];
    if terms == "" {
        return Ok(SComplex(new_terms));
    }

    match parse_arguments(terms) {
        Ok(terms) => {
            for term in terms { new_terms.push(term); }
            return Ok(SComplex(new_terms));
        },
        Err(err) => { return Err(err); },
    }

} // parse_functor_terms


#[cfg(test)]
mod test {

    use crate::*;
    use super::*;

    /// Validate complex.
    /// String must not be 0 length, or start with: $ (
    #[test]
    fn test_validate_complex() {
        let s = "aaaaaaaa";
        let chrs = str_to_chars!(s);
        match validate_complex(s, &chrs) {
            Some(err) => {
                let err = format!("validate_complex() - Should not produce error: {}", err);
                panic!("{}", err);
            },
            None => {},
        }
        let s = "";
        let chrs = str_to_chars!(s);
        match validate_complex(s, &chrs) {
            Some(err) => {
                assert_eq!(err, "Parsing error - Length of string is 0.");
            },
            None => { panic!("validate_complex() - Should generate error."); },
        }
        let s = "$aaaa";
        let chrs = str_to_chars!(s);
        match validate_complex(s, &chrs) {
            Some(err) => {
                assert_eq!(err, "Parsing error - First character is invalid: $aaaa");
            },
            None => { panic!("validate_complex() - Should generate error."); },
        }
    } // test_validate_complex()

    /// Tests creation of a complex term from a vector of terms.
    #[test]
    fn test_make_complex() {
        let d = atom!("drinks");
        let p = atom!("Picard");
        let t = atom!("Earl Grey");
        let terms = vec![d, p, t];
        let cmplx = make_complex(terms);
        let s = format!("{}", cmplx);
        assert_eq!(s, "drinks(Picard, Earl Grey)");
    } // test_make_complex()


    /// Tests creation of a query from a vector of terms.
    #[test]
    fn test_make_query() {
        let functor = atom!("loves");
        let x = logic_var!("$X");
        let y = logic_var!("$Y");
        let terms = vec![functor, x, y];
        let qry = make_query(terms);
        assert_eq!(qry.to_string(), "loves($X_1, $Y_2)");
    } // test_make_query

    /// make_complex() - Panics if the vector is empty.
    #[test]
    #[should_panic]
    fn test_make_complex_panic1() {
        let terms = vec![];
        make_complex(terms);
    }

    /// make_complex() - Panics if the first term is not an Atom.
    #[test]
    #[should_panic]
    fn test_make_complex_panic2() {
        let d = SInteger(1);
        let p = atom!("Picard");
        let t = atom!("Earl Grey");
        let terms = vec![d, p, t];
        make_complex(terms);
    }

    /// parse_functor_terms() should create a valid complex term.
    #[test]
    fn test_parse_functor_terms() {
        match parse_functor_terms("father", "Anakin, Luke") {
            Ok(c) => {
                if matches!(c, Unifiable::SComplex(_)) {
                    assert_eq!("father(Anakin, Luke)", c.to_string());
                } else {
                    panic!("parse_functor_terms() - \
                            Should create a complex term: {}", c)
                }
            },
            Err(err) => { panic!("{}", err); }
        }
    } // test_parse_functor_terms()

    /// Test parse_complex() - Test parsing and errors.
    #[test]
    fn test_parse_complex() {
        match parse_complex("father(Anakin, Luke)") {
            Ok(c) => {
                if matches!(c, Unifiable::SComplex(_)) {
                    assert_eq!("father(Anakin, Luke)", c.to_string());
                } else {
                    panic!("parse_complex() - Should create a complex term: {}", c);
                }
            },
            Err(err) => { panic!("{}", err); },
        }
        match parse_complex("punctuation(comma, \\,)") {
            Ok(c) => {
                if matches!(c, Unifiable::SComplex(_)) {
                    assert_eq!("punctuation(comma, ,)", c.to_string());
                } else {
                    panic!("parse_complex() - Should create a complex term: {}", c);
                }
            },
            Err(err) => { panic!("{}", err); }
        }
        match parse_complex("father(, Luke)") {
            Ok(c) => {
                panic!("parse_complex() - Should generate error: {}", c)
            },
            Err(err) => {
                assert_eq!(err, "parse_arguments() - Missing first argument: father(, Luke)");
            },
        }
        match parse_complex("father(Anakin,)") {
            Ok(c) => {
                panic!("parse_complex() - Should generate error: {}", c)
            },
            Err(err) => {
                assert_eq!(err, "parse_arguments() - Missing last argument: father(Anakin,)");
            },
        }
        match parse_complex("father") {
            Ok(c) => {
                // Displayed with empty parentheses.
                assert_eq!("father()", c.to_string());
            },
            Err(err) => {
                panic!("parse_complex() - Should create a complex term: {}", err);
            },
        }
    } // test_parse_complex()

} // test
