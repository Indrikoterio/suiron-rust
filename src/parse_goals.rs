//! Utilities for parsing goals and queries.
//!

use crate::atom;
use crate::str_to_chars;
use crate::chars_to_string;

use super::goal::*;
use super::infix::*;
use super::operator::*;
use super::s_complex::*;
use super::parse_terms::*;
use super::unifiable::Unifiable;
use super::built_in_predicates::*;

/// Determines the indices of parentheses in a goal or query.
///
/// For example, in the goal `parse($In, $Out)`, the indices are (5, 15).
///
/// This function also checks for errors, such as unmatched parentheses.
///
/// # Arguments
/// * vector of chars, representing a goal
/// # Return
/// * (left_index, right_index) or error message
/// # Usage
/// ```
/// use suiron::*;
///
/// let element_chr = str_to_chars!("element(Iridium, 77)");
/// match indices_of_parentheses(&element_chr) {
///     Ok(ind) => {
///         match ind {
///             Some((left, right)) => { println!("Indices: {}, {}", left, right); },
///             None => { println!("No indices were found."); },
///         } // match
///     },
///     Err(msg) => { println!("{}", msg); },
/// } // match
/// // Should print: Indices: 7, 19
/// ```
///
pub fn indices_of_parentheses(goal: &Vec<char>)
                              -> Result<Option<(usize, usize)>, String> {

    let mut left: i32   = -1;  // index of first parenthesis
    let mut right: i32  = -1;
    let mut count_left  = 0;
    let mut count_right = 0;

    for (i, ch) in goal.iter().enumerate() {
        if *ch == '(' {
            if left == -1 { left = i as i32; }
            count_left += 1;
        }
        else if *ch == ')' {
            right = i as i32;
            count_right += 1;
        }
    } // for

    if count_left != count_right {
        let s = chars_to_string!(goal);
        return Err(iop_error("Unbalanced parentheses", &s));
    }

    if right < left {
        let s = chars_to_string!(goal);
        return Err(iop_error("Invalid parentheses", &s));
    }

    if left == -1 { return Ok(None); }
    return Ok(Some((left as usize, right as usize)));

} // indices_of_parentheses


/// Gets terms on left and right-hand side of an infix.
///
/// This function divides a string (vector of characters) which contains
/// an infix,<br> such as `$X = verb` or `$X <= 47`.
/// It parses the left and right sides, to produce<br>two Unifiable terms.
///
/// # Arguments
/// * vector of chars
/// * index of infix
/// * size of infix (1 or 2)
/// # Return
/// * (Unifiable, Unifiable) or error message
/// # Usage
/// ```
/// use suiron::*;
///
/// let chrs = str_to_chars!("$X < 7");
/// let (_infix, index) = check_infix(&chrs);
///
/// match get_left_and_right(chrs, index, 2) {
///     Ok((left, right)) => { println!("Left: {left}, Right: {right}");},
///     Err(err) => { println!("Error: {err}"); },
/// };
/// // Prints - Left: $X, Right: 7
/// ```
pub fn get_left_and_right(chrs: Vec<char>, index: usize, size: usize)
                          -> Result<(Unifiable, Unifiable), String> {
    let arg1 = &chrs[0..index];
    let arg2 = &chrs[index + size..];

    let term1 = parse_term(&chars_to_string!(arg1))?;
    let term2 = parse_term(&chars_to_string!(arg2))?;
    return Ok((term1, term2));

} // get_left_and_right


/// Splits a string representation of a complex term into its functor and terms.
///
/// For example, if the complex term is:
/// <blockquote>
///    father(Philip, Alize)
/// </blockquote>
/// and the indices (index1, index2) are 6 and 20, the function will
/// return: "father", "Philip, Alize"
///
/// This method assumes that index1 and index2 are valid.
///
/// # Arguments
/// * `complex` - term (string)
/// * `index1` - index of left parenthesis
/// * `index2` - index of right parenthesis
/// # Return
/// * `(String, String)`
///
fn split_complex_term(complex: Vec<char>, index1: usize, index2: usize)
                      -> (String, String) {

      let functor = &complex[0..index1];
      let terms   = &complex[index1 + 1..index2];
      return (chars_to_string!(functor), chars_to_string!(terms));

} // split_complex_term


