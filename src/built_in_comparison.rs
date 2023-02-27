//! Suiron's comparison functions: equal, less_than, greater_than, etc.
//!
//! The functions defined in this module support Suiron's built-in
//! arithmetic functions.<br>
//! They are called from
//! [unify_sfunction()](../built_in_functions/fn.unify_sfunction.html#)
//! in built_in_functions.rs.

use std::rc::Rc;
use std::cmp::Ordering;
use super::substitution_set::*;
use super::unifiable::{*, Unifiable::*};

/// Compares two strings or two numbers. Succeeds if equal.
///
/// If one argument is an integer, and the other is a float,
/// the integer is converted to float for the comparison.
///
/// Arguments must be Atoms, SFloats or SIntegers. If one of
/// the arguments is a LogicVar, the function fetches the
/// ground term, if there is one.
///
/// # Arguments
/// * `args` - vector of [Unifiable](../unifiable/enum.Unifiable.html) terms (2)
/// * `ss` - [SubstitutionSet](../substitution_set/type.SubstitutionSet.html)
/// # Return
/// * `Option` -
/// Some([SubstitutionSet](../substitution_set/type.SubstitutionSet.html))
/// or None
pub fn bip_equal<'a>(args: Vec<Unifiable>,
                     ss: &'a Rc<SubstitutionSet<'a>>)
                     -> Option<Rc<SubstitutionSet<'a>>> {

    let two_terms: (Unifiable, Unifiable);
    match get_two_constants(args, ss) {
        None => { return None; },
        Some(t) => { two_terms = t; },
    }

    match two_terms {
        (Atom(s1), Atom(s2)) => {
            if s1 == s2 { return Some(Rc::clone(&ss)); }
        },
        (SInteger(i1), SInteger(i2)) => {
            if i1 == i2 { return Some(Rc::clone(&ss)); }
        },
        (SFloat(f1), SFloat(f2)) => {
            if f1 == f2 { return Some(Rc::clone(&ss)); }
        },
        (SFloat(f1), SInteger(i)) => {
            let f2 = i as f64;
            if f1 == f2 { return Some(Rc::clone(&ss)); }
        },
        (SInteger(i), SFloat(f2)) => {
            let f1 = i as f64;
            if f1 == f2 { return Some(Rc::clone(&ss)); }
        },
        _ => {},
    }
    return None;

} // bip_equal()

/// Compares strings or numbers. Succeeds if first < second.
///
/// If one argument is an integer, and the other is a float,
/// the integer is converted to float for the comparison.
///
/// Arguments must be Atoms, SFloats or SIntegers. If one of
/// the arguments is a LogicVar, the function fetches the
/// ground term, if there is one.
///
/// # Arguments
/// * `args` - vector of [Unifiable](../unifiable/enum.Unifiable.html) terms (2)
/// * `ss` - [SubstitutionSet](../substitution_set/type.SubstitutionSet.html)
/// # Return
/// * `Option` -
/// Some([SubstitutionSet](../substitution_set/type.SubstitutionSet.html))
/// or None
pub fn bip_less_than<'a>(args: Vec<Unifiable>,
                         ss: &'a Rc<SubstitutionSet<'a>>)
                         -> Option<Rc<SubstitutionSet<'a>>> {

    let two_terms: (Unifiable, Unifiable);
    match get_two_constants(args, ss) {
        None => { return None; },
        Some(t) => { two_terms = t; },
    }

    match two_terms {
        (Atom(s1), Atom(s2)) => {
            if s1.cmp(&s2) == Ordering::Less {
                return Some(Rc::clone(&ss));
            }
        },
        (SInteger(i1), SInteger(i2)) => {
            if i1.cmp(&i2) == Ordering::Less {
                return Some(Rc::clone(&ss));
            }
        },
        (SFloat(f1), SFloat(f2)) => {
            if f1 < f2 { return Some(Rc::clone(&ss)); }
        },
        (SFloat(f1), SInteger(i)) => {
            let f2 = i as f64;
            if f1 < f2 { return Some(Rc::clone(&ss)); }
        },
        (SInteger(i), SFloat(f2)) => {
            let f1 = i as f64;
            if f1 < f2 { return Some(Rc::clone(&ss)); }
        },
        _ => {},
    }
    return None;

} // bip_less_than()


/// Compares strings or numbers. Succeeds if first <= second.
///
/// If one argument is an integer, and the other is a float,
/// the integer is converted to float for the comparison.
///
/// Arguments must be Atoms, SFloats or SIntegers. If one of
/// the arguments is a LogicVar, the function fetches the
/// ground term, if there is one.
///
/// # Arguments
/// * `args` - vector of [Unifiable](../unifiable/enum.Unifiable.html) terms (2)
/// * `ss` - [SubstitutionSet](../substitution_set/type.SubstitutionSet.html)
/// # Return
/// * `Option` -
/// Some([SubstitutionSet](../substitution_set/type.SubstitutionSet.html))
/// or None
pub fn bip_less_than_or_equal<'a>(args: Vec<Unifiable>,
                                  ss: &'a Rc<SubstitutionSet<'a>>)
                                  -> Option<Rc<SubstitutionSet<'a>>> {

    let two_terms: (Unifiable, Unifiable);
    match get_two_constants(args, ss) {
        None => { return None; },
        Some(t) => { two_terms = t; },
    }

    match two_terms {
        (Atom(s1), Atom(s2)) => {
            let res = s1.cmp(&s2);
            if  res == Ordering::Less ||
                res == Ordering::Equal {
                return Some(Rc::clone(&ss));
            }
        },
        (SInteger(i1), SInteger(i2)) => {
            let res = i1.cmp(&i2);
            if  res == Ordering::Less ||
                res == Ordering::Equal {
                return Some(Rc::clone(&ss));
            }
        },
        (SFloat(f1), SFloat(f2)) => {
            if f1 <= f2 { return Some(Rc::clone(&ss)); }
        },
        (SFloat(f1), SInteger(i)) => {
            let f2 = i as f64;
            if f1 <= f2 { return Some(Rc::clone(&ss)); }
        },
        (SInteger(i), SFloat(f2)) => {
            let f1 = i as f64;
            if f1 <= f2 { return Some(Rc::clone(&ss)); }
        },
        _ => {},
    }
    return None;

} // bip_less_than_or_equal()


