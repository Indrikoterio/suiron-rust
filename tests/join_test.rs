// Test the join() function.
//
// The join function is used to join terms (strings and numbers),
// to make a single term. It is used to join words and punctuation
// to form a sentence.
//
// Words are separated by spaces, but punctuation is joined directly
// to the previous word. For example, the list:
//
// [coffee, \,, tea, or, juice]
//
// will become:  coffee, tea or juice
//
// Note that the comma in the list needs to be escaped by a backslash.
//
// Cleve Lendon  2023

use suiron::*;
use std::rc::Rc;

#[test]
pub fn test_join() {

    // Rule:
    // would_you_like($Out) :- $D1 = coffee, $D2 = \, , $D3 = tea,
    //                         $D4 = or, $D5 = juice,
    //                         $Out = join($D1, $D2, $D3, $D4, $D5).
    // Query:
    // ?- would_you_like($X).

    let mut kb = KnowledgeBase::new();

    fn d1()  -> Unifiable { logic_var!("$D1") }
    fn d2()  -> Unifiable { logic_var!("$D2") }
    fn d3()  -> Unifiable { logic_var!("$D3") }
    fn d4()  -> Unifiable { logic_var!("$D4") }
    fn d5()  -> Unifiable { logic_var!("$D5") }
    fn out() -> Unifiable { logic_var!("$Out") }

    let coffee = atom!("coffee");
    let comma  = atom!(",");
    let tea    = atom!("tea");
    let or     = atom!("or");
    let juice  = atom!("juice");

    let uni1 = unify!(d1(), coffee);
    let uni2 = unify!(d2(), comma);
    let uni3 = unify!(d3(), tea);
    let uni4 = unify!(d4(), or);
    let uni5 = unify!(d5(), juice);

    let join_function = sfunction!("join", d1(), d2(), d3(), d4(), d5());
    let uni6 = unify!(out(), join_function);

    let body = and_goal!(uni1, uni2, uni3, uni4, uni5, uni6);
    let head = scomplex!(atom!("would_you_like"), out());
    let rule = make_rule(head, body);
    add_rules!(&mut kb, rule);

    let query = query!(atom!("would_you_like"), logic_var!("$X"));
    let sn = make_base_node(Rc::clone(&query), &kb);

    let solution = solve(sn);
    let expected = "\"$X = coffee, tea or juice\"";
    let actual   = format!("{:?}", solution);
    assert_eq!(expected, actual);

    // New test.
    // Let the parser create a join function.

    let mut kb = KnowledgeBase::new();
    match parse_rule("would_you_like($Out) :- $D1 = coffee, $D2 = \\,, \
                                      $D3 = tea, $D4 = or, $D5 = juice, \
                                      $Out = join($D1, $D2, $D3, $D4, $D5).") {
        Ok(rule) => {
            add_rules!(&mut kb, rule);
            let query = query!(atom!("would_you_like"), logic_var!("$X"));
            let sn = make_base_node(query, &kb);
            let actual = solve(sn);
            let expected = "$X = coffee, tea or juice";
            assert_eq!(expected, actual);
        },
        Err(err) => { panic!("Should be no parsing errors. {}", err); },
    }

} // test_join()
