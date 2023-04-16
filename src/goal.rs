//! A logic expression to be solved (proven true or false).
//!
//! [Operators](../operator/enum.Operator.html) (And, Or, etc.),
//! [built-in predicates](../built_in_predicates/enum.BuiltInPredicate.html)
//! (Print, Append, etc.) and
//! [complex](../unifiable/enum.Unifiable.html#variant.SComplex)
//! terms are goals.<br>
//! The function [make_solution_node()](../goal/fn.make_solution_node.html)
//! provides a [SolutionNode](../solution_node/struct.SolutionNode.html)
//! appropriate for each type of goal.
//!
// Cleve Lendon 2023

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use crate::*;

use super::logic_var::*;
use super::solution_node::*;
use super::operator::Operator;
use super::unifiable::Unifiable;
use super::knowledge_base::*;
use super::substitution_set::*;
use super::built_in_predicates::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Goal {
    /// Holds an [Operator](../operator/enum.Operator.html),
    /// such as And, Or, Time etc.
    OperatorGoal(Operator),
    /// Holds a built-in predicate, such as print(), append(), etc.
    BuiltInGoal(BuiltInPredicate),
    /// Holds a complex term
    /// ([SComplex](../unifiable/enum.Unifiable.html#variant.SComplex)).
    ComplexGoal(Unifiable),  // Must be variant SComplex.
    /// Variant for [Rule](../rule/index.html)s which don't have a body (ie. facts).
    Nil,
}

impl Goal {

    /// Recreates logic variables to give them unique IDs.
    ///
    /// Logic variables in the knowledge base have an ID of 0, but when
    /// a rule is fetched from the knowledge base, the logic variables
    /// must be given unique IDs.
    ///
    /// # Arguments
    /// * self
    /// * vars - Map of previously recreated variable IDs.
    /// # Return
    /// * new goal
    /// # Usage
    /// ```
    /// use suiron::*;
    ///
    /// clear_id();
    ///
    /// // Create an And goal.
    /// let goal = generate_goal("father($X, $Z), mother($Z, $Y)");
    ///
    /// match goal {
    ///     Ok(goal) => {
    ///         // var_map records previously recreated variables.
    ///         let mut var_map = VarMap::new();
    ///         let goal = goal.recreate_variables(&mut var_map);
    ///         println!("{}", goal);
    ///     },
    ///     Err(msg) => { println!("{}", msg); },
    /// }
    /// // Prints: father($X_1, $Z_2), mother($Z_2, $Y_3)
    /// ```
    pub fn recreate_variables(self, vars: &mut VarMap) -> Goal {

        match self {
            Goal::OperatorGoal(op) => {
               return Goal::OperatorGoal(op.recreate_variables(vars));
            },
            Goal::ComplexGoal(u) => {
                if let Unifiable::SComplex(_) = u {
                    return Goal::ComplexGoal(u.recreate_variables(vars));
                }
                else { panic!("recreate_variables() - Unifiable must be SComplex."); }
            },
            Goal::BuiltInGoal(bipred) => {
                return Goal::BuiltInGoal(bipred.recreate_variables(vars));
            },
            _ => { panic!("{}", "goal.rs - Unknown goal."); },
        } // match

    } // recreate_variables()

