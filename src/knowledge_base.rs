//! Defines a dictionary of predicates (facts and rules).
//!
//! The dictionary is implemented as a HashMap. Each entry is a list
//! (vector)<br> of facts and/or rules, indexed by predicate name.
//!
//! A predicate's name consists of its functor and its arity, separated
//! by a slash.
//!
//! For example, for the rule,
//! <blockquote>
//! grandfather($X, $Y) :- father($X, $Z), father($Z, $Y).
//! </blockquote>
//!
//! the predicate name is: `grandfather/2`.
//!
// Cleve Lendon 2023

use std::collections::HashMap;

use crate::*;

use super::rule::*;
use super::goal::*;
use super::time_out::*;
use super::unifiable::Unifiable;
use super::logic_var::*;

pub type KnowledgeBase = HashMap<String, Vec<Rule>>;

/// Makes a rule.
///
/// Rules consist of a head term and a body goal: head :- body.
///
/// See also: [parse_rule()](../rule/fn.parse_rule.html)
///
/// # Note
/// The head of a rule must be Unifiable::SComplex, but this
/// function does not check.
/// # Usage
/// ```
/// use suiron::*;
///
/// let cmplx = parse_complex("father($X, $Y)").unwrap();
/// let and_goal = parse_subgoal("parent($X, $Y), male($X)").unwrap();
/// let rule = make_rule(cmplx, and_goal);
/// println!("{}", rule);   // Prints: father($X, $Y) :- parent($X, $Y), male($X).
/// ```
pub fn make_rule(head: Unifiable, body: Goal) -> Rule {
    Rule{ head, body }
}

/// Makes a fact.
///
/// Facts and Rules use the same structure, [Rule](../rule/struct.Rule.html).
/// For facts, the body is Goal::Nil.
///
/// See also: [parse_rule()](../rule/fn.parse_rule.html)
///
/// # Note
/// The head term of a fact must be Unifiable::SComplex, but
/// this function does not check.
/// # Usage
/// ```
/// use suiron::*;
///
/// let cmplx = parse_complex("music(Moby, Whispering Wind)").unwrap();
/// let fact = make_fact(cmplx);
/// println!("{}", fact);   // Prints: music(Moby, Whispering Wind).
/// ```
pub fn make_fact(head: Unifiable) -> Rule {
    Rule{ head, body: Goal::Nil }
}

/// Adds facts and rules to a knowledge base.
///
/// # Arguments
/// * `kb` - Knowledge Base
/// * `rules` - vector of [Rules](../rule/index.html)
///
/// # Usage
/// ```
/// use suiron::*;
///
/// let rule1 = parse_rule("test($X) :- test_1($X).").unwrap();
/// let rule2 = parse_rule("test($X) :- test_2($X).").unwrap();
/// let fact1 = parse_rule("vehicle(car).").unwrap();
/// let mut kb = KnowledgeBase::new();
/// add_rules(&mut kb, vec![rule1, rule2, fact1]);
/// ```
///
/// # Note
/// The macro [add_rules!](../macro.add_rules.html) may be more convenient.
pub fn add_rules(kb: &mut KnowledgeBase, rules: Vec<Rule>) {
    for rule in rules {
        let key = rule.key();
        match kb.get_mut(&key) {
            Some(rules_from_kb) => { rules_from_kb.push(rule); },
            None => {
                let new_rules = vec![rule];
                kb.insert(key, new_rules);
            },
        } // match
    } // for
} // add_rules()

/// Counts the number of facts and rules for the given predicate.
///
/// When the maximum execution time has been exceeded, this function returns 0.<br>
/// This allows the inference engine to back out of the search for a solution.
///
/// # Arguments
/// * `kb` - Knowledge Base
/// * `predicate_name` - eg. \"loves/2\"
/// # Return
/// * number of facts/rules
/// # Usage
/// ```
/// use suiron::*;
///
/// let kb = test_kb();
/// let n = count_rules(&kb, "loves/2"); // n == 2
/// ```
pub fn count_rules(kb: &KnowledgeBase, predicate_name: &str) -> usize {

    if query_stopped() { return 0; }

    match kb.get(predicate_name) {
        Some(list) => { return list.len(); },
        None => { return 0; }
    }

} // count_rules()

