//! Functions which support join().
//!
//! This function joins constants (atoms, numbers) to form a single
//! constant. It is used to join words and punctuation.
//!
//! Words are separated by a space, but punctuation is attached
//! directly to the previous word. For example:
//!
//! <pre>
//!   $D1 = coffee, $D2 = \, , $D3 = tea, $D4 = or, $D5 = juice,
//!   $X = join($D1, $D2, $D3, $D4, $D5).
//! </pre>
//!
//! $X is bound to the atom: `coffee, tea or juice`
//!
// Cleve Lendon  2023

use std::rc::Rc;
use super::unifiable::Unifiable;
use super::s_linked_list::*;
use super::substitution_set::*;

use crate::atom;

/// Evaluates the join() function.
///
/// This method is called by
/// [unify_sfunction()](../built_in_functions/fn.unify_sfunction.html#).
///
/// # Arguments
/// * list of [Unifiable](../unifiable/enum.Unifiable.html) terms
/// * [SubstitutionSet](../substitution_set/index.html)
/// # Returns
/// * atom (string constant)
/// # Usage
/// ```
/// use std::rc::Rc;
/// use suiron::*;
///
/// let start = atom!("Would you like");
/// let list = parse_term("[coffee, \\,, tea, or, juice]").unwrap();
/// let punc = atom!("?");
/// let ss = empty_ss!();
/// let result = evaluate_join(&vec![start, list, punc], &ss);
/// println!("{}", result); // Prints: Would you like coffee, tea or juice?
///
/// ```
pub fn evaluate_join<'a>(in_terms: &'a Vec<Unifiable>,
                         ss: &'a Rc<SubstitutionSet<'a>>) -> Unifiable {

    let mut all_terms: Vec<Unifiable> = vec![];

    for term in in_terms {
        let mut term_list = get_terms(term, ss);
        all_terms.append(&mut term_list);
    }

    let mut out = "".to_string();
    let mut first = true;
    for term in all_terms {
        let s = format!("{}", term);
        if is_punctuation(&s) {
            out += &s;
            first = false;
        }
        else {
            if first {
                out += &s;
                first = false;
            }
            else { out += &format!(" {}", &s); }
        }
    }

    return atom!(out);

} // evaluate_join

// Checks if the given string slice is punctuation.
fn is_punctuation(s: &str) -> bool {
    if s == "," || s == "." ||
       s == "?" || s == "!" { return true; }
    return false;
}
