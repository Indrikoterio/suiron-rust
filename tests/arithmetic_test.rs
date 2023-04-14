// Test built-in arithmetic predicates: Add, Subtract, Multiply, Divide.
//
// f(x, y) = ((x + y) - 6) * 3.4 / 3.4
//
// f(3, 7)  = 4
// f(3, -7) = -10
//
// The rule is:
//
// calculate($X, $Y, $Out) :- $A = add($X, $Y), $B = subtract($A, 6),
//                            $C = multiply($B, 3.4), $Out = divide($C, 3.4).
//
// Cleve Lendon  2023

use std::rc::Rc;

use suiron::*;

#[test]
pub fn test_arithmetic() {

    let mut kb = KnowledgeBase::new();

    fn x()   -> Unifiable { logic_var!("$X") }
    fn y()   -> Unifiable { logic_var!("$Y") }
    fn a()   -> Unifiable { logic_var!("$A") }
    fn b()   -> Unifiable { logic_var!("$B") }
    fn c()   -> Unifiable { logic_var!("$C") }
    fn out() -> Unifiable { logic_var!("$Out") }

    let add1      = sfunction!("add", x(), y());
    let subtract1 = sfunction!("subtract", a(), SInteger(6));
    let multiply1 = sfunction!("multiply", b(), SFloat(3.4));
    let divide1   = sfunction!("divide", c(), SFloat(3.4));

    let uni1 = unify!(a(), add1);
    let uni2 = unify!(b(), subtract1);
    let uni3 = unify!(c(), multiply1);
    let uni4 = unify!(out(), divide1);

    let body = operator_and!(uni1, uni2, uni3, uni4);
    let head = scomplex!(atom!("calculate"), x(), y(), out());

    let rule = make_rule(head, Goal::OperatorGoal(body));
    add_rules!(&mut kb, rule);

    // new rule
    let c1 = scomplex!(atom!("calculate"), SFloat(3.0), SFloat(7.0), out());
    let c2 = scomplex!(atom!("calculate"), SFloat(3.0), SFloat(-7.0), out());
    let goal1 = Goal::ComplexGoal(c1);
    let goal2 = Goal::ComplexGoal(c2);
    let body = operator_or!(goal1, goal2);

    let head = scomplex!(atom!("test"), out());
    let rule = make_rule(head, Goal::OperatorGoal(body));
    add_rules!(&mut kb, rule);

    let query = query!(atom!("test"), x());
    let sn = make_base_node(Rc::clone(&query), &kb);

    let ss = next_solution(Rc::clone(&sn));

    let ss = match ss {
        Some(ss) => { ss },
        None => { panic!("No Solution. #1"); },
    };

    let result = query.get_ground_term(1, Rc::clone(&ss));
    match result {
        Some(r) => { assert_eq!(SFloat(4.0), r); },
        None => { panic!("No solution!"); },
    }

    // Let's get a second solution.
    let ss = next_solution(Rc::clone(&sn));
    let ss = match ss {
        Some(ss) => { ss },
        None => { panic!("No Solution. #1"); },
    };

    let result = query.get_ground_term(1, Rc::clone(&ss));
    match result {
        Some(r) => { assert_eq!(SFloat(-10.0), r); },
        None => { panic!("No solution!"); },
    }

} // test_arithmetic()

#[test]

// Test to see if parsing infixes works.
// This test uses the same formula as above.
pub fn test_arithmetic_parse() {

    let mut kb = KnowledgeBase::new();

    let rule1 = parse_rule(
                "calculate($X, $Y, $Out) :- $A = $X + $Y, $B = $A - 6, \
                 $C = $B * 3.4, $Out = $C / 3.4.").unwrap();

    let rule2 = parse_rule(
                "test($Out) :- calculate(3.0, 7.0, $Out); \
                 calculate(3.0, -7.0, $Out).").unwrap();
    add_rules!(&mut kb, rule1, rule2);

    let query = query!(atom!("test"), logic_var!("$X"));
    let sn = make_base_node(Rc::clone(&query), &kb);

    let ss = next_solution(Rc::clone(&sn));

    let ss = match ss {
        Some(ss) => { ss },
        None => { panic!("No Solution. #1"); },
    };

    let result = query.get_ground_term(1, Rc::clone(&ss));
    match result {
        Some(r) => { assert_eq!(SFloat(4.0), r); },
        None => { panic!("No solution!"); },
    }

    // Let's get a second solution.
    let ss = next_solution(Rc::clone(&sn));
    let ss = match ss {
        Some(ss) => { ss },
        None => { panic!("No Solution. #1"); },
    };

    let result = query.get_ground_term(1, Rc::clone(&ss));
    match result {
        Some(r) => { assert_eq!(SFloat(-10.0), r); },
        None => { panic!("No solution!"); },
    }

} // test_arithmetic_parse()
