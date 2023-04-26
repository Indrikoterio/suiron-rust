//! Gets the functor and arity of a complex term.
//!
// Cleve Lendon 2023

use std::rc::Rc;

use crate::str_to_chars;
use crate::chars_to_string;

use super::substitution_set::*;
use super::built_in_predicates::*;
use super::unifiable::{*, Unifiable::*};

/// Evaluates a solution node for the functor() predicate.
///
/// The functor predicate gets the functor and arity of a complex term.
/// For example:
/// <pre>
///    functor(boss(Zack, Stephen), $Func, $Arity)
/// </pre>
///
/// The first term, an input argument, is a complex term.
///
/// $Func will bind to 'boss' and $Arity will bind to '2' (because
/// there are two arguments, Zack and Stephen). Arity is optional.
/// The following is valid:
/// <pre>
///    functor(boss(Zack, Stephen), $Func)
/// </pre>
///
/// Of course, the first argument would normally be a logic variable.
/// <pre>
///    $X = boss(Zack, Stephen), functor($X, $Func)
/// </pre>
///
/// The next goal will not succeed, because the arity is incorrect:
/// <pre>
///    functor($X, boss, 3)
/// </pre>
///
/// If the second argument has an asterisk at the end, the match will
/// test only the start of the string. For example, the following
/// will succeed:
/// <pre>
///    $X = noun_phrase(the blue sky), functor($X, noun*)
/// </pre>
///
/// TODO:
/// Perhaps the functionality could be expanded to accept a regex string
/// for the second argument.
///
/// This function is called by
/// [next_solution_bip()](../built_in_predicates/fn.next_solution_bip.html)
/// in built_in_predicates.rs.
///
/// # Arguments
/// * [BuiltInPredicate](../built_in_predicates/struct.BuiltInPredicate.html)
/// * [SubstitutionSet](../substitution_set/type.SubstitutionSet.html)
/// # Return
/// * [SubstitutionSet](../substitution_set/type.SubstitutionSet.html) or None
///
pub fn next_solution_functor<'a>(bip: BuiltInPredicate,
                                 ss: &'a Rc<SubstitutionSet<'a>>)
                                 -> Option<Rc<SubstitutionSet<'a>>> {

    if let Some(terms) = bip.terms {

        let length = terms.len();
        if length < 2 || length > 3 { return None; }

        let mut out_terms: Vec<&Unifiable> = vec![];

        for i in 0..length {

            let mut t = &terms[i];

            // If logic variable, get ground term.
            if let Unifiable::LogicVar{id: _, name: _} = t {
                match get_ground_term(t, &ss) {
                    Some(new_term) => { t = new_term; },
                    None => { },
                }
            }
            out_terms.push(t);

        } // for

        // Get the terms of the complex term being analyzed.
        let c_terms = match out_terms[0] {
            SComplex(terms) => { terms },
            _ => { return None; },
        };

        let arity:i64 = (c_terms.len() - 1) as i64;
        let functor = &c_terms[0];

        if length == 2 {  // two arguments

            match &out_terms[1] {
                Atom(match_string) => {
                    if atoms_match(functor, match_string) {
                        let ss = Rc::clone(ss);
                        return Some(ss);
                    }
                },
                LogicVar{id: _, name: _} => {
                    return out_terms[1].unify(functor, &ss);
                },
                _ => { return None; },
            }
            return None;
        }

        if length == 3 {  // three arguments

            let mut ss = Rc::clone(ss);

            match &out_terms[1] {
                Atom(match_string) => {
                    if !atoms_match(functor, match_string) { return None; }
                },
                LogicVar{id: _, name: _} => {
                    ss = out_terms[1].unify(functor, &ss)?;
                },
                _ => { return None; },
            }

            let arity_term = out_terms[2].clone();
            return arity_term.unify(&SInteger(arity), &ss);
        }
    }
    return None;

} // next_solution_functor()

// Compares two atoms to see if they match.
//
// For the functor predicate, if the second term ends with
// an asterisk, only the first characters are compared.
// Eg., the functor predicate below will succeed.
//    $X = noun_phrase(the blue sky), functor($X, noun*)
//
// Arguments
// * functor (atom)
// * match string (string)
// Return
// * true if arguments match
//
fn atoms_match(functor: &Unifiable, match_string: &str) -> bool {

    let f = match functor {
        Unifiable::Atom(f) => { f },
        _ => { return false; },
    };

    let chrs = str_to_chars!(match_string);
    let length = chrs.len();
    let last_ch = chrs[length - 1];

    if last_ch == '*' {
        let chrs2 = &chrs[0..length - 1];
        let s2 = chars_to_string!(chrs2);
        return f.starts_with(&s2);
    }
    else { return f.eq(match_string); }

} // atoms_match

#[cfg(test)]
mod test {

    use std::rc::Rc;
    use crate::*;
    use super::*;

    // Test append() predicate.
    #[test]
    fn test_functor() {

        let kb = KnowledgeBase::new();

        // Make a base solution node.
        let query = parse_query("go").unwrap();
        let base_node = make_base_node(Rc::new(query), &kb);

        // Make an append() predicate.
        let append_pred = match parse_subgoal(
                          "append(3.14159, [A, B, C], 6, $Out)") {
            Ok(goal) => { goal },
            Err(err) => { panic!("{}", err); },
        };

        // Recreate variables.
        let mut var_map = VarMap::new();
        let append_pred = append_pred.recreate_variables(&mut var_map);

        // Create a solution node.
        let sn = make_solution_node(Rc::new(append_pred), &kb,
                                    empty_ss!(), base_node);

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