/// Parses a string to produce a goal.
///
/// This function parses strings which represent complex terms, such
/// as `element(Xenon, $N, $W)`, and built-in predicates, such as
/// `append(…)`, to produce `ComplexGoal`s and `BuiltInGoal`s.
///
/// # Arguments
/// * string to parse
/// # Result
/// * [Goal](../goal/enum.Goal.html) or error message
///
/// # Note
/// * This function does not parse And or Or operators. See
/// [generate_goal()](../tokenizer/fn.generate_goal.html)
/// * The Not and Time operators are dealt with first, because they
/// enclose subgoals. Eg.
///    not($X = $Y)
///    time(qsort)
///
pub fn parse_subgoal(to_parse: &str) -> Result<Goal, String> {

    let s = to_parse.trim();

    if s.len() == 0 {
        let err = "parse_subgoal() - Empty string.".to_string();
        return Err(err);
    }

    let chrs = str_to_chars!(s);

    // Built-in predicates with no arguments.
    if s == "!" || s == "fail" || s == "nl" {
        let pred = BuiltInPredicate::new(s.to_string(), None);
        return Ok(Goal::BuiltInGoal(pred));
    }

    //--------------------------------------
    // Handle infixes: = > < >= <= == =

    let (infix, index) = check_infix(&chrs);
    if infix != Infix::None {

        // An infix can be 1 or 2 characters, eg: <, <=
        // The last parameter of get_left_and_right() is the
        // size of the infix. To avoid repeating this call
        // for each infix, it is called here with an infix size
        // of 2. Since all infixes must be followed by a space,
        // this shouldn't be a problem.
        let (left, right) = get_left_and_right(chrs, index, 2)?;

        let v = Some(vec![left, right]);

        let pred = match infix {
            Infix::Unify => {
                let functor = "unify".to_string();
                BuiltInPredicate::new(functor, v)
            },
            Infix::Equal => {
                let functor = "equal".to_string();
                BuiltInPredicate::new(functor, v)
            },
            Infix::LessThan => {
                let functor = "less_than".to_string();
                BuiltInPredicate::new(functor, v)
            },
            Infix::LessThanOrEqual => {
                let functor = "less_than_or_equal".to_string();
                BuiltInPredicate::new(functor, v)
            },
            Infix::GreaterThan => {
                let functor = "greater_than".to_string();
                BuiltInPredicate::new(functor, v)
            },
            Infix::GreaterThanOrEqual => {
                let functor = "greater_than_or_equal".to_string();
                BuiltInPredicate::new(functor, v)
            },
            _ => {
                let err = format!("parse_subgoal() - Invalid syntax: {}", s);
                return Err(err);
            },
        }; // let match

        return Ok(Goal::BuiltInGoal(pred));

    } // if infix != Infix::None

    // Check for parentheses.
    let left_index: usize;
    let right_index: usize;
    match indices_of_parentheses(&chrs) {
        Ok(indices) => {
            match indices {
                Some((l, r)) => { left_index = l; right_index = r; },
                None => {
                    // OK. A goal can be a simple word, without parentheses.
                    match parse_functor_terms(s, "") {
                        Ok(c) => { return Ok(Goal::ComplexGoal(c)); },
                        Err(err) => {
                            let err = format!("{}{}", err, to_parse);
                            return Err(err);
                        },
                    }
                },
            } // match
        },
        Err(err) => { return Err(err); },
    }

    let (functor_str, args_str) =
                     split_complex_term(chrs, left_index, right_index);

    // Check for operators.
    if functor_str == "time" || functor_str == "not"{
       return make_operator_goal(&functor_str, &args_str);
    }

    return make_goal(&functor_str, &args_str);

} // parse_subgoal


