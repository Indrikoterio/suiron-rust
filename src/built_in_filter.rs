//! Functions to support the built-in predicates include() and exclude().
//!
//! The include() predicate filters a list to create a new list,<br>
//! which includes terms which match the filter term.
//!
//! The exclude() predicate filters a list to create a new list,<br>
//! which excludes terms which match the filter term.
//
// Cleve Lendon  2023

use std::rc::Rc;

use super::s_linked_list::*;
use super::substitution_set::*;
use super::built_in_predicates::*;

/// Filters a Suiron list to include terms which match the filter term.
///
/// In Suiron source code, the include() predicate requires three arguments.
/// <pre>
///    include(female($_), $InList, $OutList)
/// </pre>
/// The first argument is the filter term. Terms in the list which can unify
/// with the filter term are included in the output. The second argument is
/// the input term, which should be bound to a list. The last term is the
/// output term, which unifies with the new filtered list.
///
/// This function is called from
/// [next_solution_bip()](../built_in_predicates/fn.next_solution_bip.html)
/// in built_in_predicates.rs.
///
/// # Arguments
/// * [BuiltInPredicate](../built_in_predicates/struct.BuiltInPredicate.html)
/// * [SubstitutionSet](../substitution_set/type.SubstitutionSet.html)
/// # Return
/// * [SubstitutionSet](../substitution_set/type.SubstitutionSet.html) or None
///
pub fn bip_include<'a>(bip: BuiltInPredicate, ss: &'a Rc<SubstitutionSet<'a>>)
                     -> Option<Rc<SubstitutionSet<'a>>> {

    if let Some(terms) = bip.terms {

        if terms.len() != 3 { panic!("bip_include() - Requires 3 arguments."); }
        let filtered_list = filter(&terms[0], &terms[1], ss, true)?;
        let out = &terms[2];
        return out.unify(&filtered_list, &Rc::clone(&ss));
    }
    panic!("bip_include() - Requires 3 arguments.");

} // bip_include()


/// Filters a Suiron list to exclude terms which match the filter term.
///
/// In Suiron source code, the exclude() predicate requires three arguments.
/// <pre>
///    exclude(female($_), $InList, $OutList)
/// </pre>
/// The first argument is the filter term. Terms in the list which can unify
/// with the filter term are excluded from the output. The second argument is
/// the input term, which should be bound to a list. The last term is the
/// output term, which unifies with the new filtered list.
///
/// This function is called from
/// [next_solution_bip()](../built_in_predicates/fn.next_solution_bip.html)
/// in built_in_predicates.rs.
///
/// # Arguments
/// * [BuiltInPredicate](../built_in_predicates/struct.BuiltInPredicate.html)
/// * [SubstitutionSet](../substitution_set/type.SubstitutionSet.html)
/// # Return
/// * [SubstitutionSet](../substitution_set/type.SubstitutionSet.html) or None
///
pub fn bip_exclude<'a>(bip: BuiltInPredicate, ss: &'a Rc<SubstitutionSet<'a>>)
                     -> Option<Rc<SubstitutionSet<'a>>> {

    if let Some(terms) = bip.terms {

        if terms.len() != 3 { panic!("bip_exclude() - Requires 3 arguments."); }
        let filtered_list = filter(&terms[0], &terms[1], ss, false)?;
        let out = &terms[2];
        return out.unify(&filtered_list, &Rc::clone(&ss));
    }
    panic!("bip_exclude() - Requires 3 arguments.");

} // bip_exclude()
