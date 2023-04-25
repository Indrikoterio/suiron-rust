// Test the built-in Fail predicate.
//
// Fail causes the rule to fail. It is used to force backtracking.
// The test case is:
//
// num(1).
// num(2).
// num(3).
// test($X) :- num($X), print($X), fail.
// test($X) :- nl, $X = 4.
//
// ?- test($X).
//
// This test should print 123.
// The result of the query will be: $X = 4
//
// Cleve Lendon  2023

use std::rc::Rc;
use suiron::*;

#[test]
pub fn test_fail() {

    let c1 = scomplex!(atom!("num"), SInteger(1));
    let c2 = scomplex!(atom!("num"), SInteger(2));
    let c3 = scomplex!(atom!("num"), SInteger(3));

    let f1 = make_fact(c1);
    let f2 = make_fact(c2);
    let f3 = make_fact(c3);

    fn x() -> Unifiable { logic_var!("$X") }
    let p1 = pred!("num", x());
    let p2 = pred!("print", x());
    let p3 = pred!("fail");

    let body = and_goal!(p1, p2, p3);
    let head = scomplex!(atom!("test"), x());
    let r1   = make_rule(head, body);

    let p1 = pred!("nl");
    let p2 = unify!(x(), SInteger(4));

    let body = and_goal!(p1, p2);
    let head = scomplex!(atom!("test"), x());
    let r2   = make_rule(head, body);

    let mut kb = KnowledgeBase::new();
    add_rules!(&mut kb, f1, f2, f3, r1, r2);
    print_kb(&kb);

    // ?-  test($X).
    let query = query!(atom!("test"), x());
    let sn = make_base_node(query, &kb);
    let result = solve(sn);

    assert_eq!("$X = 4", result);

} // test_fail()
