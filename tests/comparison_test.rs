// Test comparison predicates.
//
// Test the built-in comparison predicates: > >= == <= <
// Eg.: .., $X <= 23,...
//
// Cleve Lendon  2023

use std::rc::Rc;

use suiron::*;

#[test]
pub fn test_comparison() {

    const S_TIMEOUT: u64 = 1000;

    let mut kb = KnowledgeBase::new();

    fn x() -> Unifiable { logic_var!("$X") }
    fn y() -> Unifiable { logic_var!("$Y") }
    fn z() -> Unifiable { logic_var!("$Z") }

    // test_greater_than($X, $Y, $Z) :- $X > $Y, !, $Z = passed.
    let test_greater_than = atom!("test_greater_than");
    let head = scomplex!(test_greater_than, x(), y(), z());
    let p1 = pred!("greater_than", x(), y());
    let p2 = pred!("!");
    let p3 = pred!("unify", z(), atom!("passed"));
    let body = and_goal!(p1, p2, p3);
    let r1 = make_rule(head, body);

    // test_greater_than($_, $_, $Z) :- $Z = failed.
    let test_greater_than = atom!("test_greater_than");
    let head = scomplex!(test_greater_than, anon!(), anon!(), z());
    let body = pred!("unify", z(), atom!("failed"));
    let r2 = make_rule(head, body);

    // test_less_than($X, $Y, $Z) :- $X < $Y, !, $Z = passed.
    let test_less_than = atom!("test_less_than");
    let head = scomplex!(test_less_than, x(), y(), z());
    let p1 = pred!("less_than", x(), y());
    let p2 = pred!("!");
    let p3 = pred!("unify", z(), atom!("passed"));
    let body = and_goal!(p1, p2, p3);
    let r3 = make_rule(head, body);

    // test_less_than($_, $_, $Z) :- $Z = failed.
    let test_less_than = atom!("test_less_than");
    let head = scomplex!(test_less_than, anon!(), anon!(), z());
    let body = pred!("unify", z(), atom!("failed"));
    let r4 = make_rule(head, body);

    // test_greater_than_or_equal($X, $Y, $Z) :- $X >= $Y, !, $Z = passed.
    let test_greater_than_or_equal = atom!("test_greater_than_or_equal");
    let head = scomplex!(test_greater_than_or_equal, x(), y(), z());
    let p1 = pred!("greater_than_or_equal", x(), y());
    let p2 = pred!("!");
    let p3 = pred!("unify", z(), atom!("passed"));
    let body = and_goal!(p1, p2, p3);
    let r5 = make_rule(head, body);

    // test_greater_than_or_equal($_, $_, $Z) :- $Z = failed.
    let test_greater_than_or_equal = atom!("test_greater_than_or_equal");
    let head = scomplex!(test_greater_than_or_equal, anon!(), anon!(), z());
    let body = pred!("unify", z(), atom!("failed"));
    let r6 = make_rule(head, body);

    // test_less_than_or_equal($X, $Y, $Z) :- $X <= $Y, !, $Z = passed.
    let test_less_than_or_equal = atom!("test_less_than_or_equal");
    let head = scomplex!(test_less_than_or_equal, x(), y(), z());
    let p1 = pred!("less_than_or_equal", x(), y());
    let p2 = pred!("!");
    let p3 = pred!("unify", z(), atom!("passed"));
    let body = and_goal!(p1, p2, p3);
    let r7 = make_rule(head, body);

    // test_less_than_or_equal($_, $_, $Z) :- $Z = failed.
    let test_less_than_or_equal = atom!("test_less_than_or_equal");
    let head = scomplex!(test_less_than_or_equal, anon!(), anon!(), z());
    let body = pred!("unify", z(), atom!("failed"));
    let r8 = make_rule(head, body);

    // test_equal($X, $Y, $Z) :- $X == $Y, !, $Z = passed.
    let test_equal = atom!("test_equal");
    let head = scomplex!(test_equal, x(), y(), z());
    let p1 = pred!("equal", x(), y());
    let p2 = pred!("!");
    let p3 = pred!("unify", z(), atom!("passed"));
    let body = and_goal!(p1, p2, p3);
    let r9 = make_rule(head, body);

    // test_equal($_, $_, $Z) :- $Z = failed.
    let test_equal = atom!("test_equal");
    let head = scomplex!(test_equal, anon!(), anon!(), z());
    let body = pred!("unify", z(), atom!("failed"));
    let r10 = make_rule(head, body);

    add_rules!(&mut kb, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10);

    // test($Z) :- test_greater_than(4, 3, $Z).
    let head = scomplex!(atom!("test"), z());
    let body = pred!("test_greater_than", SInteger(4), SInteger(3), z());
    let r11 = make_rule(head, body);

    // test($Z) :- test_greater_than(Beth, Albert, $Z).
    let head = scomplex!(atom!("test"), z());
    let body = pred!("test_greater_than", atom!("Beth"), atom!("Albert"), z());
    let r12 = make_rule(head, body);

    // test($Z) :- test_greater_than(2, 3, $Z).
    let head = scomplex!(atom!("test"), z());
    let body = pred!("test_greater_than", SInteger(2), SInteger(3), z());
    let r13 = make_rule(head, body);

    // test($Z) :- test_less_than(1.6, 7.2, $Z).
    let head = scomplex!(atom!("test"), z());
    let body = pred!("test_less_than", SFloat(1.6), SFloat(7.2), z());
    let r14 = make_rule(head, body);

    // test($Z) :- test_less_than(Samatha, Trevor, $Z).
    let head = scomplex!(atom!("test"), z());
    let body = pred!("test_less_than", atom!("Samantha"), atom!("Trevor"), z());
    let r15 = make_rule(head, body);

    // test($Z) :- test_less_than(4.222, 4., $Z).
    let head = scomplex!(atom!("test"), z());
    let body = pred!("test_less_than", SFloat(4.222), SFloat(4.), z());
    let r16 = make_rule(head, body);

    // test($Z) :- test_greater_than_or_equal(4.0, 4, $Z).
    let head = scomplex!(atom!("test"), z());
    let body = pred!("test_greater_than_or_equal", SFloat(4.0), SInteger(4), z());
    let r17 = make_rule(head, body);

    // test($Z) :- test_greater_than_or_equal(Joseph, Joseph, $Z).
    let head = scomplex!(atom!("test"), z());
    let body = pred!("test_greater_than_or_equal", atom!("Joseph"), atom!("Joseph"), z());
    let r18 = make_rule(head, body);

    // test($Z) :- test_greater_than_or_equal(3.9, 4.0, $Z).
    let head = scomplex!(atom!("test"), z());
    let body = pred!("test_greater_than_or_equal", SFloat(3.9), SFloat(4.0), z());
    let r19 = make_rule(head, body);

    // test($Z) :- test_less_than_or_equal(7.000, 7, $Z).
    let head = scomplex!(atom!("test"), z());
    let body = pred!("test_less_than_or_equal", SFloat(7.000), SInteger(7), z());
    let r20 = make_rule(head, body);

    // test($Z) :- test_less_than_or_equal(7.000, 7.1, $Z).
    let head = scomplex!(atom!("test"), z());
    let body = pred!("test_less_than_or_equal", SFloat(7.000), SFloat(7.1), z());
    let r21 = make_rule(head, body);

    // test($Z) :- test_less_than_or_equal(0.0, -20, $Z).
    let head = scomplex!(atom!("test"), z());
    let body = pred!("test_less_than_or_equal", SFloat(0.0), SInteger(-20), z());
    let r22 = make_rule(head, body);

    // test($Z) :- test_equal(Joseph, Joseph, $Z).
    let head = scomplex!(atom!("test"), z());
    let body = pred!("test_equal", atom!("Joseph"), atom!("Joseph"), z());
    let r23 = make_rule(head, body);

    // test($Z) :- test_equal(Joseph, Trevor, $Z).
    let head = scomplex!(atom!("test"), z());
    let body = pred!("test_equal", atom!("Joseph"), atom!("Trevor"), z());
    let r24 = make_rule(head, body);

    add_rules!(&mut kb, r11, r12, r13, r14, r15, r16, r17);
    add_rules!(&mut kb, r18, r19, r20, r21, r22, r23, r24);

    // ?- test($Z).
    clear_id();
    let query = query!(atom!("test"), z());
    let sn = make_base_node(Rc::clone(&query), &kb);

    let mut results: Vec<String> = vec![];
    let timer = start_query_timer(S_TIMEOUT);

    loop {

        let solution = next_solution(Rc::clone(&sn));
        if query_stopped() { break; }

        match solution {
           Some(ss) => {
               match query.get_ground_term(1, ss) {
                   Some(result) => {
                       let result = format!("{result}");
                       results.push(result);
                   },
                   None => { },
               }
           },
           None => { break; },
        } // match solution

    } // loop

    cancel_timer(timer);
    if query_stopped() {
        let s = format!("Query timed out after {} milliseconds.", S_TIMEOUT);
        results.push(s);
    }

    let s = "[\"passed\", \"passed\", \"failed\", \
              \"passed\", \"passed\", \"failed\", \
              \"passed\", \"passed\", \"failed\", \
              \"passed\", \"passed\", \"failed\", \
              \"passed\", \"failed\"]";

    assert_eq!(s, format!("{:?}", results));

} // test_backchaining()
