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
use super::built_in_append::*;
use super::built_in_comparison::*;
use super::built_in_print_list::*;
use super::substitution_set::*;

/// Defines built-in predicates, such as print(), append(), etc.
///
/// In Suiron source code, built-in predicates look like complex terms:
/// <blockquote>
/// predicate_name(term1, term2, ...)
/// <blockquote>
#[derive(Debug, Clone, PartialEq)]
pub enum BuiltInPredicate {
    Print(Vec<Unifiable>),
    Append(Vec<Unifiable>),
    Functor(Vec<Unifiable>),
    Include(Vec<Unifiable>),
    Exclude(Vec<Unifiable>),
    PrintList(Vec<Unifiable>),
    Unify(Vec<Unifiable>),
    Equal(Vec<Unifiable>),
    LessThan(Vec<Unifiable>),
    LessThanOrEqual(Vec<Unifiable>),
    GreaterThan(Vec<Unifiable>),
    GreaterThanOrEqual(Vec<Unifiable>),
    NL,  // New Line
    Fail,
    Cut,
}

impl BuiltInPredicate {

    /// Recreate logic variables to give them unique IDs.
    ///
    /// Logic variables in the knowledge base have an ID of 0, but
    /// when a rule is fetched from the knowledge base, the logic
    /// variables must be given unique IDs.
    ///
    /// # Arguments
    /// * `self`
    /// * `vars` - set of previously recreated variable IDs
    /// # Return
    /// * `recreated BuiltInPredicate`
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

        match self {
            BuiltInPredicate::Print(terms) => {
                let new_terms = recreate_vars_terms(terms, vars);
                return BuiltInPredicate::Print(new_terms);
            },
            BuiltInPredicate::Append(terms) => {
                let new_terms = recreate_vars_terms(terms, vars);
                return BuiltInPredicate::Append(new_terms);
            },
            BuiltInPredicate::Functor(terms) => {
                let new_terms = recreate_vars_terms(terms, vars);
                return BuiltInPredicate::Functor(new_terms);
            },
            BuiltInPredicate::Include(terms) => {
                let new_terms = recreate_vars_terms(terms, vars);
                return BuiltInPredicate::Include(new_terms);
            },
            BuiltInPredicate::Exclude(terms) => {
                let new_terms = recreate_vars_terms(terms, vars);
                return BuiltInPredicate::Exclude(new_terms);
            },
            BuiltInPredicate::PrintList(terms) => {
                let new_terms = recreate_vars_terms(terms, vars);
                return BuiltInPredicate::PrintList(new_terms);
            },
            BuiltInPredicate::Unify(terms) => {
                let new_terms = recreate_vars_terms(terms, vars);
                return BuiltInPredicate::Unify(new_terms);
            },
            BuiltInPredicate::Equal(terms) => {
                let new_terms = recreate_vars_terms(terms, vars);
                return BuiltInPredicate::Equal(new_terms);
            },
            BuiltInPredicate::GreaterThan(terms) => {
                let new_terms = recreate_vars_terms(terms, vars);
                return BuiltInPredicate::GreaterThan(new_terms);
            },
            BuiltInPredicate::GreaterThanOrEqual(terms) => {
                let new_terms = recreate_vars_terms(terms, vars);
                return BuiltInPredicate::GreaterThanOrEqual(new_terms);
            },
            BuiltInPredicate::LessThan(terms) => {
                let new_terms = recreate_vars_terms(terms, vars);
                return BuiltInPredicate::LessThan(new_terms);
            },
            BuiltInPredicate::LessThanOrEqual(terms) => {
                let new_terms = recreate_vars_terms(terms, vars);
                return BuiltInPredicate::LessThanOrEqual(new_terms);
            },
            BuiltInPredicate::NL   => { BuiltInPredicate::NL },
            BuiltInPredicate::Fail => { BuiltInPredicate::Fail },
            BuiltInPredicate::Cut  => { BuiltInPredicate::Cut },
        }
    } // recreate_variables()

} // BuiltInPredicate


