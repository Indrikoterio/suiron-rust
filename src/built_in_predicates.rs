//! Functions to support built-in predicates.
//!
//! Built-in predicates, such as append() and print(), are a kind of
//! [goal](../goal/enum.Goal.html).
//!
// Cleve Lendon  2023

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use super::logic_var::*;
use super::unifiable::*;
use super::solution_node::*;
use super::built_in_print::*;
use super::built_in_count::*;
use super::built_in_append::*;
use super::built_in_filter::*;
use super::built_in_comparison::*;
use super::built_in_print_list::*;
use super::substitution_set::*;

/// Defines built-in predicates, such as print(), append(), etc.
///
/// In Suiron source code, built-in predicates have the form:
/// <blockquote>
/// functor(term1, term2, ...)
/// </blockquote>
#[derive(Debug, Clone, PartialEq)]
pub struct BuiltInPredicate {
    pub functor: String,
    pub terms: Option<Vec<Unifiable>>,
}

impl BuiltInPredicate {

    /// Creates a new BuiltInPredicate struct.
    ///
    /// # Usage
    /// ```
    /// use suiron::*;
    ///
    /// // To make: append(a, b, c)
    /// let app = "append".to_string();
    /// let terms = vec![atom!("a"), atom!("b"), atom!("c")];
    /// let pred = BuiltInPredicate::new(app, Some(terms));
    ///
    /// // To make a 'fail' predicate:
    /// let pred = BuiltInPredicate::new("fail".to_string(), None);
    /// ```
    #[inline]
    pub fn new(functor: String, terms: Option<Vec<Unifiable>>) -> Self {
        BuiltInPredicate { functor, terms }
    }

    /// Recreates logic variables to give them unique IDs.
    ///
    /// Logic variables in the knowledge base have an ID of 0, but
    /// when a rule is fetched from the knowledge base, the logic
    /// variables must be given unique IDs.
    ///
    /// # Arguments
    /// * self
    /// * map of previously recreated variable IDs
    /// # Return
    /// * new BuiltInPredicate
    ///
    /// # Usage
    /// ```
    /// use suiron::*;
    ///
    /// // Make a built-in predicate: print($X, $Y)
    /// let print_predicate = parse_subgoal("print($X, $Y)").unwrap();
    ///
    /// let mut var_map = VarMap::new();
    /// let new_print = print_predicate.recreate_variables(&mut var_map);
    /// println!("{}", new_print); // Prints: print($X_1, $Y_2)
    /// ```
    pub fn recreate_variables(self, vars: &mut VarMap) -> BuiltInPredicate {

        if let Some(terms) = self.terms {
            let new_terms = recreate_vars_terms(terms, vars);
            return BuiltInPredicate::new(self.functor, Some(new_terms));
        }
        return BuiltInPredicate::new(self.functor, None);

    } // recreate_variables()

} // BuiltInPredicate

