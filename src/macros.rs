//! Utilities for creating logic variables, complex terms, lists, etc.
//!
//! [add_rules!](../macro.add_rules.html) -
//! Adds facts and rules to a knowledge base.<br>
//! [anon!](../macro.anon.html) - Creates an anonymous variable.<br>
//! [atom!](../macro.atom.html) - Makes an atom from a string slice.<br>
//! [chars_to_string!](../macro.chars_to_string.html) -
//! Converts a vector or array of chars to a String.<br>
//! [cons_node!](../macro.cons_node.html) -
//! Constructs one node of a singly linked list.<br>
//! [empty_ss](../macro.empty_ss.html) -
//! Creates an empty substitution set, with an Rc-pointer.<br>
//! [logic_var!](../macro.logic_var.html) -
//! Creates a logic variable from a string slice and an optional ID.<br>
//! [operator_and!](../macro.operator_and.html) -
//! Creates an And operator from a list of goals.<br>
//! [operator_or!](../macro.operator_or.html) -
//! Creates an Or operator from a list of terms.<br>
//! [pred!](../macro.pred.html)
//! Creates a built-in predicate.<br>
//! [query!](../macro.query.html) -
//! Creates a query from a list of terms.<br>
//! [rc_cell!](../macro.rc_cell.html) -
//! Creates a smart pointer to mutable data.<br>
//! [scomplex!](../macro.scomplex.html) -
//! Creates a complex term (= compound term).<br>
//! [sfunction!](../macro.sfunction.html) -
//! Creates a built-in function.<br>
//! [slist!](../macro.slist.html) - Builds a Suiron list.<br>
//! [str_to_chars!](../macro.str_to_chars.html) -
//! Converts a string slice to a vector of characters.<br>
//! [unify](../macro.unify.html)
//! Creates a Unify goal (=).

/// Constructs one node of a singly linked list.
///
/// # Notes
/// * This macro does no validation.
/// * Ordinarily, use
/// [slist!](../suiron/macro.slist.html) to make a Suiron list.
/// # Arguments
/// * `term`  - [Unifiable](../suiron/unifiable/enum.Unifiable.html)
/// * `next`  - must be an
/// [SLinkedList](../suiron/unifiable/enum.Unifiable.html#variant.SLinkedList) or
/// [Nil](../suiron/unifiable/enum.Unifiable.html#variant.Nil)
/// * `count` - number of nodes in the linked list
/// * `tail_var`  - boolean, true indicates that the term is a tail variable
/// # Return
/// * `node` -
/// [SLinkedList](../suiron/unifiable/enum.Unifiable.html#variant.SLinkedList)
/// # Usage
/// * To build: [a | $X]
///
/// ```
/// use suiron::*;
///
/// let x = logic_var!("$X");
/// let a = atom!("a");
/// let node1 = cons_node!(x, Nil, 1, true);  // tail variable
/// let node2 = cons_node!(a, node1, 2, false);
/// println!("{}", node2);  // Prints: [a | $X]
/// ```
#[macro_export]
macro_rules! cons_node {
    ($term:expr, $next:expr, $count:expr, $tail_var:expr) => {
        Unifiable::SLinkedList{term: Box::new($term), next: Box::new($next),
                               count: $count, tail_var: $tail_var}
    };
}

/// Builds a Suiron list.
///
/// Suiron lists are defined and used the same way as Prolog lists. Examples:<br>
/// <blockquote>
/// [a, b, c]<br>
/// [a, b, c | $Tail]<br>
/// [$Head | $Tail]
/// </blockquote>
/// Internally, these lists are implemented as singly linked lists.
///
/// # Arguments
/// * `vbar`  - boolean, true if there is a vertical bar (pipe): [a, b, c | $X]
/// * `terms` - list of [Unifiable](../suiron/unifiable/enum.Unifiable.html) terms
/// # Return
/// * list -
/// [SLinkedList](../suiron/unifiable/enum.Unifiable.html#variant.SLinkedList)
/// # Usage
/// * To build: [a, b, c]
///
/// ```
/// use suiron::*;
///
/// let a = atom!("a");
/// let b = atom!("b");
/// let c = atom!("c");
/// let list1 = slist!(false, a, b, c);
/// ```
///
/// * To build: [a, b, c | $X]
///
/// ```
/// use suiron::*;
///
/// let a = atom!("a");
/// let b = atom!("b");
/// let c = atom!("c");
/// let x = logic_var!(next_id(), "$X");
/// let list2 = slist!(true, a, b, c, x);
/// ```
///
/// * To build: []
///
/// ```
/// use suiron::*;
///
/// let list3 = slist!();
/// ```
#[macro_export]
macro_rules! slist {
    () => ( cons_node!(Unifiable::Nil, Unifiable::Nil, 0, false) ); // empty list
    ($vbar:expr, $($terms:expr),*) => (
        make_linked_list($vbar, vec!($($terms),*))
    );
}