/// Fetches a rule (or fact) from the knowledge base.
///
/// Rules are indexed by predicate name (eg. `loves/2`) and by index number.
///
/// This function calls
/// [recreate_variables()](../rule/struct.Rule.html#method.recreate_variables)
/// to make the variables unique.
///
/// # Arguments
/// * `kb` - Knowledge Base
/// * `predicate_name` - eg. \"loves/2\"
/// * `index`
/// # Return
/// * [Rule](../rule/index.html)
/// # Panics
/// * When the required fact/rule does not exist.
/// * When the index is out of range.
/// # Usage
/// ```
/// use suiron::*;
///
/// let kb = test_kb();
/// let fact = get_rule(&kb, "loves/2", 1);
/// println!("{}", fact); // Prints: loves(Penny, Leonard).
/// ```
pub fn get_rule(kb: &KnowledgeBase, predicate_name: &str, index: usize) -> Rule {

    match kb.get(predicate_name) {
        Some(rules) => {
            if index >= rules.len() {  // Should never happen.
                panic!("get_rule() - Index out of range: {}", index);
            }
            let fact_or_rule = rules[index].clone();  // Can't fail.
            return fact_or_rule.recreate_variables(&mut VarMap::new());
        },
        None => {
            panic!("get_rule() - Rule does not exist: {}", predicate_name);
        },
    } // match

} // get_rule()

/// Formats the knowledge base for display. Use for debugging.
///
/// See also [print_kb()](../knowledge_base/fn.print_kb.html).
///
/// # Note
/// * The keys (predicate names) are sorted.
/// * KnowledgeBase is a type alias. Because of this, it is not
///   possible to implement the Display trait.
/// # Arguments
/// * `kb` - Knowledge Base
/// # Return
/// * `String`
/// # Usage
/// ```
/// use suiron::*;
///
/// let kb = test_kb();
/// let s = format_kb(&kb);
/// println!("{}", s);
/// ```
/// The above will print:
/// <pre>
/// _____ Contents of Knowledge Base _____
/// father/2
/// 	father(Alfred, Edward).
/// 	father(Edward, Aethelstan).
/// grandfather/2
/// 	grandfather($X, $Y) :- father($X, $Z), father($Z, $Y).
/// 	grandfather($X, $Y) :- father($X, $Z), mother($Z, $Y).
/// loves/2
/// 	loves(Leonard, Penny).
/// 	loves(Penny, Leonard).
/// ______________________________________
/// </pre>
pub fn format_kb(kb: &KnowledgeBase) -> String {

    // header
    let mut out = "_____ Contents of Knowledge Base _____\n".to_string();

    // Sort the keys.
    let mut keys: Vec<String> = vec![];
    for (k, _) in kb.iter() { keys.push(k.clone()); }
    keys.sort();

    // Format each entry.
    for key in keys {
        out += &format!("{}\n", key);
        let rules = kb.get(&key).unwrap();
        for rule in rules { out += &format!("\t{}\n", rule); }
    }

    out += "______________________________________";
    return out;

} // format_kb()

/// Prints a formatted knowledge base. Use for debugging.
///
/// This function calls [format_kb()](../knowledge_base/fn.format_kb.html)
/// and prints out the knowledge base.
///
/// # Arguments
/// * `kb` - knowledge base
/// # Usage
/// ```
/// use suiron::*;
///
/// let kb = test_kb();
/// print_kb(&kb);
/// ```
///
/// The above will print:
/// <pre>
/// _____ Contents of Knowledge Base _____
/// father/2
/// 	father(Alfred, Edward).
/// 	father(Edward, Aethelstan).
/// grandfather/2
/// 	grandfather($X, $Y) :- father($X, $Z), father($Z, $Y).
/// 	grandfather($X, $Y) :- father($X, $Z), mother($Z, $Y).
/// loves/2
/// 	loves(Leonard, Penny).
/// 	loves(Penny, Leonard).
/// ______________________________________
/// </pre>
pub fn print_kb(kb: &KnowledgeBase) {
    println!("{}", format_kb(kb));
} // print_kb()


