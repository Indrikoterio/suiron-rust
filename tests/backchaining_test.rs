// Test Backchaining
//
// Define facts and rules for test:
//
// parent(Charles, Tony).  % Charles is the parent of Tony.
// parent(Bill, Audrey).
// parent(Maria, Bill).
// parent(Tony, Maria).
//
// An ancestor is a parent or the parent of an ancestor:
//
// ancestor($X, $Y) :- parent($X, $Y).
// ancestor($X, $Y) :- parent($X, $Z), ancestor($Z, $Y).
//
// Find out who Charle's descendants are.
//
// ?- ancestor(Charles, $Desc).
//
// Cleve Lendon  2023

use std::rc::Rc;

use suiron::*;

#[test]
pub fn test_backchaining() {

    let mut kb = KnowledgeBase::new();

    fn x() -> Unifiable { logic_var!("$X") }
    fn y() -> Unifiable { logic_var!("$Y") }
    fn z() -> Unifiable { logic_var!("$Z") }

    let c1 = scomplex!(atom!("parent"), atom!("Charles"), atom!("Tony"));
    let c2 = scomplex!(atom!("parent"), atom!("Bill"), atom!("Audrey"));
    let c3 = scomplex!(atom!("parent"), atom!("Maria"), atom!("Bill"));
    let c4 = scomplex!(atom!("parent"), atom!("Tony"), atom!("Maria"));

    // Make facts.
    let f1 = make_fact(c1);
    let f2 = make_fact(c2);
    let f3 = make_fact(c3);
    let f4 = make_fact(c4);

    add_rules!(&mut kb, f1, f2, f3, f4);

    let head = scomplex!(atom!("ancestor"), x(), y());
    let c5 = scomplex!(atom!("parent"), x(), y());

    // ancestor($X, $Y) := parent($X, $Y)
    let r1 = make_rule(head, Goal::ComplexGoal(c5));

    // ancestor($X, $Y) := parent($X, $Z), ancestor($Z, $Y).
    let head = scomplex!(atom!("ancestor"), x(), y());
    let c6 = scomplex!(atom!("parent"), x(), z());
    let c7 = scomplex!(atom!("ancestor"), z(), y());
    let g6 = Goal::ComplexGoal(c6);
    let g7 = Goal::ComplexGoal(c7);
    let body = operator_and!(g6, g7);
    let r2 = make_rule(head, Goal::OperatorGoal(body));

    add_rules!(&mut kb, r1, r2);

    let query = query!(atom!("ancestor"), atom!("Charles"), x());
    let sn = make_base_node(Rc::clone(&query), &kb);

    let solutions = solve_all(sn);
    let s1 = "[\"$X = Tony\", \"$X = Maria\", \"$X = Bill\", \"$X = Audrey\"]";
    let s2 = format!("{:?}", solutions);
    assert_eq!(s1, s2);


} // test_backchaining()
