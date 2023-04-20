//! Functions to support the built-in predicate count().
//!
//! The count predicate counts terms in a Suiron list.
//
// Cleve Lendon  2023

use std::rc::Rc;

use super::unifiable::Unifiable::*;
use super::s_linked_list::*;
use super::substitution_set::*;
use super::built_in_predicates::*;

/// Counts the terms in a Suiron list.
///
/// In Suiron source code, the count() predicate requires two arguments.
/// <pre>
///     count($MyList, $Count)
/// </pre>
///
/// The first argument is the list to be counted. (Or a logic variable
/// which is bound to a list.) The second is an output variable, which
/// unifies with the calculated count.
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
pub fn bip_count<'a>(bip: BuiltInPredicate, ss: &'a Rc<SubstitutionSet<'a>>)
                     -> Option<Rc<SubstitutionSet<'a>>> {

    if let Some(terms) = bip.terms {

        if terms.len() != 2 { panic!("bip_count() - Requires 2 arguments."); }

        let count = count_terms(&terms[0], &Rc::clone(&ss));
        let count = SInteger(count);

        let out = &terms[1];
        let ss = out.unify(&count, &Rc::clone(&ss));

        return ss;
    }
    panic!("bip_count() - Requires 2 arguments.");

} // bip_count()


#[cfg(test)]
mod test {

    use std::rc::Rc;
    use crate::*;
    use super::*;

    // Test count() predicate.
    #[test]
    fn test_count() {

        let mut kb = KnowledgeBase::new();

        fn c() -> Unifiable { logic_var!("$Count") }
        let list = slist!(false, atom!("a"), atom!("b"), atom!("c"));

        let c1 = scomplex!(atom!("test_count"), c());
        let count_pred = pred!("count", list, c());
        let rule1 = make_rule(c1, count_pred);

        add_rules!(&mut kb, rule1);

        let x = logic_var!("$X");
        let query = query!(atom!("test_count"), x);
        let sn = make_base_node(query, &kb);

        let solutions = solve(sn);
        let result = format!("{}", solutions);
        assert_eq!("$X = 3", result);

    } // test_count()

} // test