/// Creates a knowledge base with a few facts and rules for testing.
///
/// <blockquote>
/// father(Alfred, Edward).<br>
/// father(Edward, Aethelstan).<br>
/// grandfather($X, $Y) :- father($X, $Z), father($Z, $Y).<br>
/// grandfather($X, $Y) :- father($X, $Z), mother($Z, $Y).<br>
/// loves(Penny, Leonard).<br>
/// loves(Leonard, Penny).<br>
/// </blockquote>
///
/// # Usage
/// ```
/// use suiron::*;
///
/// let kb = test_kb();
/// ```
pub fn test_kb() -> KnowledgeBase {

    fn loves()   -> Unifiable { atom!("loves") }
    fn penny()   -> Unifiable { atom!("Penny") }
    fn leonard() -> Unifiable { atom!("Leonard") }

    fn grandfather() -> Unifiable { atom!("grandfather") }
    fn father()      -> Unifiable { atom!("father") }
    fn mother()      -> Unifiable { atom!("mother") }

    fn alfred()      -> Unifiable { atom!("Alfred") }
    fn edward()      -> Unifiable { atom!("Edward") }
    fn aethelstan()  -> Unifiable { atom!("Aethelstan") }

    let cmplx1 = scomplex!(loves(), leonard(), penny());
    let fact1 =  Rule{head: cmplx1, body: Goal::Nil};

    let cmplx2 = scomplex!(loves(), penny(), leonard());
    let fact2 =  Rule{head: cmplx2, body: Goal::Nil};

    let cmplx3 = scomplex!(father(), alfred(), edward());
    let fact3 =  Rule{head: cmplx3, body: Goal::Nil};

    let cmplx4 = scomplex!(father(), edward(), aethelstan());
    let fact4 =  Rule{head: cmplx4, body: Goal::Nil};

    fn x() -> Unifiable { logic_var!("$X") }
    fn y() -> Unifiable { logic_var!("$Y") }
    fn z() -> Unifiable { logic_var!("$Z") }

    // grandfather($X, $Y) :- father($X, $Z), father($Z, $Y).
    let cmplx5 = scomplex!(grandfather(), x(), y());
    let cmplx6 = scomplex!(father(), x(), z());
    let cmplx7 = scomplex!(father(), z(), y());
    let goal1  = Goal::ComplexGoal(cmplx6);
    let goal2  = Goal::ComplexGoal(cmplx7);
    let goal3  = and_goal!(goal1, goal2);
    let rule1  =  Rule{head: cmplx5, body: goal3};

    // grandfather($X, $Y) :- father($X, $Z), mother($Z, $Y).
    let cmplx5 = scomplex!(grandfather(), x(), y());
    let cmplx6 = scomplex!(father(), x(), z());
    let cmplx7 = scomplex!(mother(), z(), y());
    let goal1  = Goal::ComplexGoal(cmplx6);
    let goal2  = Goal::ComplexGoal(cmplx7);
    let goal3 = and_goal!(goal1, goal2);
    let rule2  =  Rule{head: cmplx5, body: goal3};

    // Create a knowledge base and add in the facts and rules.
    let mut kb = KnowledgeBase::new();
    add_rules(&mut kb, vec![fact1, fact2, fact3, fact4, rule1, rule2]);

    // Uncomment print_kb() to verify.
    // print_kb(&kb);

    return kb;

} // test_kb


#[cfg(test)]
mod test {

    use serial_test::serial;
    use crate::*;

    // Create logic vars for testing.
    fn x() -> Unifiable { logic_var!("$X") }
    fn y() -> Unifiable { logic_var!("$Y") }
    fn z() -> Unifiable { logic_var!("$Z") }

