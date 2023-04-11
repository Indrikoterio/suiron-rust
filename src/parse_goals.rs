//! Utilities for parsing goals and queries.
//!

use crate::str_to_chars;
use crate::chars_to_string;

use super::goal::*;
use super::operator::*;
use super::s_complex::*;
use super::parse_terms::*;
use super::unifiable::Unifiable;
use super::built_in_predicates::*;

//-----------Infixes-----------
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Infix {
    None,
    /// = Unification operator.
    Unify,
    /// == Equal. No unification. Simply compares.
    Equal,
    /// &gt;
    GreaterThan,
    /// &lt;
    LessThan,
    /// &gt;=
    GreaterThanOrEqual,
    /// &lt;=
    LessThanOrEqual,
    /// &plus;
    Add,
    /// &minus;
    Subtract,
    /// &#42;
    Multiply,
    /// &#47;
    Divide,
}

/// Determines the indices of parentheses in a goal or query.
///
/// For example, in the goal `parse($In, $Out)`, the indices are (5, 15).
///
/// This function also checks for errors, such as unmatched parentheses.
///
/// # Arguments
/// * `goal` - vector of chars
/// # Return
/// * `Result` - Ok(Option((left, right))) or Err(message)
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
pub fn indices_of_parentheses(goal: &Vec<char>) -> Result<Option<(usize, usize)>, String> {

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


/// Determines whether a string contains an infix: >=, ==, etc.
///
/// This function returns the type and index of the
/// [infix](../parse_goals/enum.Infix.html).
/// For example,<br>
/// <blockquote>
///    $X < 6
/// </blockquote>
/// ...contains the `LessThan` infix, at index 3.
///
/// # Arguments
/// * `chrs` - vector of chars
/// # Return
/// * `(infix, index)` - [Infix](../parse_goals/enum.Infix.html)
///
/// # Note
/// * An infix must be preceded and followed by a space. Don't do this:  $X<6
/// * The function ignores characters between double quotes and parentheses.<br>
/// For example, for the the string of characters \" <= \" (double quotes included),
/// the function will return (Infix::None, 0).
///
pub fn identify_infix(chrs: &Vec<char>) -> (Infix, usize) {

    let length = chrs.len();
    let mut prev   = '#';  // not a space

    let mut i = 0;
    while i < length {

        // Skip past quoted text: ">>>>>"
        let c1 = chrs[i];
        if c1 == '"' {
            let mut j = i + 1;
            while j < length  {
                let c2 = chrs[j];
                if c2 == '"' {
                    i = j;
                    break;
                }
                j += 1;
            }
        }
        else if c1 == '(' {
            // Skip past text within parentheses: (...)
            let mut j = i + 1;
            while j < length {
                let c2 = chrs[j];
                if c2 == ')' {
                    i = j;
                    break;
                }
                j += 1;
            }
        }
        else {
            // Previous character must be space.
            if prev != ' ' {
                prev = c1;
                i += 1;
                continue;
            }
            // Bad:  $X =1
            // Good: $X = 1
            if i >= (length - 2) { return (Infix::None, 0); }
            if c1 == '<' {
                let c2 = chrs[i + 1];
                if c2 == '=' {
                    // Bad:  $X <=1
                    // Good: $X <= 1
                    if i >= length - 3 { return (Infix::None, 0); }
                    let c3 = chrs[i + 2];
                    if c3 == ' ' {
                        return (Infix::LessThanOrEqual, i);
                    }
                }
                else if c2 == ' ' {
                    return (Infix::LessThan, i);
                }
            }
            else if c1 == '>' {
                let c2 = chrs[i + 1];
                if c2 == '=' {
                    // Bad:  $X >=1
                    // Good: $X >= 1
                    if i >= length - 3 { return (Infix::None, 0); }
                    let c3 = chrs[i + 2];
                    if c3 == ' ' {
                        return (Infix::GreaterThanOrEqual, i);
                    }
                }
                else if c2 == ' ' {
                    return (Infix::GreaterThan, i);
                }
            }
            else if c1 == '=' {
                let c2 = chrs[i + 1];
                if c2 == '=' {
                    // Bad:  $X ==1
                    // Good: $X == 1
                    if i >= length - 3 { return (Infix::None, 0); }
                    let c3 = chrs[i + 2];
                    if c3 == ' ' {
                        return (Infix::Equal, i);
                    }
                }
                else if c2 == ' ' {
                    return (Infix::Unify, i);
                }
            }
            else if c1 == '+' {
                if chrs[i + 1] == ' ' { return (Infix::Add, i); }
            }
            else if c1 == '-' {
                if chrs[i + 1] == ' ' { return (Infix::Subtract, i); }
            }
            else if c1 == '*' {
                if chrs[i + 1] == ' ' { return (Infix::Multiply, i); }
            }
            else if c1 == '/' {
                if chrs[i + 1] == ' ' { return (Infix::Divide, i); }
            }

        } // else

        prev = c1;
        i += 1;

    } // while

    return (Infix::None, 0);  // failed to find infix

} // identify_infix

/// Gets terms on left and right-hand side of an infix.
///
/// This function divides a string (vector of characters) which contains
/// an infix, such as "$X = verb" or "$X <= 47". It parses the left and
/// right-hand side, to produce two Unifiable terms.
///
/// If there is an error in parsing a term, the function throws a panic.
///
/// # Arguments
/// * `chrs`  - vector of chars
/// * `index` - index of infix in chrs
/// * `size`  - size of infix
/// # Return
/// * `(Unifiable, Unifiable)`
fn get_left_and_right(chrs: Vec<char>, index: usize, size: usize) -> (Unifiable, Unifiable) {
    let arg1 = &chrs[0..index];
    let arg2 = &chrs[index + size..];
    let term1: Unifiable;
    let term2: Unifiable;
    match parse_term(&chars_to_string!(arg1)) {
        Ok(t) => { term1 = t; },
        Err(err) => { panic!("{}", err); },
    }
    match parse_term(&chars_to_string!(arg2)) {
        Ok(t) => { term2 = t; },
        Err(err) => { panic!("{}", err); },
    }
    return (term1, term2);
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
fn split_complex_term(complex: Vec<char>, index1: usize, index2: usize) -> (String, String) {

      let functor = &complex[0..index1];
      let terms   = &complex[index1 + 1..index2];
      return (chars_to_string!(functor), chars_to_string!(terms));

} // split_complex_term


/// Parses strings to produce subgoals.
///
/// Complex terms, eg. `element(Xenon, $N, $W)`, built-in predicates,
/// eg. `append(…)` and operators such as `!` and `fail` are parsed here.
///
/// # Arguments
/// * `to_parse` - &str
/// # Result
/// * `Result` - Ok([Goal](../goal/enum.Goal.html)) or Err(message)
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
//    let length = chrs.len();

    // not() looks like a built-in predicate
    // but it's actually an operator.
/*
    if s.starts_with("not(") {
        let c = chrs[4..length - 1];
        match parse_subgoal(chars_to_string!(c)) {
            Ok(g) => { return Ok(Not(g)); },
            Err(err) => { return Err(err); },
        }
    }
*/

    // Built-in predicates with no arguments.
    if s == "!" {  // cut
        return Ok(Goal::BuiltInGoal(BuiltInPredicate::Cut));
    }
    if s == "fail" {
        return Ok(Goal::BuiltInGoal(BuiltInPredicate::Fail));
    }
    if s == "nl" { // new line
        return Ok(Goal::BuiltInGoal(BuiltInPredicate::NL));
    }

    //--------------------------------------
    // Handle infixes: = > < >= <= == =

    let (infix, index) = identify_infix(&chrs);
    if infix != Infix::None {

        let pred: BuiltInPredicate;

        if infix == Infix::Unify {
            let (left, right) = get_left_and_right(chrs, index, 1);
            pred = BuiltInPredicate::Unify(vec![left, right]);
        }
        else
        if infix == Infix::Equal {
            let (left, right) = get_left_and_right(chrs, index, 2);
            pred = BuiltInPredicate::Equal(vec![left, right]);
        }
        else
        if infix == Infix::LessThan {
            let (left, right) = get_left_and_right(chrs, index, 1);
            pred = BuiltInPredicate::LessThan(vec![left, right]);
        }
        else
        if infix == Infix::LessThanOrEqual {
            let (left, right) = get_left_and_right(chrs, index, 2);
            pred = BuiltInPredicate::LessThanOrEqual(vec![left, right]);
        }
        else
        if infix == Infix::GreaterThan {
            let (left, right) = get_left_and_right(chrs, index, 1);
            pred = BuiltInPredicate::GreaterThan(vec![left, right]);
        }
        else
        if infix == Infix::GreaterThanOrEqual {
            let (left, right) = get_left_and_right(chrs, index, 2);
            pred = BuiltInPredicate::GreaterThanOrEqual(vec![left, right]);
        }
        else
        if infix == Infix::Add {
            let (left, right) = get_left_and_right(chrs, index, 1);
            pred = BuiltInPredicate::Add(vec![left, right]);
        }
        else
        if infix == Infix::Subtract {
            let (left, right) = get_left_and_right(chrs, index, 1);
            pred = BuiltInPredicate::Subtract(vec![left, right]);
        }
        else
        if infix == Infix::Multiply {
            let (left, right) = get_left_and_right(chrs, index, 1);
            pred = BuiltInPredicate::Multiply(vec![left, right]);
        }
        else
        if infix == Infix::Divide{
            let (left, right) = get_left_and_right(chrs, index, 1);
            pred = BuiltInPredicate::Divide(vec![left, right]);
        }
        else { panic!("parse_subgoal() - Unknown infix: {:?}", infix); }

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
            }
        },
        Err(err) => { return Err(err); },
    }

    let (functor_str, args_str) = split_complex_term(chrs, left_index, right_index);

    // Check for time operator.
    if functor_str == "time" {
        match parse_subgoal(&args_str) {
            Ok(g) => {
                // Wrap g in time-goal.
                let time = Operator::Time(vec![g]);
                let goal = Goal::OperatorGoal(time);
                return Ok(goal);
            },
            Err(err) => { return Err(err); },
        }
    }

    let mut args: Vec<Unifiable>;
    match parse_arguments(&args_str) {
        Ok(a) => { args = a; },
        Err(err) => { return Err(err); },
    }

    match make_built_in_pred(&functor_str, &args) {
        Some(goal) => { return Ok(goal); },
        None => {},
    }

    // Create a complex term.
    let f = Unifiable::Atom(functor_str);
    let mut unifiables = vec![f];
    unifiables.append(&mut args);
    let c = Unifiable::SComplex(unifiables);
    let g = Goal::ComplexGoal(c);
    return Ok(g);

} // parse_subgoal