    /// Replaces logic variables with their ground terms from the substitution set.
    ///
    /// This method is useful for displaying the results of a query.
    ///
    /// For example, if the query `loves(Leonard, $Whom)` has a solution,
    /// calling `replace_variables()` will produce a new term which shows
    /// the solution, eg. `loves(Leonard, Penny)`.
    ///
    /// # Arguments
    /// * self
    /// * [substitution_set](../substitution_set/index.html)
    /// # Return
    /// * new term - should contain no variables
    /// # Panics
    /// * If the goal is not a
    /// [ComplexGoal](../goal/enum.Goal.html#variant.ComplexGoal)
    /// # Usage
    /// ```
    /// use std::rc::Rc;
    /// use suiron::*;
    ///
    /// // Setup test knowledge base and base solution node.
    /// let kb = test_kb();
    /// let query = parse_query("loves(Leonard, $Whom)").unwrap();
    /// let q = Rc::new(query);
    /// let base = make_base_node(Rc::clone(&q), &kb);
    ///
    /// // Find a solution.
    /// let solution = next_solution(base);
    /// match solution {
    ///    Some(ss) => { println!("{}", q.replace_variables(&ss)); },
    ///    None => { println!("No more."); },
    /// }
    /// // Prints: loves(Leonard, Penny)
    /// ```
    pub fn replace_variables(&self, ss: &SubstitutionSet) -> Unifiable {

        match self {
            Goal::ComplexGoal(cmplx) => { return cmplx.replace_variables(&ss); },
            _ => { panic!("replace_variables() - Not a complex goal."); }
        }

    } // replace_variables()

    /// Creates a key (= predicate name) for indexing into a
    /// [knowledge base](../knowledge_base/index.html).
    ///
    /// The name of a predicate consists of its functor plus its arity,
    /// separated by a slash. For example, for the fact `loves(Chandler, Monica)`,
    /// the functor is `loves` and the arity is 2, therefore the name of the
    /// predicate is `loves/2`.
    ///
    /// # Panics
    /// * If self is not a
    /// [complex](../unifiable/enum.Unifiable.html#variant.SComplex) term.
    /// # Usage
    /// ```
    /// use suiron::*;
    ///
    /// let goal = parse_subgoal("loves(Chandler, Monica)").unwrap();
    /// let key = goal.key();
    /// println!("{}", key);  // Should print: loves/2
    /// ```
    pub fn key(&self) -> String {
        match self {
            Goal::ComplexGoal(unifiable_term) => {
                match unifiable_term {
                    Unifiable::SComplex(terms) => {
                        let functor = &terms[0];
                        let arity = terms.len() - 1;
                        return format!("{}/{}", functor, arity);
                    },
                    _ => { panic!("Goal::key() - \
                           Valid only for SComplex. {}", unifiable_term); }
                }
            },
            _ => { panic!("Goal::key() - \
                           Valid only for ComplexGoal: {}", self); },
        } // match
    } // key()

    /// Gets a ground term of a query-goal.
    ///
    /// This method is useful for getting the results of a query.
    ///
    /// # Arguments
    /// * self
    /// * index of term
    /// * [SubstitutionSet](../substitution_set/index.html)
    /// (containing solution of query)
    /// # Returns
    /// * [Unifiable](../unifiable/enum.Unifiable.html) or None
    ///
    /// # Usage
    /// ```
    /// use suiron::*;
    /// use std::rc::Rc;
    ///
    /// let kb = test_kb();
    /// let query = parse_query("loves(Penny, $Whom)").unwrap();
    /// let query = Rc::new(query);
    /// let base_node = make_base_node(Rc::clone(&query), &kb);
    /// let solution = next_solution(base_node);
    ///
    /// match solution {
    ///    Some(ss) => {
    ///        // Get the result.
    ///        // The index of the variable $Whom is 2.
    ///        match query.get_ground_term(2, ss) {
    ///            Some(result) => { println!("{}", result); },
    ///            None => { println!("No solution."); },
    ///        }
    ///    },
    ///    None => { println!("No solution."); },
    /// } // match solution
    /// // Prints: Leonard
    /// ```
    pub fn get_ground_term<'a>(&self, index: usize,
                               ss: Rc<SubstitutionSet<'a>>) -> Option<Unifiable> {
        match self {
            Goal::ComplexGoal(Unifiable::SComplex(terms)) => {
                match get_binding(&terms[index], &ss) {
                    Some(term) => { Some(term.clone()) },
                    None => { None },
                }
            },
            _ => None,
        } // match

    } // get_ground_term

} // impl Goal