/// Compares strings or numbers. Succeeds if first > second.
///
/// If one argument is an integer, and the other is a float,
/// the integer is converted to float for the comparison.
///
/// Arguments must be Atoms, SFloats or SIntegers. If one of
/// the arguments is a LogicVar, the function fetches the
/// ground term, if there is one.
///
/// # Arguments
/// * `args` - vector of [Unifiable](../unifiable/enum.Unifiable.html) terms (2)
/// * `ss` - [SubstitutionSet](../substitution_set/type.SubstitutionSet.html)
/// # Return
/// * `Option` -
/// Some([SubstitutionSet](../substitution_set/type.SubstitutionSet.html))
/// or None
pub fn bip_greater_than<'a>(args: Vec<Unifiable>,
                            ss: &'a Rc<SubstitutionSet<'a>>)
                            -> Option<Rc<SubstitutionSet<'a>>> {

    let two_terms: (Unifiable, Unifiable);
    match get_two_constants(args, ss) {
        None => { return None; },
        Some(t) => { two_terms = t; },
    }

    match two_terms {
        (Atom(s1), Atom(s2)) => {
            if s1.cmp(&s2) == Ordering::Greater {
                return Some(Rc::clone(&ss));
            }
        },
        (SInteger(i1), SInteger(i2)) => {
            if i1.cmp(&i2) == Ordering::Greater {
                return Some(Rc::clone(&ss));
            }
        },
        (SFloat(f1), SFloat(f2)) => {
            if f1 > f2 { return Some(Rc::clone(&ss)); }
        },
        (SFloat(f1), SInteger(i)) => {
            let f2 = i as f64;
            if f1 > f2 { return Some(Rc::clone(&ss)); }
        },
        (SInteger(i), SFloat(f2)) => {
            let f1 = i as f64;
            if f1 > f2 { return Some(Rc::clone(&ss)); }
        },
        _ => {},
    }
    return None;

} // bip_greater_than()

/// Compares strings or numbers. Succeeds if first >= second.
///
/// If one argument is an integer, and the other is a float,
/// the integer is converted to float for the comparison.
///
/// Arguments must be Atoms, SFloats or SIntegers. If one of
/// the arguments is a LogicVar, the function fetches the
/// ground term, if there is one.
///
/// # Arguments
/// * `args` - vector of [Unifiable](../unifiable/enum.Unifiable.html) terms (2)
/// * `ss` - [SubstitutionSet](../substitution_set/type.SubstitutionSet.html)
/// # Return
/// * `Option` -
/// Some([SubstitutionSet](../substitution_set/type.SubstitutionSet.html))
/// or None
pub fn bip_greater_than_or_equal<'a>(args: Vec<Unifiable>,
                                     ss: &'a Rc<SubstitutionSet<'a>>)
                                     -> Option<Rc<SubstitutionSet<'a>>> {

    let two_terms: (Unifiable, Unifiable);
    match get_two_constants(args, ss) {
        None => { return None; },
        Some(t) => { two_terms = t; },
    }

    match two_terms {
        (Atom(s1), Atom(s2)) => {
            let res = s1.cmp(&s2);
            if  res == Ordering::Greater ||
                res == Ordering::Equal {
                return Some(Rc::clone(&ss));
            }
        },
        (SInteger(i1), SInteger(i2)) => {
            let res = i1.cmp(&i2);
            if  res == Ordering::Greater ||
                res == Ordering::Equal {
                return Some(Rc::clone(&ss));
            }
        },
        (SFloat(f1), SFloat(f2)) => {
            if f1 >= f2 { return Some(Rc::clone(&ss)); }
        },
        (SFloat(f1), SInteger(i)) => {
            let f2 = i as f64;
            if f1 >= f2 { return Some(Rc::clone(&ss)); }
        },
        (SInteger(i), SFloat(f2)) => {
            let f1 = i as f64;
            if f1 >= f2 { return Some(Rc::clone(&ss)); }
        },
        _ => {},
    }
    return None;

} // bip_greater_than_or_equal()


/// Gets two constants (Atom, SFloat or SInteger).
///
/// If an argument is a logic variable, get the ground term.
/// If one of the ground terms is not a constant, return None.
/// # Arguments
/// * `list of arguments`
/// * `ss` - [SubstitutionSet](../substitution_set/type.SubstitutionSet.html)
/// # Return
/// * `Option` - Some((constant1, constant2)) or None
fn get_two_constants<'a>(args: Vec<Unifiable>, ss: &'a Rc<SubstitutionSet<'a>>)
                         -> Option<(Unifiable, Unifiable)> {

    let left:  Unifiable;
    let right: Unifiable;

    match get_constant(&args[0], ss) {
        Some(term) => { left = term.clone(); },
        None => { return None; },
    }

    match get_constant(&args[1], ss) {
        Some(term) => { right = term.clone(); },
        None => { return None; },
    }

    return Some((left, right));

} // get_two_constants()