/// Makes a goal from a functor and a vector of unifiable terms.
///
/// Complex terms and built-in predicates have the form: `functor(term1, term2...)`
/// If the given functor represents a built-in predicate, such as print() or
/// append(), this function will construct the predicate and wrap it in
/// Goal::BuiltInGoal(). Otherwise, the function will construct a complex term,
/// and wrap it in Goal::ComplexGoal().
///
/// # Arguments
/// * functor (string)
/// * arguments / terms (string)
/// # Result
/// * [Goal](../goal/enum.Goal.html)
/// # Usage
/// ```
/// use suiron::*;
///
/// let args = "3.14159, [A, B, C], 6, $Out";
/// let append_pred = match make_goal("append", args) {
///     Ok(goal) => { println!("{}", &goal); },
///     Err(err) => { panic!("{}", err); },
/// };
/// // Prints: append(3.14159, [A, B, C], 6, $Out)
/// ```
pub fn make_goal(functor: &str, args_str: &str) -> Result<Goal, String> {

    let mut args = parse_arguments(args_str)?;

    if functor == "print" || functor == "append" || functor == "functor" ||
       functor == "include" || functor == "exclude" ||
       functor == "print_list" || functor == "unify" || functor == "equal" ||
       functor == "less_than"    || functor == "less_than_or_equal" ||
       functor == "greater_than" || functor == "greater_than_or_equal" {
        let pred = BuiltInPredicate::new(functor.to_string(), Some(args));
        return Ok(Goal::BuiltInGoal(pred));
    }

    // Create a complex term.
    let mut unifiables = vec![atom!(functor)];
    unifiables.append(&mut args);
    return Ok(Goal::ComplexGoal(Unifiable::SComplex(unifiables)));

} // make_goal()

/// Makes a operator goal for the given name and argument.
///
/// A built-in predicate or complex term holds a vectors of unifiable terms.
/// An operator, on the other hand, holds a vector of goals, so it must be
/// handled separately.
///
/// # Arguments
/// * name of operator
/// * argument string
/// # Return
/// * operator goal or error message
///
fn make_operator_goal(name: &str, args_str: &str) -> Result<Goal, String> {
    let subgoal = parse_subgoal(&args_str)?;
    match name {
        "time" => {
            return Ok(Goal::OperatorGoal(Operator::Time(vec![subgoal])));
        },
        "not" => {
            return Ok(Goal::OperatorGoal(Operator::Not(vec![subgoal])));
        },
        _ => {
           let err = "make_operator_goal() - Invalid operator.".to_string();
           return Err(err)
        },
    }
} // make_operator_goal()

// Formats an error message for indices_of_parentheses().
// Arguments:
//   err - error description
//   bad - string which caused the error
// Return:
//   error message (String)
fn iop_error(err: &str, bad: &str) -> String {
    format!("indices_of_parentheses() - {}: {}", err, bad)
}

#[cfg(test)]
mod test {

    use crate::str_to_chars;

    use super::*;

    #[test]
    fn test_indices_of_parentheses() {

        let goal_chr = str_to_chars!("parse($In, $Out)");
        match indices_of_parentheses(&goal_chr) {
            Ok(indices) => {
                match indices {
                    Some(ind) => { assert_eq!((5, 15), ind, "Incorrect indices."); },
                    None => { panic!("Could not get indices."); },
                }
            },
            Err(err) => { panic!("{err}"); }
        }

        let goal_chr = str_to_chars!("parse");
        match indices_of_parentheses(&goal_chr) {
            Ok(indices) => {
                match indices {
                    Some(_) => { panic!("Should not find indices."); },
                    None => {}, // None found.
                }
            },
            Err(msg) => { panic!("{}", msg); }
        }

        let goal_chr = str_to_chars!("parse($In, $Out");
        let err = "indices_of_parentheses() - Unbalanced parentheses: parse($In, $Out";
        match indices_of_parentheses(&goal_chr) {
            Ok(_) => { panic!("Should produce error message."); },
            Err(msg) => { assert_eq!(err, msg, "Unexpected error message."); }
        }

        let goal_chr = str_to_chars!("parse)$In, $Out(");
        let err = "indices_of_parentheses() - Invalid parentheses: parse)$In, $Out(";
        match indices_of_parentheses(&goal_chr) {
            Ok(_) => { panic!("Should produce error message."); },
            Err(msg) => { assert_eq!(err, msg, "Unexpected error message."); }
        }
    } // test_indices_of_parentheses()

    #[test]
    fn test_get_left_and_right() {

        let chrs = str_to_chars!("$X < 7");
        let (_inf, ind) = check_infix(&chrs);

        let (left, right) = match get_left_and_right(chrs, ind, 1) {
            Ok((left, right)) => (left, right),
            Err(_err) => { panic!("get_left_and_right() - Should not fail."); }
        };
        assert_eq!("$X", left.to_string());
        assert_eq!("7", right.to_string());

    } // test_get_left_and_right()

} // test