/// Adds facts and rules to a knowledge base.
///
/// # Arguments
/// * [knowledge base](../suiron/knowledge_base/index.html)
/// * list of [facts and rules](../suiron/rule/index.html)
///
/// # Usage
///
/// ```
/// use suiron::*;
///
/// let rule1 = parse_rule("parent($X, $Y) :- mother($X, $Y).").unwrap();
/// let rule2 = parse_rule("parent($X, $Y) :- father($X, $Y).").unwrap();
/// let fact1 = parse_rule("music(The Cure, Just Like Heaven).").unwrap();
/// let fact2 = parse_rule("music(China Crisis, Black Man Ray).").unwrap();
///
/// let mut kb = KnowledgeBase::new();
/// add_rules!(&mut kb, rule1, rule2, fact1, fact2);
/// print_kb(&kb);
/// ```
///
/// Should print:
/// <pre>
/// ########## Contents of Knowledge Base ##########
/// music/2
/// 	music(The Cure, Just Like Heaven).
/// 	music(China Crisis, Black Man Ray).
/// parent/2
/// 	parent($X, $Y) :- mother($X, $Y).
/// 	parent($X, $Y) :- father($X, $Y).
/// </pre>
///
/// # Note
/// This macro calls the function
/// [add_rules()](../suiron/knowledge_base/fn.add_rules.html).
#[macro_export]
macro_rules! add_rules {
    ($kb:expr, $($rules:expr),*) => (
        add_rules($kb, vec!($($rules),*))
    );
}

/// Makes an atom from a string slice.
///
/// [Atom](../suiron/unifiable/enum.Unifiable.html#variant.Atom)s are
/// string constants. Unlike Prolog, Suiron's atoms can be capitalized
/// or lower case.<br>
/// For example, in the complex term 'father(Anakin, Luke)',
/// the terms 'father', 'Anakin', and 'Luke' are all atoms.
///
/// # Usage
/// ```
/// use suiron::*;
///
/// let functor = atom!("father");
/// let term1 = atom!("Anakin");
/// let term2 = atom!("Luke");
/// ```
///
#[macro_export]
macro_rules! atom {
    ($the_str:expr) => {
        Unifiable::Atom($the_str.to_string())
    };
}

/// Creates a logic variable from a string slice and an optional ID.
///
/// Suiron variables are similar to Prolog variables, except that the
/// names begin with a dollar sign.<br>
/// For example:
/// <blockquote>
/// [1, 2, 3, 4] = [$Head | $Tail]
/// </blockquote>
///
/// # Notes
/// * Logic variable names should begin with a dollar sign followed by
/// a letter (eg. $Age), but this macro does not check.
/// * If the ID argument is missing, it is set to 0 by default.
/// # Arguments
/// * `id` - positive integer
/// * `name` - &str
/// # Return
/// * [LogicVar](../suiron/unifiable/enum.Unifiable.html#variant.LogicVar)
/// # Usage
/// ```
/// use suiron::*;
///
/// let x = logic_var!(next_id(), "$X");
/// let y = logic_var!("$Y"); // ID is 0
/// ```
#[macro_export]
macro_rules! logic_var {
    ($name:expr) => {
        Unifiable::LogicVar{ id: 0, name: $name.to_string() }
    };
    ($id:expr, $name:expr) => {
        Unifiable::LogicVar{ id: $id, name: $name.to_string() }
    };
}

/// Creates an anonymous variable.
///
/// [Anonymous](../suiron/unifiable/enum.Unifiable.html#variant.Anonymous)
/// is a unifiable term which unifies with any other term.<br>
/// Suiron source code represents the anonymous variable as $_ . Eg.:
/// <blockquote>
/// check_noun_verb($_, $_, $_, past) :- !.
/// </blockquote>
///
/// # Usage
/// ```
/// use suiron::*;
///
/// let dont_care = anon!();
/// ```
#[macro_export]
macro_rules! anon {
    () => { Unifiable::Anonymous };
}

