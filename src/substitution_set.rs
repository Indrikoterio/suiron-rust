//! A substitution set is an array of bindings for logic variables.
//!
//! As the inference engine attempts to solve a goal (or query), it
//! generates substitution sets, which record logic variable bindings.
//! A substitution set can be thought of as a solution, or partial
//! solution, for a given goal.
//
// Cleve Lendon 2023

use std::rc::Rc;

use super::unifiable::{*, Unifiable::*};

/// Records bindings of logic variables to unifiable terms.
///
/// Logic variable IDs are used to index into the substitution set.
///
/// [substitution_set](../substitution_set/index.html)
///
// Note:
// A vector is heap allocated, so why is the Box necessary?
// One of the speed bottlenecks of the inference engine is the
// time it takes to copy an entire substitution set. The size
// of Unifiable is 56 bytes, while the size of a Box pointer is
// only 8.
// Also note:
// It is not possible to add methods, because SubstitutionSet
// is an alias of built-in data types.
pub type SubstitutionSet<'a> = Vec<Option<Rc<Unifiable>>>;

/// Is the logic variable bound?
///
/// A logic variable is bound if an entry for it exists in the substitution set.
///
/// # Arguments
/// * `term` - must be a [LogicVar](../unifiable/enum.Unifiable.html#variant.LogicVar) 
/// * `ss`  - substitution set
/// # Return
/// * True if bound. False otherwise.
/// # Panics
/// * If `term` is not a logic variable.
/// # Usage
/// ```
/// use std::rc::Rc;
/// use suiron::*;
///
/// let a = atom!("Promethium");
/// let x = logic_var!(next_id(), "$X");
///
/// let ss = empty_ss!();
/// let ss = x.unify(&a, &ss).unwrap();   // x -> a
///
/// if is_bound(&x, &ss) { println!("X is bound."); }
/// ```
pub fn is_bound(term: &Unifiable, ss: &SubstitutionSet) -> bool {
    if let LogicVar{id, name: _} = *term {
        if id >= ss.len() { return false; }
        return ss[id] != None;
    }
    else {
        panic!("is_bound() - First argument must be a logic variable.");
    }
} // is_bound()

/// Gets the term which a logic variable is bound to.
///
/// If the variable is not bound, returns None.
///
/// # Notes
/// * The bound term is not necessarily a ground term.<br>
/// It might be another variable.
/// * This function is used only for debugging and testing.
///
/// # Arguments
/// * `term` - must be a [LogicVar](../unifiable/enum.Unifiable.html#variant.LogicVar) 
/// * `ss`  - substitution set
/// # Return
/// * `bound term` - [Unifiable](../unifiable/enum.Unifiable.html)
/// # Panics
/// * If `term` is not a logic variable.
/// # Usage
/// ```
/// use std::rc::Rc;
/// use suiron::*;
///
/// clear_id();
/// let ne = atom!("Neon");
/// let x = logic_var!(next_id(), "$X");
/// let y = logic_var!(next_id(), "$Y");
/// let ss = empty_ss!();
///
/// // Bind $X_1 -> Neon
/// let ss = x.unify(&ne, &ss).unwrap();
/// // Bind $Y_2 -> $X_1 -> Neon
/// let ss = y.unify(&x, &ss).unwrap();
///
/// let binding = get_binding(&y, &ss);
/// match binding {
///     None => { panic!("$Y is not bound."); },
///     Some(entry) => {
///         let s = format!("{}", *entry);
///         println!("{}", s);  // Prints: $X_1
///     },
/// }
/// ```
///
pub fn get_binding<'a>(term: &Unifiable, ss: &'a SubstitutionSet)
                       -> Option<&'a Unifiable> {
    if let LogicVar{id, name: _} = *term {
        if id >= ss.len() { return None; }
        match &ss[id] {
            None => { return None; },
            Some(entry) => { return Some(&*entry); },
        }
    }
    else { panic!("get_binding() - First argument must be a logic variable."); }
} // get_binding()