/// Finds solutions for built-in predicates.
///
/// See also [next_solution()](../solution_node/fn.next_solution.html)
/// in solution_node.rs.
///
/// # Arguments
/// * reference to [SolutionNode](../solution_node/struct.SolutionNode.html)
/// * [BuiltInPredicate](../built_in_predicates/struct.BuiltInPredicate.html)
/// # Return
/// * [SubstitutionSet](../substitution_set/type.SubstitutionSet.html) or None
pub fn next_solution_bip<'a>(sn: Rc<RefCell<SolutionNode<'a>>>,
                             bip: BuiltInPredicate)
                             -> Option<Rc<SubstitutionSet<'a>>> {

    let mut sn_ref = sn.borrow_mut(); // Get a mutable reference.

    if !sn_ref.more_solutions { return None; };
    sn_ref.more_solutions = false;

    match bip.functor.as_str() {
        "print" => {
            next_solution_print(bip, &sn_ref.ss);
            let ss = Rc::clone(&sn_ref.ss);
            return Some(ss);
        },
        "append" => {
            // next_solution_append writes to ss.
            return next_solution_append(bip, &sn_ref.ss);
        },
        "functor" => {
            panic!("Implement this: functor()");
        },
        "include" => { // filters a list
            return bip_include(bip, &sn_ref.ss);
        },
        "exclude" => {
            return bip_exclude(bip, &sn_ref.ss);
        },
        "print_list" => {
            next_solution_print_list(bip, &sn_ref.ss);
            let ss = Rc::clone(&sn_ref.ss);
            return Some(ss);
        },
        "unify" => {
            if let Some(terms) = &bip.terms {
                let left  = &terms[0];
                let right = &terms[1];
                return left.unify(right, &sn_ref.ss);
            }
            return None;
        },
        "equal" => {
            return bip_equal(bip, &sn_ref.ss);
        },
        "less_than" => {
            return bip_less_than(bip, &sn_ref.ss);
        },
        "less_than_or_equal" => {
            return bip_less_than_or_equal(bip, &sn_ref.ss);
        },
        "greater_than" => {
            return bip_greater_than(bip, &sn_ref.ss);
        },
        "greater_than_or_equal" => {
            return bip_greater_than_or_equal(bip, &sn_ref.ss);
        },
        "nl" => { // New Line. This cannot fail.
            print!("\n");
            return Some(Rc::clone(&sn_ref.ss));
        },
        "!" => { // !
            sn_ref.set_no_backtracking();
            return Some(Rc::clone(&sn_ref.ss));
        },
        "count" => { // count terms in list
            return bip_count(bip, &sn_ref.ss);
        },
        "fail" => { return None; }, // always fails
        _ => { panic!("next_solution_bip() - Not implemented yet: {}",
                       bip.functor.as_str()); },
    }
} // next_solution_bip()

/// Formats a built-in predicate (or function) for Display.
///
/// Built-in predicates and functions have the format:
/// <blockquote>
/// name(term1, term2, term3...)
/// </blockquote>
///
/// # Arguments
/// * name (string)
/// * vector of [Unifiable](../unifiable/enum.Unifiable.html) terms
/// # Return
/// * formatted string
/// # Usage
/// ```
/// use suiron::*;
///
/// let terms = vec![logic_var!("$X"), logic_var!("$Y")];
/// let s = format_built_in("compare", &terms);
/// println!("{}", s);  // Prints: compare($X, $Y)
/// ```
pub fn format_built_in(name: &str, terms: &Vec<Unifiable>) -> String {
    let mut out = format!("{}(", name);
    let mut comma = false;
    for term in terms {
        if comma { out += ", "; }
        else { comma = true; }
        out += &term.to_string();
    }
    out += ")";
    out
} // format_built_in

// Display trait, to display built-in predicates.
impl fmt::Display for BuiltInPredicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let func = self.functor.as_str();
        match &self.terms {
            Some(terms) => {
                match func {
                    "unify" => {
                        let (left, right) = (&terms[0], &terms[1]);
                        return write!(f, "{} = {}", left, right);
                    },
                    _ => {
                        let out = format_built_in(func, terms);
                        return write!(f, "{}", out);
                    },
                }
            },
            None => { return write!(f, "{}", func); }
        }
    }
} // Display


#[cfg(test)]
mod test {

    use super::*;
    use crate::*;

    // Create logic vars for testing.
    fn x() -> Unifiable { logic_var!("$X") }

    fn two_terms() -> Vec<Unifiable> {
        let my_name = atom!("Klivo");
        vec![x(), my_name]
    }

    // Test formatting of (some) built-in predicates.
    #[test]
    fn test_format_built_in() {
        let s = format_built_in("pred_name", &two_terms());
        assert_eq!("pred_name($X, Klivo)", s);
    }

    // Test Display trait for built-in predicates.
    #[test]
    fn test_display() {

        let functor = "print".to_string();
        let print_pred = BuiltInPredicate::new(functor, Some(two_terms()));
        assert_eq!("print($X, Klivo)", format!("{}", print_pred));

        let functor = "unify".to_string();
        let unify_pred = BuiltInPredicate::new(functor, Some(two_terms()));
        assert_eq!("$X = Klivo", format!("{}", unify_pred));
    }

} // test