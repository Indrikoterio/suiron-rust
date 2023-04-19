// Test the built-in predicate count().
//
// This predicate returns the count of items in a list.
//
// For example:
//
//   test_count($Count) :- count([], $Count).          # $Count is 0
//   test_count($Count) :- count([a, b, c], $Count).   # $Count is 3
//   test_count($Count) :- count([a | $_], $Count).    # $Count is 2
//   test_count($Count) :- count([red | [green, blue]], $Count).    # $Count is 3
//
//   test_count($Count) :- $TailVar = [one, two, three],
//                         $NewList = [red, green, blue | $TailVar],
//                         count($NewList, $Count).    # $Count is 6
//
//   test_count($Count) :- $TailVar = [],
//                         $NewList = [red, green, blue | $TailVar],
//                         count($NewList, $Count).    # $Count is 3
//
// Cleve Lendon  2023

use std::rc::Rc;

use suiron::*;

#[test]
pub fn test_count() {

    const S_TIMEOUT: u64 = 1000;

    let mut kb = KnowledgeBase::new();

    fn c() -> Unifiable { logic_var!("$Count") }

    // test_count($Count) :- count([], $Count).
    let tc = scomplex!(atom!("test_count"), c());
    let count_pred = pred!("count", slist!(), c());
    let r1 = make_rule(tc, count_pred);

    // test_count($Count) :- count([a, b, c], $Count).   # $C is 3
    let tc = scomplex!(atom!("test_count"), c());
    let list = slist!(false, atom!("a"), atom!("b"), atom!("c"));
    let count_pred = pred!("count", list, c());
    let r2 = make_rule(tc, count_pred);

    // test_count($Count) :- count([a | $_], $Count).    # $C is 2
    let tc = scomplex!(atom!("test_count"), c());
    let list = slist!(true, atom!("a"), anon!());
    let count_pred = pred!("count", list, c());
    let r3 = make_rule(tc, count_pred);

    // test_count($Count) :- count([red | [green, blue]], $Count).  # $C is 3
    let tc = scomplex!(atom!("test_count"), c());
    let red   = atom!("red");
    let green = atom!("green");
    let blue  = atom!("blue");
    let list1 = slist!(false, green, blue);
    let list2 = slist!(false, red, list1);
    let count_pred = pred!("count", list2, c());
    let r4 = make_rule(tc, count_pred);

    // test_count($Count) :- $TailVar = [one, two, three],
    //                       $NewList = [red, green, blue | $TailVar],
    //                       count($NewList, $Count).    # $C is 6

    let tc = scomplex!(atom!("test_count"), c());

    fn tail_var() -> Unifiable { logic_var!("$TailVar") }
    let list1 = slist!(false, atom!("one"), atom!("two"), atom!("three"));
    fn new_list() -> Unifiable { logic_var!("$NewList") }
    let list2 = slist!(true,  atom!("red"), atom!("green"), atom!("blue"), tail_var());

    let uni1 = unify!(tail_var(), list1);
    let uni2 = unify!(new_list(), list2);
    let count_pred = pred!("count", new_list(), c());
    let body = and_goal!(uni1, uni2, count_pred);
    let r5 = make_rule(tc, body);

    // test_count($Count) :- $TailVar = [],
    //                       $NewList = [red, green, blue | $TailVar],
    //                       count($NewList, $Count).    # $C is 3

    let tc = scomplex!(atom!("test_count"), c());

    let uni1 = unify!(tail_var(), slist!());
    let list = slist!(true, atom!("red"), atom!("green"), atom!("blue"), tail_var());
    let uni2 = unify!(new_list(), list);
    let count_pred = pred!("count", new_list(), c());
    let body = and_goal!(uni1, uni2, count_pred);
    let r6 = make_rule(tc, body);

    add_rules!(&mut kb, r1, r2, r3, r4, r5, r6);
    //print_kb(&kb);

    // ?- test_count($X).
    clear_id();

    let x = logic_var!("$X");
    let query = query!(atom!("test_count"), x);
    let sn = make_base_node(Rc::clone(&query), &kb);

    let mut results: Vec<i64> = vec![];
    let timer = start_query_timer(S_TIMEOUT);

    loop {

        let solution = next_solution(Rc::clone(&sn));
        if query_stopped() { break; }

        match solution {
           Some(ss) => {
               if let Some(result) = query.get_ground_term(1, ss) {
                   if let SInteger(i) = result {
                       results.push(i);
                   }
               }
           },
           None => { break; },
        } // match solution

    } // loop

    cancel_timer(timer);
    if query_stopped() {
        let s = format!("Query timed out after {} milliseconds.", S_TIMEOUT);
        println!("{s}");
    }

    let results = format!("{:?}", results);
    assert_eq!("[0, 3, 2, 3, 6, 3]", results);

} // test_count()