/// Is the logic variable ultimately bound to a ground term?
///
/// If the given logic variable is bound to a ground term, return true.
/// If the logic variable is bound to another logic variable, follow the
/// bindings in the substitution set until a ground term or None is found.
///
/// # Arguments
/// * `term` - must be a
/// [LogicVar](../unifiable/enum.Unifiable.html#variant.LogicVar) 
/// * `ss`  - substitution set
/// # Return
/// * True if ground. False otherwise.
/// # Panics
/// * If `term` is not a logic variable.
/// # Usage
/// ```
/// use std::rc::Rc;
/// use suiron::*;
///
/// let a = atom!("Promethium");
/// let x = logic_var!(next_id(), "$X");
/// let y = logic_var!(next_id(), "$Y");
/// let z = logic_var!(next_id(), "$Z");
///
/// let ss = empty_ss!();
///
/// let ss = x.unify(&a, &ss).unwrap(); // x -> a
/// let ss = y.unify(&x, &ss).unwrap(); // y -> x -> a
/// let ss = z.unify(&y, &ss).unwrap(); // z -> y -> x -> a
///
/// if is_ground_variable(&z, &ss) { println!("Z is grounded."); }
/// else { println!("Z is NOT grounded."); }
/// // Prints: Z is grounded.
/// ```
pub fn is_ground_variable(term: &Unifiable, ss: &SubstitutionSet) -> bool {
    if let LogicVar{id, name: _} = *term {
        let mut id = id;
        loop {
            if id >= ss.len() { return false; }
            match &ss[id] {
                None => { return false; },
                Some(term) => {
                    if let LogicVar{id: id2, name: _} = **term { id = id2; }
                    else { return true; }
                },
            } // match
        } // loop
    }
    else {
        panic!("is_ground_variable() - First argument must be a logic variable.");
    }
} // is_ground_variable()


/// Gets the ground term which a logic variable is ultimately bound to.
///
/// If the given term is already a ground term, simply return it.
/// If the given term is a logic variable, check the substitution set to
/// determine whether it is bound to a ground term.
/// If it is, return the ground term.
/// If the logic variable is bound to another variable, keep checking the
/// substitution set until a ground term is found, or
/// [Nil](../unifiable/enum.Unifiable.html#variant.Nil).
/// # Note
/// In Prolog, 'ground term' refers to a term which has no logic variables
/// in it. (Complex terms can contain constants and/or variables.)
/// Here, however, 'ground term' simply means 'not a logic variable'.
/// # Arguments
/// * `term`  - [Unifiable](../unifiable/enum.Unifiable.html)
/// * `ss`  - substitution set
/// # Return
/// * `Option` - Some([Unifiable](../unifiable/enum.Unifiable.html)) or None
/// # Usage
/// ```
/// use std::rc::Rc;
/// use suiron::*;
///
/// let a = atom!("Promethium");
/// let x = logic_var!(next_id(), "$X");
/// let y = logic_var!(next_id(), "$Y");
/// let z = logic_var!(next_id(), "$Z");
///
/// let ss = empty_ss!();
///
/// let ss = x.unify(&a, &ss).unwrap(); // x -> a
/// let ss = y.unify(&x, &ss).unwrap(); // y -> x -> a
/// let ss = z.unify(&y, &ss).unwrap(); // z -> y -> x -> a
///
/// let gt = get_ground_term(&z, &ss).unwrap();
/// println!("{}", gt); // Prints: Promethium
/// ```
pub fn get_ground_term<'a>(term: &'a Unifiable, ss: &'a SubstitutionSet)
                           -> Option<&'a Unifiable> {
    let mut term2 = term;
    loop {
        if let LogicVar{id, name: _} = *term2 {
            if id >= ss.len() { return None; }
            match &ss[id] {
                None => { return None; },
                Some(entry) => { term2 = &*entry; },
            }
        }
        else { return Some(term2); }
    } // loop
} // get_ground_term()


/// Checks if a term is a complex term or bound to a complex term.
///
/// If the given term is a complex term, simply return it.
/// If the given term is a logic variable, check the substitution
/// set to determine whether it is ultimately bound to a complex term.
/// If it is, return the complex term. Otherwise, return None.
/// # Note
/// In other implementations of Suiron, this function is equivalent to cast_complex().
/// # Arguments
/// * `term`  - [Unifiable](../unifiable/enum.Unifiable.html)
/// * `ss`  - substitution set
/// # Return
/// * `Option` - Some([SComplex](../unifiable/enum.Unifiable.html#variant.SComplex)) or None
/// # Usage
/// ```
/// use std::rc::Rc;
/// use suiron::*;
///
/// let zol = parse_complex("music(Dilnaz, Zolotoi)").unwrap();
/// let x = logic_var!(next_id(), "$X");
/// let y = logic_var!(next_id(), "$Y");
/// let ss = empty_ss!();
/// let ss = x.unify(&zol, &ss).unwrap();  // $X -> music()
/// let ss = y.unify(&x, &ss).unwrap();    // $Y -> $X -> music()
///
/// let result = get_complex(&y, &ss).unwrap();
/// println!("{}", result);  // Prints: music(Dilnaz, Zolotoi)
/// ```
///
pub fn get_complex<'a>(term: &'a Unifiable, ss: &'a SubstitutionSet) -> Option<&'a Unifiable> {
    match *term {
        Unifiable::SComplex(_) => { return Some(term); },
        Unifiable::LogicVar{id: _, name: _} => {
            match get_ground_term(term, ss) {
                Some(gt) => {
                    match gt {
                        Unifiable::SComplex(_) => { return Some(gt); },
                        _ => None,
                    }
                },
                None => None,
            }
        },
        _ => None,
    }
} // get_complex()