/// Finds solutions for built-in predicates.
///
/// See also [next_solution()](../solution_node/fn.next_solution.html)
/// in solution_node.rs.
///
/// # Arguments
/// * `bip` - [BuiltInPredicate](../built_in_predicates/enum.BuiltInPredicate.html)
/// * `sn_ref` - reference to
/// [SolutionNode](../solution_node/struct.SolutionNode.html)
/// # Return
/// * `Option` -
/// Some([SubstitutionSet](../substitution_set/type.SubstitutionSet.html))
/// or None
pub fn next_solution_bip<'a>(sn: Rc<RefCell<SolutionNode<'a>>>,
                             bip: BuiltInPredicate)
                             -> Option<Rc<SubstitutionSet<'a>>> {

    let mut sn_ref = sn.borrow_mut(); // Get a mutable reference.

    if !sn_ref.more_solutions { return None; };
    sn_ref.more_solutions = false;

    match bip {
        BuiltInPredicate::Print(_) => {
            next_solution_print(bip, &sn_ref.ss);
            let ss = Rc::clone(&sn_ref.ss);
            return Some(ss);
        },
        BuiltInPredicate::Append(_) => {
            // next_solution_append writes to ss.
            return next_solution_append(bip, &sn_ref.ss);
        },
        BuiltInPredicate::Unify(args) => {
            let left  = &args[0];
            let right = &args[1];
            return left.unify(right, &sn_ref.ss);
        },
        BuiltInPredicate::Equal(args) => {
            return bip_equal(args, &sn_ref.ss);
        },
        BuiltInPredicate::LessThan(args) => {
            return bip_less_than(args, &sn_ref.ss);
        },
        BuiltInPredicate::LessThanOrEqual(args) => {
            return bip_less_than_or_equal(args, &sn_ref.ss);
        },
        BuiltInPredicate::GreaterThan(args) => {
            return bip_greater_than(args, &sn_ref.ss);
        },
        BuiltInPredicate::GreaterThanOrEqual(args) => {
            return bip_greater_than_or_equal(args, &sn_ref.ss);
        },
        BuiltInPredicate::PrintList(_) => {
            next_solution_print_list(bip, &sn_ref.ss);
            let ss = Rc::clone(&sn_ref.ss);
            return Some(ss);
        },
        BuiltInPredicate::NL => { // New Line. This cannot fail.
            print!("\n");
            return Some(Rc::clone(&sn_ref.ss));
        },
        BuiltInPredicate::Cut => { // !
            sn_ref.set_no_backtracking();
            return Some(Rc::clone(&sn_ref.ss));
        },
        BuiltInPredicate::Fail => { return None; }, // always fails
        _ => { panic!("next_solution_bip() - Not implemented yet."); },
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
/// * `name` - &str
/// * `terms` - vector of
/// [Unifiable](../unifiable/enum.Unifiable.html) terms
/// # Return
/// * `formatted string`
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
        let out = match &self {
            BuiltInPredicate::Print(args) => {
                format_built_in("print", args)
            },
            BuiltInPredicate::Append(args) => {
                format_built_in("append", args)
            },
            BuiltInPredicate::Functor(args) => {
                format_built_in("functor", args)
            },
            BuiltInPredicate::Include(args) => {
                format_built_in("include", args)
            },
            BuiltInPredicate::Exclude(args) => {
                format_built_in("exclude", args)
            },
            BuiltInPredicate::PrintList(args) => {
                format_built_in("print_list", args)
            },
            BuiltInPredicate::Unify(args) => {
                format!("{} = {}", &args[0], &args[1])
            },
            BuiltInPredicate::Equal(args) => {
                format!("{} == {}", &args[0], &args[1])
            },
            BuiltInPredicate::LessThan(args) => {
                format!("{} < {}", &args[0], &args[1])
            },
            BuiltInPredicate::LessThanOrEqual(args) => {
                format!("{} <= {}", &args[0], &args[1])
            },
            BuiltInPredicate::GreaterThan(args) => {
                format!("{} > {}", &args[0], &args[1])
            },
            BuiltInPredicate::GreaterThanOrEqual(args) => {
                format!("{} >= {}", &args[0], &args[1])
            },
            BuiltInPredicate::NL => { "nl".to_string() },
            BuiltInPredicate::Cut => { "!".to_string() },
            BuiltInPredicate::Fail => { "fail".to_string() },

        }; // match

        write!(f, "{}", out)
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

        let print_pred = BuiltInPredicate::Print(two_terms());
        assert_eq!("print($X, Klivo)", format!("{}", print_pred));

        let unify_pred = BuiltInPredicate::Unify(two_terms());
        assert_eq!("$X = Klivo", format!("{}", unify_pred));
    }

} // test