/// Makes a base solution node for a proof tree.
///
/// The parent node of the base node is initialized to None.<br>
/// The parent solution is initialized to an empty substitution set.
///
/// # Arguments
/// * goal to be proven
/// * knowledge base
/// # Return
/// * reference to a [SolutionNode](../solution_node/struct.SolutionNode.html)
///
/// # Panics
/// * If the goal is not a
/// [ComplexGoal](../goal/enum.Goal.html#variant.ComplexGoal)
/// # Usage
/// ```
/// use std::rc::Rc;
/// use suiron::*;
///
/// let kb = test_kb();
/// let query = parse_query("loves($Who, $Whom)").unwrap();
/// let base_node = make_base_node(Rc::new(query), &kb);
/// ```
pub fn make_base_node<'a>(goal: Rc<Goal>, kb: &'a KnowledgeBase)
                         -> Rc<RefCell<SolutionNode<'a>>> {

    // Get predicate key for knowledge base.
    let key: String;
    match &*goal {
        Goal::ComplexGoal(cmplx) => { key = cmplx.key(); },
        _ => { panic!("make_base_node() - Goal must be a ComplexGoal."); },
    };

    let mut node = SolutionNode::new(goal, kb);
    node.number_facts_rules = count_rules(kb, &key);
    return rc_cell!(node);

} // make_base_node()

/// Makes a solution node according to the type of goal.
///
/// # Arguments
/// * goal to be proven
/// * knowledge base
/// * parent substitution set
/// * parent solution node
/// # Return
/// * reference to a [SolutionNode](../solution_node/struct.SolutionNode.html)
/// # Usage
/// ```
/// use std::rc::Rc;
/// use std::cell::RefCell;
/// use suiron::*;
///
/// // Setup a base solution node.
/// let kb = KnowledgeBase::new();
/// let query = parse_query("goal1()").unwrap();
/// let base = make_base_node(Rc::new(query), &kb);
///
/// // Setup another solution node.
/// let ss = empty_ss!();
/// let query = parse_query("goal2()").unwrap();
/// let sn = make_solution_node(Rc::new(query), &kb, ss, Rc::clone(&base));
/// ```
pub fn make_solution_node<'a>(goal: Rc<Goal>,
                              kb: &'a KnowledgeBase,
                              ss: Rc<SubstitutionSet<'a>>,
                              parent_node: Rc<RefCell<SolutionNode<'a>>>)
                              -> Rc<RefCell<SolutionNode<'a>>> {

    // Make a solution node with defaults.
    let mut node = SolutionNode::new(Rc::clone(&goal), kb);
    node.parent_node = Some(parent_node);

    match &*goal {

        Goal::OperatorGoal(op) => {

            match op {

                Operator::Or(_) | Operator::And(_) => {

                    node.ss = Rc::clone(&ss);
                    let (head, tail) = op.split_head_tail();
                    node.operator_tail = Some(tail);

                    let rc_node = rc_cell!(node);

                    // Solution node of first goal.
                    let head_node = make_solution_node(Rc::new(head), kb,
                                                       Rc::clone(&ss),
                                                       Rc::clone(&rc_node));

                    set_head_node(&rc_node, head_node);
                    return rc_node;
                },
                Operator::Time(goals) |
                Operator::Not(goals) => {
                    node.ss = Rc::clone(&ss);
                    let goal = goals[0].clone();
                    let rc_node = rc_cell!(node);
                    let head_node = make_solution_node(Rc::new(goal), kb,
                                                       Rc::clone(&ss),
                                                       Rc::clone(&rc_node));
                    set_head_node(&rc_node, head_node);
                    return rc_node;
                },

            } // match op
        },
        Goal::ComplexGoal(cmplx) => {

            node.ss = ss;

            // Count the number of rules or facts which match the goal.
            node.number_facts_rules = count_rules(kb, &cmplx.key());
            return rc_cell!(node);

        },
        Goal::BuiltInGoal(_) => {

            node.ss = ss;
            return rc_cell!(node);

        },
        Goal::Nil => { panic!("goal.rs - Implement later."); },

    } // match
} // make_solution_node()