/// Checks if a term is a list or bound to a list.
///
/// If the given term is a list, simply return it.
/// If the given term is a logic variable, check the substitution
/// set to determine whether it is ultimately bound to a list.
/// If it is, return the list. Otherwise, return None.
/// # Note
/// In other implementations of Suiron, this function is equivalent
/// to cast_linked_list().
/// # Arguments
/// * `term`  - [Unifiable](../unifiable/enum.Unifiable.html)
/// * `ss`  - substitution set
/// # Return
/// * `Option` -
///    Some([SLinkedList](../unifiable/enum.Unifiable.html#variant.SLinkedList))
///    or None
/// # Usage
/// ```
/// use std::rc::Rc;
/// use suiron::*;
///
/// // First make a list.
/// let ar = atom!("Argon");
/// let kr = atom!("Krypton");
/// let l1 = slist!(false, ar, kr);  // [Argon, Krypton]
///
/// // Unify $X with [Argon, Krypton].
/// let x = logic_var!(next_id(), "$X");
/// let y = logic_var!(next_id(), "$Y");
/// let ss = empty_ss!();
/// let ss = x.unify(&l1, &ss).unwrap();  // $X -> [Argon, Krypton]
/// let ss = y.unify(&x, &ss).unwrap();  // $Y -> $X -> [Argon, Krypton]
///
/// let result = get_list(&y, &ss).unwrap();
/// println!("{}", result);  // Prints: [Argon, Krypton]
/// ```
///
pub fn get_list<'a>(term: &'a Unifiable, ss: &'a SubstitutionSet) -> Option<&'a Unifiable> {
    match *term {
        Unifiable::SLinkedList{term: _, next: _, count: _, tail_var: _}
            => { return Some(term); },
        Unifiable::LogicVar{id: _, name: _} => {
            match get_ground_term(term, ss) {
                Some(gt) => {
                    match gt {
                        Unifiable::SLinkedList{term: _, next: _, count: _, tail_var: _}
                          => { return Some(gt); },
                        _ => None,
                    }
                },
                None => None,
            }
        },
        _ => None,
    }
} // get_list()

/// Checks whether a term is a constant or bound to a constant.
///
/// If the given term is a simple constant (Atom, SFloat or SInteger),
/// return it. If the given term is a logic variable, check the
/// substitution set to determine whether it is ultimately bound to
/// a constant. If it is, return the constant. Otherwise, return None.
///
/// # Arguments
/// * `term`  - [Unifiable](../unifiable/enum.Unifiable.html)
/// * `ss` - substitution set
/// # Return
/// * `Option` - Some([Unifiable](../unifiable/enum.Unifiable.html)) or None
/// # Usage
/// ```
/// use std::rc::Rc;
/// use suiron::*;
///
/// let kr = atom!("Krypton");
/// let x = logic_var!(next_id(), "$X");
/// let y = logic_var!(next_id(), "$Y");
/// let ss = empty_ss!();
/// let ss = kr.unify(&x, &ss).unwrap(); // $X -> Argon
/// let ss = y.unify(&x, &ss).unwrap(); // $Y -> $X -> Argon
///
/// let result = get_constant(&y, &ss).unwrap();
/// println!("{:?}", result);  // Prints: Atom("Krypton");
/// ```
///
pub fn get_constant<'a>(term: &'a Unifiable, ss: &'a SubstitutionSet) -> Option<&'a Unifiable> {
    match *term {
        Unifiable::SFloat(_) |
        Unifiable::SInteger(_) |
        Unifiable::Atom(_) => { return Some(term); },
        Unifiable::LogicVar{id: _, name: _} => {
            match get_ground_term(term, ss) {
                Some(gt) => {
                    match gt {
                        Unifiable::SFloat(_) |
                        Unifiable::SInteger(_) |
                        Unifiable::Atom(_) => { return Some(gt); },
                        _ => None,
                    }
                },
                None => None,
            }
        },
        _ => None,
    }
} // get_constant()


