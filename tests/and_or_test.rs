// Tests the And and Or operators of the inference engine.
// Cleve Lendon  2023

use suiron::*;
use std::rc::Rc;

#[test]
pub fn test_and_or() {

    let mut kb = KnowledgeBase::new();

    // Relationships
    let c1 = parse_complex("father(George, Frank)").unwrap();
    let c2 = parse_complex("father(George, Sam)").unwrap();
    let c3 = parse_complex("mother(Gina, Frank)").unwrap();
    let c4 = parse_complex("mother(Gina, Sam)").unwrap();
    let c5 = parse_complex("mother(Maria, Marcus)").unwrap();
    let c6 = parse_complex("father(Frank, Marcus)").unwrap();

    // Make facts.
    let f1 = make_fact(c1);
    let f2 = make_fact(c2);
    let f3 = make_fact(c3);
    let f4 = make_fact(c4);
    let f5 = make_fact(c5);
    let f6 = make_fact(c6);

    // Add facts to the knowledge base.
    add_rules!(&mut kb, f1, f2, f3, f4, f5, f6);

    // parent($X, $Y) :- father($X, $Y); mother($X, $Y).
    let parent = parse_complex("parent($X, $Y)").unwrap();
    let father = parse_subgoal("father($X, $Y)").unwrap();
    let mother = parse_subgoal("mother($X, $Y)").unwrap();
    let or = operator_or!(father, mother);
    let r1 = make_rule(parent, or);

    // relative($X, $Y) :- grandfather($X, $Y); father($X, $Y);
    //                     grandmother($X, $Y); mother($X, $Y).
    let relative = parse_complex("relative($X, $Y)").unwrap();
    let grandfather = parse_subgoal("grandfather($X, $Y)").unwrap();
    let grandmother = parse_subgoal("grandmother($X, $Y)").unwrap();
    let father = parse_subgoal("father($X, $Y)").unwrap();
    let mother = parse_subgoal("mother($X, $Y)").unwrap();
    let or2 = operator_or!(grandfather, father, grandmother, mother);
    let r2 = make_rule(relative, or2);

    // grandfather($X, $Y) :- father($X, $Z), parent($Z, $Y).
    let father = parse_subgoal("father($X, $Z)").unwrap();
    let parent = parse_subgoal("parent($Z, $Y)").unwrap();
    let and = operator_and!(father, parent);
    let grandfather = parse_complex("grandfather($X, $Y)").unwrap();
    let r3 = make_rule(grandfather, and);

    // grandmother() deliberately not defined.

    // Add more rules to the knowledge base.
    add_rules!(&mut kb, r1, r2, r3);

    // Who is related to Marcus?
    let query = parse_query("relative($X, Marcus)").unwrap();
    let q = Rc::new(query);
    let sn = make_base_node(q, &kb);

    let results = solve_all(sn);
    assert_eq!(results, ["$X = George", "$X = Frank", "$X = Maria"]);

}
