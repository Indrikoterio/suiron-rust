// Test the Append predicate.
//
// The append predicate is used to join terms into a list. For example:
//
// $X = raspberry, append(cherry, [strawberry, blueberry], $X, $Out).
//
// The last term of append() is an output term. For the above, $Out
// should unify with: [cherry, strawberry, blueberry, raspberry]
//
// Cleve Lendon  2023

use suiron::*;
use std::rc::Rc;

#[test]
pub fn test_append() {

    // Make some comments.
    let red    = atom!("red");
    let orange = atom!("orange");
    let green  = atom!("green");
    let blue   = atom!("blue");
    let purple = atom!("purple");

    /*
      Suiron rule:
      test_append($Out) :- $X = red, $Y = [green, blue, purple],
                           append($X, orange, $Y, $Out).

      The Prolog equivalent would be:
      test_append(Out) :- X = red, Y = [green, blue, purple],
                          append(X, orange, Y, Out).
     */

    let mut kb = KnowledgeBase::new();

    // A lot of this could be done with parse_
    // functions, but let's do it the hard way.

    fn x()   -> Unifiable { logic_var!("$X") }
    fn y()   -> Unifiable { logic_var!("$Y") }
    fn out() -> Unifiable { logic_var!("$Out") }

    let test_append = atom!("test_append");
    let head = scomplex!(test_append, out());

    let list = slist!(false, green, blue, purple);

    let u1 = unify!(x(), red);
    let u2 = unify!(y(), list);
    let append_pred = BuiltInPredicate::Append(vec![x(), orange, y(), out()]);
    let append_goal = Goal::BuiltInGoal(append_pred);

    let body = operator_and!(u1, u2, append_goal);
    let body = Goal::OperatorGoal(body);

    let rule = make_rule(head, body);
    add_rules!(&mut kb, rule);

    let test_append = atom!("test_append");
    let query = query!(test_append, out());
    let base_node = make_base_node(Rc::clone(&query), &kb);

    let solution = next_solution(Rc::clone(&base_node));
    match solution {
        Some(ss) => {
            // Get the result.
            match query.get_ground_term(1, ss) {
                Some(result) => {
                    let s = format!("{}", result);
                    assert_eq!("[red, orange, green, blue, purple]", s);
                },
                None => { panic!("No solution."); },
            }
        },
        None => { panic!("No solution."); },
    } // match solution


    // A second test.

    let rule = parse_rule("test_append2($Out) :- $X = raspberry, \
               append(cherry, [strawberry, blueberry], $X, $Out).").unwrap();
    add_rules!(&mut kb, rule);

    let query = parse_query("test_append2($Z)").unwrap();
    let query = Rc::new(query);
    let base_node = make_base_node(Rc::clone(&query), &kb);

    let solution = next_solution(Rc::clone(&base_node));
    match solution {
        Some(ss) => {
            // Get the result.
            match query.get_ground_term(1, ss) {
                Some(result) => {
                    let s = format!("{}", result);
                    assert_eq!("[cherry, strawberry, blueberry, raspberry]", s);
                },
                None => { panic!("No solution."); },
            }
        },
        None => { panic!("No solution."); },
    } // match solution

} // test_append()