    // Make a fact: father(Anakin, Luke).
    fn make_fact1() -> Rule {
        let f = atom!("father");
        let a = atom!("Anakin");
        let l = atom!("Luke");
        let cmplx = scomplex!(f, a, l);
        Rule{ head: cmplx, body: Goal::Nil }
    }

    // Make a fact: father(Anakin, Leia).
    fn make_fact2() -> Rule {
        let f = atom!("father");
        let a = atom!("Anakin");
        let l = atom!("Leia");
        let cmplx = scomplex!(f, a, l);
        Rule{ head: cmplx, body: Goal::Nil }
    }

    // grandfather($X, $Y)
    fn c1() -> Unifiable { scomplex!(atom!("grandfather"), x(), y()) }

    // father($X, $Z)
    fn c2() -> Unifiable { scomplex!(atom!("father"), x(), z()) }

    // father($Z, $Y)
    fn c3() -> Unifiable { scomplex!(atom!("father"), z(), y()) }

    // Make a rule: grandfather($X, $Y) :- father($X, $Z), father($Z, $Y).
    fn make_rule() -> Rule {
        let c1 = c1();
        let c2 = c2();
        let c3 = c3();
        let goal1 = Goal::ComplexGoal(c2);
        let goal2 = Goal::ComplexGoal(c3);
        let goal3 = and_goal!(goal1, goal2);  // father($X, $Z), father($Z, $Y)
        // grandfather($X, $Y) :- father($X, $Z), father($Z, $Y).
        Rule{head: c1, body: goal3}
    }

    // Test add_rules(), count_rules() and format_kb().
    #[test]
    #[serial]
    fn test_add_rules() {

        start_query();  // Set SUIRON_STOP_QUERY to false.

        // For testing, create a rule and two facts:
        //   grandfather($X, $Y) :- father($X, $Z), father($Z, $Y).
        //   father(Anakin, Luke).
        //   father(Anakin, Leia).

        let rule1 = make_rule();
        let fact1 = make_fact1();
        let fact2 = make_fact2();

        // Create a knowledge base and add in the rule and fact.
        let mut kb = KnowledgeBase::new();
        add_rules(&mut kb, vec![rule1, fact1, fact2]);

        // Call format_kb() to confirm the knowledge base.
        let s = "_____ Contents of Knowledge Base _____\n\
            father/2\n\
            \tfather(Anakin, Luke).\n\
            \tfather(Anakin, Leia).\n\
            grandfather/2\n\
            \tgrandfather($X, $Y) :- father($X, $Z), father($Z, $Y).\n\
            ______________________________________";

        assert_eq!(s, format_kb(&kb));

        // father($X, $Y)
        let c1 = scomplex!(atom!("father"), x(), y());
        let n = count_rules(&kb, &c1.key());
        assert_eq!(n, 2);

    } // test_add_rules()

    // Test the get_rule() function.
    #[test]
    #[serial]
    fn test_get_rule() {

        clear_id();
        let kb = test_kb();

        // Fetch the grandfather rule, index 0.
        let rule = get_rule(&kb, "grandfather/2", 0);
        let rule_str = format!("{}", rule);
        let s = "grandfather($X_1, $Y_2) :- father($X_1, $Z_3), father($Z_3, $Y_2).";
        assert_eq!(s, rule_str);

        let rule = get_rule(&kb, "loves/2", 1);
        let rule_str = format!("{}", rule);
        let s = "loves(Penny, Leonard).";
        assert_eq!(s, rule_str);

    } // test_get_rule()

    // get_rule() should panic if predicate name is invalid.
    #[test]
    #[serial]
    #[should_panic]
    fn test_get_rule_panic1() {
        let kb = test_kb();
        get_rule(&kb, "luvs/2", 0);
    } // test_get_rule_panic1()

    // get_rule() should panic if index is invalid.
    #[test]
    #[serial]
    #[should_panic]
    fn test_get_rule_panic2() {
        let kb = test_kb();
        get_rule(&kb, "loves/2", 20);
    } // test_get_rule_panic2()

} // test