/// Formats a substitution set for display. Use for debugging.
/// # Note
/// * It is not possible to implement the Display trait for
///   SubstitutionSet, because it is a type alias.
/// # Arguments
/// * `ss` - substitution set
/// # Return
/// * `String`
/// # Usage
/// ```
/// use std::rc::Rc;
/// use suiron::*;
///
/// clear_id();
/// let ss = empty_ss!();
/// let ar = atom!("Argon");
/// let x = logic_var!(next_id(), "$X");
/// let ss = ar.unify(&x, &ss).unwrap();
/// let f = format_ss(&ss);
/// println!("{}", f);
/// ```
/// Prints:
/// <pre>
/// ----- Substitution Set -----
/// 0	None
/// 1	Argon
/// ----------------------------
/// </pre>
pub fn format_ss(ss: &SubstitutionSet) -> String {
    let mut out = "----- Substitution Set -----\n".to_string();
    if ss.len() == 0 { out += "\tEmpty\n"; }
    else {
        for (i, term) in ss.iter().enumerate() {
            match term {
                None => { out += &format!("{}\tNone\n", i); },
                Some(uni) => { out += &format!("{}\t{}\n", i, *uni); },
            }
        }
    }
    out += "----------------------------";
    return out;
} // format_ss()


/// Prints a formatted substitution set. Use for debugging.
/// # Arguments
/// * `ss` - substitution set
/// # Usage
/// ```
/// use std::rc::Rc;
/// use suiron::*;
///
/// clear_id();
/// let ss = empty_ss!();
/// let ar = atom!("Argon");
/// let x = logic_var!(next_id(), "$X");
/// let ss = ar.unify(&x, &ss).unwrap();
/// print_ss(&ss);
/// ```
/// Prints:
/// <pre>
/// ----- Substitution Set -----
/// 0	None
/// 1	Argon
/// ----------------------------
/// </pre>
pub fn print_ss(ss: &SubstitutionSet) {
    println!("{}", format_ss(ss));
} // print_ss()


#[cfg(test)]
mod test {

    use std::rc::Rc;
    use crate::*;

    #[test]
    fn test_format_ss() {

        let mut ss = empty_ss!();

        let s = "----- Substitution Set -----\n\t\
                Empty\n----------------------------";
        assert_eq!(s, format_ss(&ss));

        let ar = atom!("Argon");
        let x = logic_var!(1, "$X");

        // Binding: $X -> Argon
        if let Some(ss2) = ar.unify(&x, &ss) {
            ss = ss2;
        }
        else { panic!("Cannot bind $X = Argon."); }

        let s = "----- Substitution Set -----\n\
                0\tNone\n\
                1\tArgon\n\
                ----------------------------";
        assert_eq!(s, format_ss(&ss));

    } // test_format_ss()

    // Test is_bound(), is_ground_variable(), get_binding(), get_ground_term().
    #[test]
    fn test_bound_and_ground() {

        let a = atom!("Alpha");
        let w = logic_var!(1, "$W");
        let x = logic_var!(2, "$X");
        let y = logic_var!(3, "$Y");
        let z = logic_var!(4, "$Z");

        let mut ss = empty_ss!();

        if let Some(ss2) = x.unify(&w, &ss) { ss = ss2; }
        else { panic!("Cannot unify $X = $W."); }

        if let Some(ss2) = a.unify(&y, &ss) { ss = ss2; }
        else { panic!("Cannot unify $Y = Alpha."); }

        if let Some(ss2) = z.unify(&y, &ss) { ss = ss2; }
        else { panic!("Cannot unify $Z = $Y."); }

        if is_bound(&w, &ss) { panic!("is_bound() - $W should not be bound."); }
        if !is_bound(&x, &ss) { panic!("is_bound() - $X should be bound."); }

        if is_ground_variable(&x, &ss) {
             panic!("is_ground_variable() - $X should not be ground.");
        }
        if !is_ground_variable(&z, &ss) {
             panic!("is_ground_variable() - $Z should be ground.");
        }

        let b = get_binding(&z, &ss);
        match b {
            None => { panic!("get_binding() - $Z should be bound to $Y."); },
            Some(entry) => {
                let s = format!("{}", *entry);
                assert_eq!("$Y_3", s, "get_binding() - $Z should be bound to $Y.");
            },
        }

        if let Some(c) = get_ground_term(&z, &ss) {
            assert_eq!("Alpha", c.to_string(),
                       "get_ground_term() - \
                        The ground term of $Z should be Alpha.");
        }

    } // test_bound_and_ground()

