// Test the built-in predicates include() and exclude().
//
// The include and exclude() predicates filter a list of terms to produce a new list.
//
// Facts:
// male(Sheldon).
// male(Leonard).
// male(Raj).
// male(Howard).
// female(Penny).
// female(Bernadette).
// female(Amy).
//
// Rules:
// get_people($X) :- $X = [male(Sheldon), male(Leonard), male(Raj),
//                         male(Howard), female(Penny),
//                         female(Bernadette), female(Amy)].
// list_wimmin($W) :- get_people($X), include($X, female($_), $W).
// list_nerds($N)  :- get_people($X), exclude($X, female($_), $N).
//
// Queries:
//   ?- list_wimmin($W).
//   ?- list_nerds($N).
//
// Cleve Lendon  2023

use std::rc::Rc;
use suiron::*;

#[test]
pub fn test_filter() {

    fn list_of_people() -> Unifiable {
        let c1 = scomplex!(atom!("male"), atom!("Sheldon"));
        let c2 = scomplex!(atom!("male"), atom!("Leonard"));
        let c3 = scomplex!(atom!("male"), atom!("Raj"));
        let c4 = scomplex!(atom!("male"), atom!("Howard"));
        let c5 = scomplex!(atom!("female"), atom!("Penny"));
        let c6 = scomplex!(atom!("female"), atom!("Bernadette"));
        let c7 = scomplex!(atom!("female"), atom!("Amy"));
        slist!(false, c1, c2, c3, c4, c5, c6, c7)
    }

    let mut kb = KnowledgeBase::new();

    // get_people($X) :- $X = [male(Sheldon), male(Leonard), ...].
    fn x() -> Unifiable { logic_var!("$X") }
    let get_people = scomplex!(atom!("get_people"), x());
    let uni = unify!(x(), list_of_people());
    let r1 = make_rule(get_people, uni);

    // list_wimmin($W) :- get_people($X), include($X, female($_), $W).
    fn w() -> Unifiable { logic_var!("$W") }
    let head = scomplex!(atom!("list_wimmin"), w());
    let filter = scomplex!(atom!("female"), anon!());
    let get_people = pred!("get_people", x());
    let inc = pred!("include", filter, x(), w());
    let body = and_goal!(get_people, inc);
    let r2 = make_rule(head, body);

    // list_nerds($N) :- get_people($X), exclude($X, female($_), $N).
    fn n() -> Unifiable { logic_var!("$N") }
    let head = scomplex!(atom!("list_nerds"), n());
    let filter = scomplex!(atom!("female"), anon!());
    let get_people = pred!("get_people", x());
    let ex  = pred!("exclude", filter, x(), n());
    let body = and_goal!(get_people, ex);
    let r3 = make_rule(head, body);

    add_rules!(&mut kb, r1, r2, r3);
    //print_kb(&kb);

    // ?- list_wimmin($W).
    let query = query!(atom!("list_wimmin"), w());
    let sn = make_base_node(query, &kb);
    let result = solve(sn);
    let expected = "$W = [female(Penny), female(Bernadette), female(Amy)]";
    assert_eq!(expected, result);

    // ?- list_nerds($N).
    let query = query!(atom!("list_nerds"), n());
    let sn = make_base_node(query, &kb);
    let result = solve(sn);
    let expected = "$N = [male(Sheldon), male(Leonard), male(Raj), male(Howard)]";
    assert_eq!(expected, result);

} // test_filter()
