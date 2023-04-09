// Test creation and unification of constants (Atoms, SIntegers, SFloats).
// Cleve Lendon  2023

use suiron::*;
use std::rc::Rc;

#[test]
pub fn test_atom() {

    let a1 = Atom("This is an atom.".to_string());
    let a2 = Atom("This is an atom.".to_string());
    let a3 = Atom("Just another.".to_string());
    let anon = Unifiable::Anonymous;
    let ss = empty_ss!();

    let ss = match a1.unify(&a2, &ss) {
        Some(ss) => { ss },
        None => { panic!("Unification must succeed: a1 = a2"); },
    };

    let ss = match a1.unify(&anon, &ss) {
        Some(ss) => { ss },
        None => { panic!("Unification must succeed: a1 = $_"); },
    };

    let ss = match a1.unify(&a3, &ss) {
        Some(_ss) => { panic!("Unification must fail: a1 != a3"); },
        None => { ss },
    };

    assert_eq!(ss.len(), 0, "Unification must not change substitution set.");

} // test_atom()

#[test]
pub fn test_integer() {

    let i1 = SInteger(45);
    let i2 = SInteger(45);
    let i3 = SInteger(46);
    let ss = empty_ss!();

    let ss = match i1.unify(&i2, &ss) {
        Some(ss) => { ss },
        None => { panic!("Unification must succeed: i1 = i2"); },
    };

    let ss = match i1.unify(&i3, &ss) {
        Some(_ss) => { panic!("Unification must fail: i1 != i3"); },
        None => { ss },
    };

    assert_eq!(ss.len(), 0, "Unification must not change substitution set.");

} // test_integer()

#[test]
pub fn test_float() {

    let a1 = atom!("An atom.");
    let i1 = SInteger(45);
    let f1 = SFloat(45.0);
    let f2 = SFloat(45.0);
    let f3 = SFloat(45.0000000001);
    let ss = empty_ss!();

    let ss = match f1.unify(&f2, &ss) {
        Some(ss) => { ss },
        None => { panic!("Unification must succeed: f1 = f2"); },
    };

    let ss = match f1.unify(&f3, &ss) {
        Some(_ss) => { panic!("Unification must fail: f1 != f3"); },
        None => { ss },
    };

    let ss = match f1.unify(&a1, &ss) {
        Some(_ss) => { panic!("Unification must fail: f1 != a1"); },
        None => { ss },
    };

    let ss = match f1.unify(&i1, &ss) {
        Some(_ss) => { panic!("Unification must fail: f1 != i1"); },
        None => { ss },
    };

    assert_eq!(ss.len(), 0, "Unification must not change substitution set.");

} // test_float()