/// Creates a complex term (= compound term).
///
/// As in Prolog, complex terms consist of a functor, followed by
/// a sequence of arguments (terms) enclosed in parentheses.
/// For example:
/// <blockquote>
/// animal(horse, mammal, herbivore)
/// </blockquote>
///
/// See also: [make_complex()](../suiron/s_complex/fn.make_complex.html).
///
/// # Arguments
/// * list of [Unifiable](../suiron/unifiable/enum.Unifiable.html) terms<br>
/// # Return
/// * [SComplex](../suiron/unifiable/enum.Unifiable.html#variant.SComplex)
/// # Usage
/// * To build: pronoun(I, subject, first, singular)
///
/// ```
/// use suiron::*;
///
/// let functor = atom!("pronoun");
/// let word = atom!("I");
/// let case = atom!("subject");
/// let person = atom!("first");
/// let sing_plur = atom!("singular");
/// let pronoun = scomplex!(functor, word, case, person, sing_plur);
/// ```
/// # Note
/// * The first term must be an
///   [Atom](../suiron/unifiable/enum.Unifiable.html#variant.Atom),
///   but this macro does not check.
#[macro_export]
macro_rules! scomplex {
    ($($term:expr),*) => (
        Unifiable::SComplex(vec!($($term),*))
    );
} // scomplex!

/// Creates an And operator.
///
/// The [And](../suiron/operator/enum.Operator.html#variant.And)
/// operator is represented in Suiron source code by a comma separated
/// list of goals.<br>For example, the following source code:<br>
///
/// <blockquote>
/// parent($X, $Z), parent($Z, $Y), male($X)
/// </blockquote>
///
/// represents 'parent And parent And male':
///
/// # Arguments
/// * list of [Goals](../suiron/goal/enum.Goal.html)
/// # Return
/// * [And](../suiron/operator/enum.Operator.html#variant.And) operator
/// # Usage
/// ```
/// use suiron::*;
///
/// // parent($Grandfather, $P), parent($P, $Child), male($Grandfather)
/// let g1 = parse_subgoal("parent($Grandfather, $P)").unwrap();
/// let g2 = parse_subgoal("parent($P, $Child)").unwrap();
/// let g3 = parse_subgoal("male($Grandfather)").unwrap();
/// let the_and = operator_and!(g1, g2, g3);
/// ```
#[macro_export]
macro_rules! operator_and {
    ($($goal:expr),*) => (
        Operator::And(vec!($($goal),*))
    );
} // operator_and!

/// Creates an Or operator.
///
/// The [Or](../suiron/operator/enum.Operator.html#variant.Or)
/// operator is represented in Suiron source by a list of goals
/// separated by semicolons.<br>For example, the following source code:<br>
///
/// <blockquote>
/// mother($P, $C); father($P, $C)
/// </blockquote>
///
/// means 'mother Or father'.
///
/// # Arguments
/// * list of [Goals](../suiron/goal/enum.Goal.html)
/// # Return
/// * [Or](../suiron/operator/enum.Operator.html#variant.Or) operator
/// # Usage
/// ```
/// use suiron::*;
///
/// // mother($X, $Y); father($X, $Y)
/// let g1 = parse_subgoal("mother($X, $Y)").unwrap();
/// let g2 = parse_subgoal("father($X, $Y)").unwrap();
/// let the_or = operator_or!(g1, g2);
/// ```
#[macro_export]
macro_rules! operator_or {
    ($($goal:expr),*) => (
        Operator::Or(vec!($($goal),*))
    );
} // operator_or!

/// Converts a string slice to a vector of characters.
///
/// # Usage
/// ```
/// use suiron::*;
///
/// let city = str_to_chars!("渋谷");
/// let n = city.len();  // n == 2
/// ```
#[macro_export]
macro_rules! str_to_chars {
    ($st:expr) => { $st.chars().collect::<Vec<char>>() };
}

/// Converts a vector or array of chars to a String.
///
/// # Usage
/// ```
/// use suiron::*;
///
/// let city_of_light = chars_to_string!(['P', 'a', 'r', 'i', 's']);
/// ```
#[macro_export]
macro_rules! chars_to_string {
    ($chrs:expr) => { $chrs.iter().collect::<String>() };
}

/// Creates an empty
/// [substitution set](../suiron/substitution_set/index.html),
/// pointed to by an Rc-pointer.
///
/// # Return
/// * Rc&lt;[SubstitutionSet](../suiron/substitution_set/type.SubstitutionSet.html)&gt;
/// # Usage
/// ```
/// use std::rc::Rc;
/// use suiron::*;
///
/// let ss = empty_ss!();
/// ```
#[macro_export]
macro_rules! empty_ss {
    () => { Rc::new(SubstitutionSet::new()) };
}

