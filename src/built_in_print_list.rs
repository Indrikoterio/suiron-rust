//! Support functions for the built-in predicate print_list().

use std::rc::Rc;

use super::substitution_set::*;
use super::built_in_predicates::*;
use super::unifiable::Unifiable;

/// Prints out terms of an SLinkedList for the predicate print_list().
///
/// This built-in predicate prints out a list of terms in a readable,
/// comma separated form. <br>It is mainly used for debugging purposes.
///
/// When a logic variable appears in the list, the function prints its
/// ground term.
///
/// # Usage
/// ```
/// use std::rc::Rc;  // needed for empty_ss!()
/// use suiron::*;
///
/// // Make a Suiron list.
/// let list = slist!(false, atom!("一"), atom!("二"), atom!("三"));
///
/// // Make a print_list() predicate.
/// let v = vec![list, atom!("Not a list.")];
/// let print_list_pred = BuiltInPredicate::PrintList(v);
///
/// // Call next_solution_print_list() with predicate and substitution set.
/// let ss = empty_ss!();
/// next_solution_print_list(print_list_pred, &ss);
/// // Prints out:
/// // 一, 二, 三
/// // Not a list.
/// ```
pub fn next_solution_print_list<'a>(bip: BuiltInPredicate,
                                    ss: &'a Rc<SubstitutionSet<'a>>) {

    match bip { // match

        BuiltInPredicate::PrintList(args) => {

            if args.len() == 0 { return; };

            // Iterate through arguments
            let mut first = true;
            for arg in args {

                // If the argument is a variable, get the ground term.
                let arg = match arg {
                    Unifiable::LogicVar{id: _, name: _} => {
                        let ground = get_ground_term(&arg, &ss);
                        match ground {
                            None => { arg },
                            Some(arg) => { arg.clone() },
                        }
                    },
                    _ => arg,
                };

                // If argument is an SLinkedList
                if let Unifiable::SLinkedList{term: _, next: _,
                                  tail_var: _, count: _} = arg {
                    if !first { print!(",\n"); }
                    if let Some(term) = get_ground_term(&arg, &ss) {
                        let s = format_slist(term, &ss);
                        println!("{}", s);
                    }
                    else { println!("{}", arg); }
                }
                else { println!("{}", arg); } // Not a list.
                first = false;
            } // for...
        },
        _ => { panic!("next_solution_print_list() - Invalid built-in predicate."); },

    } // match

} // next_solution_print_list()


/// Formats terms of a Suiron list (SLinkedList) for display.
///
/// The formatted output is a comma separated list of terms. If a term is
/// a logic variable, the function will fetch its ground term for display.
///
/// Called from
/// [next_solution_print_list()](../built_in_print_list/fn.next_solution_print_list.html).
///
/// # Usage
/// ```
/// use std::rc::Rc;  // needed for empty_ss!()
/// use suiron::*;
///
/// let list = parse_term("[Α, Β, Γ, Δ]").unwrap();
/// let ss = empty_ss!();
/// let s = format_slist(&list, &ss);
/// println!("{}", s);
/// // Prints: Α, Β, Γ, Δ
/// ```
pub fn format_slist<'a>(the_list: &'a Unifiable,
                        ss: &'a Rc<SubstitutionSet<'a>>) -> String {

    let mut out = "".to_string();  // The output string.

    let mut s_list = the_list;
    let mut term = &Unifiable::Nil;

    if let Unifiable::SLinkedList{term: t, next: _,
                                  tail_var: _, count: _} = s_list {
        term = t;
        match term {
            Unifiable::Nil => {},
            _ => {
                if let Some(t) = get_ground_term(&t, &ss) {
                    out += &format!("{}", t);
                }
            },
        } // match
    } // if let

    loop {

        if *term == Unifiable::Nil { break; }
        if let Unifiable::SLinkedList{term: _, next,
                           tail_var: _, count: _} = &*s_list {
            s_list = next;
            if let Unifiable::SLinkedList{term: t1, next: _,
                               tail_var, count: _} = s_list {
                term = t1;
                if *tail_var && **t1 != Unifiable::Anonymous {
                    let maybe_list = get_list(t1, &ss);
                    if let Some(list) = maybe_list {
                        s_list = list;
                        if let Unifiable::SLinkedList{ term: t, next: _,
                                          tail_var: _, count: _} = s_list {
                            term = t;
                        }
                    }
                }
            }

            if *term == Unifiable::Nil { break; }
            if let Some(ground) = get_ground_term(&term, &ss) {
                out += &format!(", {}", ground);
            }

        } // if let ....

    } // loop

    return out;

} // format_slist()

#[cfg(test)]
mod test {

    use crate::*;
    use super::*;
    use serial_test::serial;

    // Test format_slist() function.
    #[test]
    fn test_format_slist() {
        let list = parse_term("[Α, Β, Γ, Δ]").unwrap();
        let ss = empty_ss!();  // substitution set
        let s = format_slist(&list, &ss);
        assert_eq!(s, "Α, Β, Γ, Δ");
    }

    // Test next_solution_print_list() function.
    #[test]
    #[serial]
    fn test_next_solution_print_list() {

        clear_id();  // Clear logic variable ID.
        let mut kb = KnowledgeBase::new();
        let rule = parse_rule("test :- $X = Δ, $L = [Α, Β, Γ, $X], \
                               print_list($L), nl.").unwrap();
        add_rules!(&mut kb, rule);

        let goal = parse_subgoal("test").unwrap();
        let base_node = goal.base_node(&kb);

        let solution = next_solution(base_node);

        let s = "----- Substitution Set -----\n\
                0\tNone\n\
                1\tΔ\n\
                2\t[Α, Β, Γ, $X_1]\n\
                ----------------------------";

        match solution {
            Some(ss) => { assert_eq!(s, format_ss(&ss)) },
            None => { panic!("print_list(): No solution."); },
        }

    } // test_next_solution_print_list()

} // test