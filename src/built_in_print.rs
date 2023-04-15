//! Support for built-in predicate print().

use std::rc::Rc;

use super::substitution_set::*;
use super::built_in_predicates::*;

static FORMAT_SPECIFIER: &str = "%s";

/// Prints out terms for the built-in print() predicate.
///
/// This function is called by
/// [next_solution_bip()](../built_in_predicates/fn.next_solution_bip.html)
/// in built_in_predicates.rs.
///
/// It converts the predicate's operands (Unifiable terms) into strings,
/// formats them for output, and prints out the result.
///
/// # Arguments
/// * [BuiltInPredicate](../built_in_predicates/enum.BuiltInPredicate.html)
/// * [SubstitutionSet](../substitution_set/type.SubstitutionSet.html)
/// # Usage
/// ```
/// use std::rc::Rc;
/// use suiron::*;
///
/// // Make a print() predicate.
/// let functor = "print".to_string();
/// let v = vec![atom!("A"), SInteger(7), atom!("\n")];
/// let print_pred = BuiltInPredicate::new(functor, Some(v));
///
/// // Call function with print predicate and substitution set.
/// let ss = empty_ss!();
/// next_solution_print(print_pred, &ss);
/// // Prints: A7
/// ```
pub fn next_solution_print<'a>(bip: BuiltInPredicate,
                               ss: &'a Rc<SubstitutionSet<'a>>) {
    let mut v: Vec<String> = vec![];
    if let Some(terms) = bip.terms {
        // Collect ground terms into v.
        for term in terms {
            match get_ground_term(&term, &ss) {
                Some(ground_term) => { v.push(format!("{}", ground_term)); },
                None              => { v.push(format!("{}", term)); },
            }
        }
        print!("{}", format_for_print_pred(&v));
    }
} // next_solution_print()

/// Formats a list of strings for the built-in predicate print().
///
/// If the first string contains format specifiers (%s), the function
/// uses them as place-holders for the following strings.
///
/// For example, if the first string is "I'm sorry %s, I'm afraid I can't
/// do that.", and the second string is "Dave", the function will output
/// "I'm sorry Dave, I'm afraid I can't do that."
///
/// Otherwise, if there are no format specifiers, the function simply
/// concatenates the strings.
///
/// This function is called by
/// [next_solution_print()](../built_in_print/fn.next_solution_print.html).
///
/// # Arguments
/// * vector of Strings
/// # Return
/// * a String
/// # Usage
/// ```
/// use suiron::*;
///
/// let s1 = "Hello, %s. You're looking well today.".to_string();
/// let s2 = "Dave".to_string();
/// let v = vec![s1, s2];
///
/// let s = format_for_print_pred(&v);
/// println!("{}", s);
/// // Prints: Hello, Dave. You're looking well today.
/// ```
pub fn format_for_print_pred(the_strings: &Vec<String>) -> String {
    if the_strings.len() == 0 {
        panic!("format_for_print_pred() - No strings to format.");
    }
    let format_string = the_strings[0].to_string();
    let split: Vec<_> = format_string.split(FORMAT_SPECIFIER).collect();
    let mut out: String = split[0].to_string();
    let mut i = 1;
    let mut j = 1;
    let split_length = split.len();
    let num_of_strings = the_strings.len();
    loop {
        if j < num_of_strings { out += &the_strings[j]; j += 1; }
        if i < split_length { out += split[i]; i += 1; }
        if i >= split_length && j >= num_of_strings { break; }
    }
    out
} // format_for_print_pred()

#[cfg(test)]
mod test {

    use super::*;

    // Test formatting with a format specifier: %s
    #[test]
    fn test_format_for_print_pred() {
        let s1 = "Hello, %s. ".to_string();
        let s2 = "Dave".to_string();
        let s3 = "You're looking well today.".to_string();
        let v = vec![s1, s2, s3];
        let s = format_for_print_pred(&v);
        assert_eq!("Hello, Dave. You're looking well today.", s);
    }

} // test