/// This helper function sets the head_sn field of the given and/or node.
///
/// # Arguments
/// * solution node
/// * head solution node
fn set_head_node<'a>(node: &Rc<RefCell<SolutionNode<'a>>>,
                     head_sn: Rc<RefCell<SolutionNode<'a>>>) {
    let mut mut_node = node.borrow_mut();
    mut_node.head_sn = Some(head_sn);
}

// Display trait, to display goals.
impl fmt::Display for Goal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Goal::OperatorGoal(operator) => { write!(f, "{}", operator) },
            Goal::ComplexGoal(complex) => { write!(f, "{}", complex) },
            Goal::BuiltInGoal(pred) => { write!(f, "{}", pred) },
            Goal::Nil => { write!(f, "Nil") },
        } // match
    } // fmt
} // fmt::Display

#[cfg(test)]
mod test {

    // Why run tests serially?
    // LOGIC_VAR_ID is a global variable, which is unsafe.
    // For testing purposes, logic variable IDs should always be consistent.
    use serial_test::serial;

    use std::rc::Rc;
    use crate::*;

    // goals1 - Test key(), recreate_variables() and replace_variables().
    #[test]
    #[serial]
    fn test_goals1() {

        start_query();  // SUIRON_STOP_QUERY = false, LOGIC_VAR_ID = 0

        let goal = parse_subgoal("grandfather($Who, Aethelstan)").unwrap();
        let s1 = format!("{}", goal);
        assert_eq!("grandfather($Who, Aethelstan)", s1);

        let predicate_name = goal.key();
        assert_eq!("grandfather/2", predicate_name);

        // var_map lists previously recreated logic variables.
        let mut var_map = VarMap::new();
        let query = goal.recreate_variables(&mut var_map);

        let s1 = format!("{}", query);
        assert_eq!("grandfather($Who_1, Aethelstan)", s1);

        // Set up substitution set.
        let mut ss = SubstitutionSet::new();
        ss.push(None);
        ss.push(Some(Rc::new(Atom("Alfred".to_string()))));

        let result = query.replace_variables(&ss);
        let s1 = format!("{}", result);
        assert_eq!("grandfather(Alfred, Aethelstan)", s1);

    } // test_goals1

    // goals2 - Test base_node() and get_sn().
    #[test]
    #[serial]
    fn test_goals2() {

        start_query();  // SUIRON_STOP_QUERY = false, LOGIC_VAR_ID = 0

        let kb = test_kb();

        // Make a base node from a query: grandfather($X, $Y)
        // (Don't worry about recreating variable IDs for this test.)
        let query = parse_subgoal("grandfather($X, $Y)").unwrap();
        let base_node = make_base_node(Rc::new(query), &kb);
        let s1 = format!("{}", base_node.borrow());
        let s2 = "----- Solution Node -----\n\
                  \tgoal: grandfather($X, $Y)\n\
                  \tparent_node: None\n\
                  \tno_backtracking: false\n\
                  \trule_index: 0\n\
                  \tnumber_facts_rules: 2\n\
                  \thead_sn: None\n\
                  \toperator_tail: None\n\
                  -------------------------";
        assert_eq!(s1, s2);

        // Make an And goal: father($X, $Z), father($Z, $Y)
        let goal1 = parse_subgoal("father($X, $Z)").unwrap();
        let goal2 = parse_subgoal("father($Z, $Y)").unwrap();
        let goal3 = operator_and!(goal1, goal2);

        let ss = empty_ss!();
        let node = make_solution_node(Rc::new(goal3), &kb, ss, base_node);
        let s1 = format!("{}", node.borrow());
        let s2 = "----- Solution Node -----\n\
                  \tgoal: father($X, $Z), father($Z, $Y)\n\
                  \tparent_node (goal only): grandfather($X, $Y)\n\
                  \tno_backtracking: false\n\
                  \trule_index: 0\n\
                  \tnumber_facts_rules: 0\n\
                  \thead_sn (goal only): father($X, $Z)\n\
                  \toperator_tail: father($Z, $Y)\n\
                  -------------------------";
        assert_eq!(s1, s2);
    } // test_goals2

} // test