/// Makes a built-in predicate goal, for print(), append(), etc.
///
/// The function first checks to see if the given functor represents
/// a built-in predicate. If it does, it creates a BuiltInPredicate
/// enum, wraps this enum in a Goal, and returns the Goal. Otherwise
/// it returns None.
///
/// # Arguments
/// * `functor` - &str
/// * `args` - vector of [Unifiable](../unifiable/enum.Unifiable.html) terms
/// # Result
/// * `Option` - Some([Goal](../goal/enum.Goal.html)) or None
///
pub fn make_built_in_pred(functor: &str, args: &Vec<Unifiable>) -> Option<Goal> {
    let functor = functor.to_string();
    if functor == "print" {
        let bip = BuiltInPredicate::Print(args.clone());
        return Some(Goal::BuiltInGoal(bip));
    }
    if functor == "append" {
        let bip = BuiltInPredicate::Append(args.clone());
        return Some(Goal::BuiltInGoal(bip));
    }
    if functor == "include" {
        let bip = BuiltInPredicate::Include(args.clone());
        return Some(Goal::BuiltInGoal(bip));
    }
    if functor == "exclude" {
        let bip = BuiltInPredicate::Exclude(args.clone());
        return Some(Goal::BuiltInGoal(bip));
    }
    if functor == "print_list" {
        let bip = BuiltInPredicate::PrintList(args.clone());
        return Some(Goal::BuiltInGoal(bip));
    }
    None
} // make_built_in_pred()


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

    use super::Infix;
    use super::identify_infix;
    use super::indices_of_parentheses;

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
            Err(msg) => { panic!("{}", msg); }
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
    fn test_identify_infix() {

        let chrs = str_to_chars!("$X = $Y");
        let (inf, ind) = identify_infix(&chrs);
        assert_eq!(inf, Infix::Unify);
        assert_eq!(ind, 3, "Unify");

        let chrs = str_to_chars!("$X =$Y");
        let (inf, ind) = identify_infix(&chrs);
        assert_eq!(inf, Infix::None);
        assert_eq!(ind, 0);

        let chrs = str_to_chars!("$X > $Y");
        let (inf, ind) = identify_infix(&chrs);
        assert_eq!(inf, Infix::GreaterThan);
        assert_eq!(ind, 3, "GreaterThan");

        let chrs = str_to_chars!("$Age >= 50");
        let (inf, ind) = identify_infix(&chrs);
        assert_eq!(inf, Infix::GreaterThanOrEqual);
        assert_eq!(ind, 5, "GreaterThanOrEqual");

        let chrs = str_to_chars!("$Height < 152");
        let (inf, ind) = identify_infix(&chrs);
        assert_eq!(inf, Infix::LessThan);
        assert_eq!(ind, 8, "LessThan");

        let chrs = str_to_chars!("$Grade <= 60");
        let (inf, ind) = identify_infix(&chrs);
        assert_eq!(inf, Infix::LessThanOrEqual);
        assert_eq!(ind, 7, "LessThanOrEqual");

        let chrs = str_to_chars!("100 == $Score");
        let (inf, ind) = identify_infix(&chrs);
        assert_eq!(inf, Infix::Equal);
        assert_eq!(ind, 4, "Equal");

        let chrs = str_to_chars!("\" <= \"");
        let (inf, ind) = identify_infix(&chrs);
        assert_eq!(inf, Infix::None);
        assert_eq!(ind, 0, "Infix between double quotes should be ignored.");

        //-------------------------------------------------------
        // There must be a space after an infix. Otherwise ignore.

        let chrs = str_to_chars!("$X =1");
        let (inf, ind) = identify_infix(&chrs);
        assert_eq!(inf, Infix::None);
        assert_eq!(ind, 0);

        let chrs = str_to_chars!("$X <=1");
        let (inf, ind) = identify_infix(&chrs);
        assert_eq!(inf, Infix::None);
        assert_eq!(ind, 0);

        let chrs = str_to_chars!("$X >=1");
        let (inf, ind) = identify_infix(&chrs);
        assert_eq!(inf, Infix::None);
        assert_eq!(ind, 0);

        let chrs = str_to_chars!("$X ==1");
        let (inf, ind) = identify_infix(&chrs);
        assert_eq!(inf, Infix::None);
        assert_eq!(ind, 0);

        let chrs = str_to_chars!("$X <1");
        let (inf, ind) = identify_infix(&chrs);
        assert_eq!(inf, Infix::None);
        assert_eq!(ind, 0);

        let chrs = str_to_chars!("$X >1");
        let (inf, ind) = identify_infix(&chrs);
        assert_eq!(inf, Infix::None);
        assert_eq!(ind, 0);

    }
} // test