    #[test]
    fn test_get_complex() {

        let ar = atom!("Argon");
        let kr = atom!("Krypton");
        let ne = atom!("Neon");
        let el = atom!("element");
        let c1 = scomplex!(el, ar, SInteger(18));
        let el = atom!("element");
        let c2 = scomplex!(el, kr, SInteger(36));

        let x = logic_var!(1, "$X");
        let y = logic_var!(2, "$Y");

        let mut ss = empty_ss!();

        // Binding: $X -> element(Argon, 18)
        if let Some(ss2) = c1.unify(&x, &ss) { ss = ss2; }
        else { panic!("Cannot bind $X = element(Argon, 18)."); }

        // Binding: $Y -> $X -> element(Argon, 18)
        if let Some(ss2) = y.unify(&x, &ss) { ss = ss2; }
        else { panic!("Cannot bind $Y = $X."); }

        // C2 is a complex term. It should simply be returned.
        if let Some(res1) = get_complex(&c2, &ss) {
            assert_eq!("element(Krypton, 36)", res1.to_string(),
                       "First test - Must return same complex term.");
        }
        else { panic!("First test - element(Krypton, 36) should be returned."); }

        // If term is not a complex term, return None.
        if let Some(res2) = get_complex(&ne, &ss) {
            panic!("Second test - Should return None. Got {}.", res2);
        }

        // The ground term of $Y is a complex term: element(Argon, 18).
        // The function should be able to get this.
        if let Some(res3) = get_complex(&y, &ss) {
            assert_eq!("element(Argon, 18)", res3.to_string(),
                       "Third test - Should get ground term.");
        }
        else { panic!("Third test - Cannot get ground term: element(Argon, 18)."); }
    } // test_get_complex

    #[test]
    fn test_get_list() {

        let ar = atom!("Argon");
        let kr = atom!("Krypton");
        let ne = atom!("Neon");
        let l1 = slist!(false, ar, kr);
        let l2 = slist!();

        let x = logic_var!(1, "$X");
        let y = logic_var!(2, "$Y");

        let mut ss = empty_ss!();

        // Binding: $X -> [Argon, Krypton]
        if let Some(ss2) = l1.unify(&x, &ss) { ss = ss2; }
        else { panic!("Cannot bind $X = [Argon, Krypton]."); }

        // Binding: $Y -> $X -> element(Argon, 18)
        if let Some(ss2) = y.unify(&x, &ss) { ss = ss2; }
        else { panic!("Cannot bind $Y = $X."); }

        // L2 is a list. It should simply be returned.
        if let Some(res1) = get_list(&l2, &ss) {
            assert_eq!("[]", res1.to_string(),
                       "First test - Must return same list.");
        }
        else { panic!("First test - [] should be returned."); }

        // If term is not a list, return None.
        if let Some(res2) = get_list(&ne, &ss) {
            panic!("Second test - Should return None. Got {}.", res2);
        }

        // The ground term of $Y is a list: [Argon, Krypton].
        // The function should be able to get this.
        if let Some(res3) = get_list(&y, &ss) {
            assert_eq!("[Argon, Krypton]", res3.to_string(),
                       "Third test - Should get ground term.");
        }
        else { panic!("Third test - Cannot get ground term: [Argon, Krypton]."); }

    } // test_get_list

    #[test]
    fn test_get_constant() {

        let ar = atom!("Argon");
        let kr = atom!("Krypton");
        let ne = atom!("Neon");
        let l = slist!(false, ne);
        let x = logic_var!(1, "$X");
        let y = logic_var!(2, "$Y");

        let mut ss = empty_ss!();

        // Binding: $X -> Argon
        if let Some(ss2) = ar.unify(&x, &ss) { ss = ss2; }
        else { panic!("Cannot bind $X = Argon."); }

        // Binding: $Y -> $X -> Argon
        if let Some(ss2) = y.unify(&x, &ss) { ss = ss2; }
        else { panic!("Cannot bind $Y = $X."); }

        // Kryton is an atom. It should simply be returned.
        if let Some(res1) = get_constant(&kr, &ss) {
            assert_eq!("Krypton", res1.to_string(),
                       "First test - Must return same atom.");
        }
        else { panic!("First test - Krypton is an atom. It should be returned."); }

        // If term is a list (not an atom), return None.
        if let Some(res2) = get_constant(&l, &ss) {
            panic!("Second test - Should return None. Got {}.", res2);
        }

        // The ground term of $Y is an atom (Argon).
        // The function should be able to get this.
        if let Some(res3) = get_constant(&y, &ss) {
            assert_eq!("Argon", res3.to_string(),
                       "Third test - Should get ground term.");
        }
        else { panic!("Third test - Cannot get ground term (Argon)."); }
    } // test_get_constant

} // test
