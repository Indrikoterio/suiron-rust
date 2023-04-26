// Test the built-in predicate functor().
//
// Rules and queries:
//
//    get($Y) :- functor(mouse(mammal, rodent), $X), $X = $Y.
//    get($Y) :- $X = cat(mammal, carnivore), functor($X, $Y).
//    ?- get($X).
//
//    Results:
//      $X = mouse
//      $X = cat
//-----------------------------------
//
//    check_arity($X, $Y) :- functor(diamonds(forever, a girl's best friend), $X, $Y).
//    ?- check_arity($X, $Y).
//
//    Results:
//      $X = diamonds, $Y = 2
//
//-----------------------------------
//
//    test_functor($Y) :- $X = symptom(cold, sneezing), functor($X, symptom),
//                        $Y = `Success #1`.
//    test_functor($Y) :- $X = symptom(cold, sneezing), not(functor($X, symptoms)),
//                        $Y = `Success #2`.
//    test_functor($Y) :- $X = symptom(cold, sneezing), functor($X, symp*),
//                        $Y = `Success #3`.
//    ?- test_functor($X).
//
//    Results:
//      $X = Success #1
//      $X = Success #2
//      $X = Success #3
//
// Cleve Lendon  2023

use std::rc::Rc;
use suiron::*;

#[test]
pub fn test_functor() {

    let mut kb = KnowledgeBase::new();

    fn x() -> Unifiable { logic_var!("$X") }
    fn y() -> Unifiable { logic_var!("$Y") }

    // get($Y) :- functor(mouse(mammal, rodent), $X), $X = $Y.
    let g = atom!("get");
    let m = scomplex!(atom!("mouse"), atom!("mammal"), atom!("rodent"));
    let f = pred!("functor", m, x());
    let u = unify!(x(), y());
    let head = scomplex!(g, y());
    let body = and_goal!(f, u);
    let rule1 = make_rule(head, body);

    // Test to see if the inference engine can parse the functor predicate.
    // get($Y) :- $X = cat(mammal, carnivore), functor($X, $Y).
    let rule2 = parse_rule("get($Y) :- $X = cat(mammal, carnivore), functor($X, $Y).");
    let rule2 = match rule2 {
        Ok(r) => { r },
        Err(err) => { panic!("{}", err); },
    };

    add_rules!(&mut kb, rule1, rule2);

    let query = query!(atom!("get"), x());
    let sn = make_base_node(query, &kb);
    let solutions = solve_all(sn);

    assert_eq!("$X = mouse", solutions[0]);
    assert_eq!("$X = cat", solutions[1]);

    //----------------------------------------------
    // Check to make sure we can get the arity also.
    // check_arity(X, Y) :- functor(diamonds(forever, a girl's best friend), X, Y).

    let mineral = parse_complex("diamonds(forever, a girl's best friend)").unwrap();
    let check_arity = atom!("check_arity");
    let head = scomplex!(check_arity, x(), y());
    let body = pred!("functor", mineral, x(), y());
    let rule3 = make_rule(head, body);

    add_rules!(&mut kb, rule3);

    let query = query!(atom!("check_arity"), x(), y());
    let sn = make_base_node(query, &kb);
    let solution = solve(sn);

    assert_eq!("$X = diamonds, $Y = 2", solution);


    //----------------------------------------------
    // Another test.

    let symptom = parse_complex("symptom(cold, sneezing)").unwrap();
    let head = scomplex!(atom!("test_functor"), y());
    let uni1 = unify!(x(), symptom);
    let func = pred!("functor", x(), atom!("symptom"));
    let uni2 = unify!(y(), atom!("Success #1"));
    let body = and_goal!(uni1, func, uni2);
    let rule4 = make_rule(head, body);

    let symptom = parse_complex("symptom(cold, sneezing)").unwrap();
    let head = scomplex!(atom!("test_functor"), y());
    let uni1 = unify!(x(), symptom);
    let func = pred!("functor", x(), atom!("symptoms"));
    let not_op = Goal::OperatorGoal(Operator::Not(vec![func]));
    let uni2 = unify!(y(), atom!("Success #2"));
    let body = and_goal!(uni1, not_op, uni2);
    let rule5 = make_rule(head, body);

    let symptom = parse_complex("symptom(cold, sneezing)").unwrap();
    let head = scomplex!(atom!("test_functor"), y());
    let uni1 = unify!(x(), symptom);
    let func = pred!("functor", x(), atom!("symp*"));
    let uni2 = unify!(y(), atom!("Success #3"));
    let body = and_goal!(uni1, func, uni2);
    let rule6 = make_rule(head, body);

    add_rules!(&mut kb, rule4, rule5, rule6);

    let query = query!(atom!("test_functor"), x());
    let sn = make_base_node(query, &kb);
    let solutions = solve_all(sn);

    assert_eq!("$X = Success #1", solutions[0]);
    assert_eq!("$X = Success #2", solutions[1]);
    assert_eq!("$X = Success #3", solutions[2]);

} // test_functor()