/// Creates a smart pointer to mutable data.
///
/// rc_cell!(data) is equivalent to Rc::new(RefCell::new(data)).
///
/// # Note
/// Rc and RefCell must be imported, as shown below.
///
/// # Usage
/// ```
/// use std::rc::Rc;
/// use std::cell::RefCell;
/// use suiron::*;
///
/// let data = atom!("Fawlty Towers");
/// let r = rc_cell!(data);
/// println!("{:?}", r.borrow());  // Prints: Atom("Fawlty Towers") }
/// ```
#[macro_export]
macro_rules! rc_cell {
    ($data:expr) => { Rc::new(RefCell::new($data)) };
}

/// Creates a query from a list of terms.
///
/// A query is a [Goal](../suiron/goal/index.html), a logic expression to be solved.
/// It has the same form as a
/// [complex term](../suiron/unifiable/enum.Unifiable.html#variant.SComplex),
/// consisting of a functor with arguments (terms) enclosed in parentheses:
/// `functor(term1, term2, term3)`.
///
/// The functor must be an
/// [atom](../suiron/unifiable/enum.Unifiable.html#variant.Atom),
/// but a term can be any [unifiable term](../suiron/unifiable/enum.Unifiable.html).
///
/// This utility calls [make_query()](../suiron/s_complex/fn.make_query.html),
/// which [clears](../suiron/logic_var/fn.clear_id.html) the logic variable ID,
/// and [recreates](../suiron/unifiable/enum.Unifiable.html#method.recreate_variables)
/// all logic variables within the query.
///
/// The macro returns an Rc pointer to the newly created query, which can be
/// passed to the function [make_base_node()](../suiron/goal/fn.make_base_node.html).
///
/// See also: [parse_query()](../suiron/s_complex/fn.parse_query.html)
///
/// # Usage
/// ```
/// use std::rc::Rc;
/// use suiron::*;
///
/// let kb = test_kb();
/// let functor = atom!("loves");
/// let who = logic_var!("$Who");
/// let penny = atom!("Penny");
///
/// // Query is: loves($Who, Penny).
/// let query = query!(functor, who, penny);
/// let base_node = make_base_node(Rc::clone(&query), &kb);
/// println!("{}", solve(base_node));   // Prints: $Who = Leonard
/// ```
#[macro_export]
macro_rules! query {
    ($($term:expr),*) => (
        Rc::new(make_query((vec!($($term),*))))
    );
}

/// Creates a Unify goal.
///
/// Calling `unify!($X, 7)` in Rust is equivalent to `$X = 7` in Suiron source code.
///
/// # Usage
/// ```
/// use suiron::*;
///
/// let x = logic_var!(next_id(), "$X");
/// let number = SInteger(7);
/// let goal = unify!(x, number);   // Goal is: $X = 7
/// ```
#[macro_export]
macro_rules! unify {
    ($left:expr, $right:expr) => {
        Goal::BuiltInGoal(
            BuiltInPredicate::new("unify".to_string(), Some(vec![$left, $right]))
        )
    };
}

/// Creates a built-in function.
///
/// # Arguments
/// * name of function
/// * list of [Unifiable](../suiron/unifiable/enum.Unifiable.html) terms
/// # Return
/// * [SFunction](../suiron/unifiable/enum.Unifiable.html#variant.SFunction)
///
/// # Usage
/// ```
/// use suiron::*;
///
/// // Define: add(7.0, 3.0)
/// let add_func = sfunction!("add", SFloat(7.0), SFloat(3.0));
/// ```
#[macro_export]
macro_rules! sfunction {
    ($name:expr, $($term:expr),*) => {
        Unifiable::SFunction{ name: $name.to_string(), terms: vec!($($term),*) }
    };
}

/// Creates a built-in predicate.
///
/// # Arguments
/// * functor
/// * list of [Unifiable](../suiron/unifiable/enum.Unifiable.html) terms (Optional)
/// # Return
/// * [BuiltInPredicate](../suiron/built_in_predicates/struct.BuiltInPredicate.html)
///
/// # Usage
/// ```
/// use suiron::*;
///
/// // Make a fail predicate. (No terms.)
/// let fail_pred = pred!("fail");
///
/// // Make a unify predicate.
/// let x = logic_var!("$X");
/// let n = SFloat(3.0);
/// let unify_pred = pred!("unify", x, n);
/// println!("{}", unify_pred);  // Prints: $X = 3
/// ```
#[macro_export]
macro_rules! pred {
    ($functor:expr) => {  // For predicates without terms.
        BuiltInPredicate{ functor: $functor.to_string(), terms: None }
    };
    ($functor:expr, $($term:expr),*) => {
        BuiltInPredicate{ functor: $functor.to_string(), terms: Some(vec!($($term),*)) }
    };
}
