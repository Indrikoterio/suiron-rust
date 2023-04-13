//! Appends terms (including lists) to make a new linked list.

use std::rc::Rc;

use super::unifiable::*;
use super::s_linked_list::*;
use super::substitution_set::*;
use super::built_in_predicates::*;

/// Evaluates a solution node for the append() predicate.
///
/// Appends terms together to create a list.
///
/// The append() predicate requires at least two arguments.
/// The first n - 1 arguments are input arguments, and the
/// last argument is the output argument. (All arguments are
/// unifiable terms.)
///
/// This function is called by
/// [next_solution_bip()](../built_in_predicates/fn.next_solution_bip.html)
/// in built_in_predicates.rs.
///
/// # Arguments
/// * [BuiltInPredicate](../built_in_predicates/enum.BuiltInPredicate.html)
/// * [SubstitutionSet](../substitution_set/type.SubstitutionSet.html)
/// # Return
/// * [SubstitutionSet](../substitution_set/type.SubstitutionSet.html) or None
///
pub fn next_solution_append<'a>(bip: BuiltInPredicate,
                                ss: &'a Rc<SubstitutionSet<'a>>)
                                -> Option<Rc<SubstitutionSet<'a>>> {

    if let BuiltInPredicate::Append(terms) = bip {

        let length = terms.len();
        if length < 2 { return None; }

        let mut out_terms: Vec<Unifiable> = vec![];

        for i in 0..(length - 1) {

            let mut t = terms[i].clone();

            // If logic variable, get ground term.
            if let Unifiable::LogicVar{id: _, name: _} = t {
                match get_ground_term(&t, &ss) {
                    Some(new_term) => { t = new_term.clone(); },
                    None => {},
                }
            }

            match t {
                Unifiable::Nil |
                Unifiable::Anonymous |
                Unifiable::Atom(_) |
                Unifiable::SInteger(_) |
                Unifiable::SFloat(_) |
                Unifiable::SFunction{name: _, terms: _} |
                Unifiable::SComplex(_) => { out_terms.push(t); },
                Unifiable::SLinkedList{term: _, next: _, count: _, tail_var: _} => {
                    let mut list = t;
                    loop {
                        if let Unifiable::SLinkedList{term, next,
                                          count: _, tail_var: _} = list {
                            if *term == Unifiable::Nil { break; }
                            out_terms.push(*term);
                            list = *next;
                        }
                    }
                },
                // LogicVar was dealt with above.
                Unifiable::LogicVar{id: _, name: _} => {},
            } // match

        } // for

        let out = make_linked_list(false, out_terms);
        let last_term = terms[length - 1].clone();

        // Unify new list with last term.
        return last_term.unify(&out, &ss);
    }
    panic!("next_solution_append() - Invalid built-in predicate.");

} // next_solution_append()

/// Appends terms together to create a list.
///
/// The append() predicate requires at least two arguments.
/// The first n - 1 arguments are input arguments, and the
/// last argument is the output argument. (All arguments are
/// unifiable terms.)
///
/// Examples:
///
/// # Arguments
/// * vector of Unifiable terms
/// * [SubstitutionSet](../substitution_set/type.SubstitutionSet.html)
/// # Result
/// * [SubstitutionSet](../substitution_set/type.SubstitutionSet.html) or None
pub fn append_terms<'a>(terms: Vec<Unifiable>,
                        ss: Rc<SubstitutionSet<'a>>)
                        -> Option<Rc<SubstitutionSet<'a>>> {

    let length = terms.len();
    if length < 2 { return None; }

    let mut out_terms: Vec<Unifiable> = vec![];

    for i in 0..(length - 1) {

        let mut t = terms[i].clone();

        // If logic variable, get ground term.
        if let Unifiable::LogicVar{id: _, name: _} = t {
            match get_ground_term(&t, &ss) {
                Some(new_term) => { t = new_term.clone(); },
                None => {},
            }
        }

        match t {
            Unifiable::Nil |
            Unifiable::Anonymous |
            Unifiable::Atom(_) |
            Unifiable::SInteger(_) |
            Unifiable::SFloat(_) |
            Unifiable::SFunction{name: _, terms: _} |
            Unifiable::SComplex(_) => { out_terms.push(t); },
            Unifiable::SLinkedList{term: _, next: _, count: _, tail_var: _} => {
                let mut list = t;
                loop {
                    if let Unifiable::SLinkedList{term, next,
                                      count: _, tail_var: _} = list {
                        if *term == Unifiable::Nil { break; }
                        out_terms.push(*term);
                        list = *next;
                    }
                }
            },
            // LogicVar was dealt with above.
            Unifiable::LogicVar{id: _, name: _} => {},
        } // match

    } // for

    let out = make_linked_list(false, out_terms);
    let last_term = terms[length - 1].clone();

    // Unify new list with last term.
    return last_term.unify(&out, &ss);

} // append_terms()


#[cfg(test)]
mod test {

    use std::rc::Rc;
    use crate::*;
    use super::*;

    // Test append() predicate.
    #[test]
    fn test_append() {

        let kb = KnowledgeBase::new();

        // Make a base solution node.
        let query = parse_query("go").unwrap();
        let base_node = make_base_node(Rc::new(query), &kb);

        // Make a list of arguments: float, list, integer, logic var.
        let args = parse_arguments("3.14159, [A, B, C], 6, $Out").unwrap();

        // Make an append() predicate.
        let append_pred = make_goal("append", args);

        // Recreate variables.
        let mut var_map = VarMap::new();
        let append_pred = append_pred.recreate_variables(&mut var_map);

        // Create a solution node.
        let sn = make_solution_node(Rc::new(append_pred), &kb, empty_ss!(), base_node);

        // Get the solution. This will run next_solution_append().
        match next_solution(sn) {
            Some(ss) => {
                let term = ss[1].clone().unwrap();
                let s = format!("{}", *term);
                assert_eq!("[3.14159, A, B, C, 6]", s)
            },
            None => { panic!("Append() should join terms together."); },
        } // match

    } // test_append()

} // test