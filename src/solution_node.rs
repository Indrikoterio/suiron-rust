//! A SolutionNode is a node in a proof tree.
//!
//! [Goals](../goal/enum.Goal.html) (operators, built-in predicates, and complex
//! terms) implement a method called
//! [get_sn()](../goal/enum.Goal.html#method.get_sn), which returns a solution
//! node appropriate to each operator, built-in predicate, or complex term.
//!
//! The function
//! [next_solution()](../solution_node/fn.next_solution.html),
//! initiates the search for a solution. When a solution is found, the search stops.
//!
//! Each solution node preserves its state (goal, parent_solution, rule_index, etc.).
//! Calling next_solution() again will continue the search for alternative solutions.
//!
// Cleve Lendon 2023

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Instant;

use crate::*;

use super::goal::Goal;
use super::substitution_set::*;
use super::knowledge_base::*;

/// Represents a node in a proof tree.
///
/// A solution node holds the goal to be resolved, various
/// parameters concerning that goal, and a reference to the
/// parent_solution, which represents the state of the search
/// so far.
///
#[derive(Debug, Clone)]
pub struct SolutionNode<'a> {

    /// The goal which this solution node seeks to resolve.
    pub goal: Goal,
    /// Reference to the Knowledge Base.
    pub kb: &'a KnowledgeBase,
    /// Reference to the parent node in the proof tree.
    pub parent_node: Option<Rc<RefCell<SolutionNode<'a>>>>,
    /// Represents the solution up to this point in the proof tree.
    pub parent_solution: Rc<SubstitutionSet<'a>>,
    /// Used by the Cut operator (!) to prevent back-tracking.
    pub no_backtracking: bool,

    // For Complex Solution Nodes.
    /// Refers to the solution node of a rule's body. (For Complex goals.)
    pub child: Option<Rc<RefCell<SolutionNode<'a>>>>,
    /// The index of a fact or rule. (For Complex goals.)
    pub rule_index: usize,
    /// The number of facts and rules for the goal above. (For Complex goals.)
    pub number_facts_rules: usize,

    // For And/Or Solution Nodes.
    /// Head solution node. (For And/Or goals.)
    pub head_sn: Option<Rc<RefCell<SolutionNode<'a>>>>,
    /// Tail solution node. (For And/Or goals.)
    pub tail_sn: Option<Rc<RefCell<SolutionNode<'a>>>>,
    /// Tail of And/Or operator. (For And/Or goals.)
    pub operator_tail: Option<Operator>,

    /// For built-in predicates which have only 1 solution.
    pub more_solutions: bool,
}

impl<'a> SolutionNode<'a> {

    /// Creates a new SolutionNode struct, with default values.
    ///
    /// The parent_node is set to None, and the parent_solution
    /// is initialized to an empty substitution set.
    ///
    /// # Arguments
    /// * `goal`
    /// * `kb` - Knowledge Base
    /// # Returns
    /// * `SolutionNode`
    /// # Usage
    /// ```
    /// use suiron::*;
    ///
    /// let goal = parse_query("test($X)").unwrap();
    /// let kb   = test_kb();
    /// let node = SolutionNode::new(goal, &kb);
    /// ```
    #[inline]
    pub fn new(goal: Goal, kb: &'a KnowledgeBase) -> Self {
        SolutionNode {
            goal, kb,
            parent_node: None,
            parent_solution: empty_ss!(),
            no_backtracking: false,
            child: None,
            rule_index: 0,
            number_facts_rules: 0,
            head_sn: None,
            tail_sn: None,
            operator_tail: None,
            more_solutions: true,
        }
    } // new()

} // impl SolutionNode

/// Finds the first and next solutions of the given solution node.
///
/// This method fetches facts and rules from the knowledge base,
/// and attempts to unify (match) each fact or rule head with the goal.
/// When unification succeeds, if the rule has no body (ie. it is a fact),
/// the method returns the updated substitution set.<br>
///
/// If there is a body, the method gets its solution node (child node)
/// and attempts to solve that.<br>
///
/// If a fact or rule fails to unify, the method will fetch the next
/// fact/rule until the relevant predicate in the knowledge base has
/// been exhausted. In such a case, the method returns None to indicate
/// failure.
/// # Arguments
/// * `sn` - solution node
/// # Return
/// * `Option` -
/// Some([SubstitutionSet](../substitution_set/type.SubstitutionSet.html))
/// or None
/// # Usage
/// ```
/// use std::rc::Rc;
/// use std::cell::RefCell;
/// use suiron::*;
///
/// let kb = test_kb();
///
/// // Whom does Leonard love?
/// // Make a query and a solution node.
/// let query = parse_query("loves(Leonard, $Whom)").unwrap();
/// let solution_node = query.base_node(&kb);
///
/// // Get a solution.
/// match next_solution(solution_node) {
///     Some(ss) => {
///         let result = query.replace_variables(&ss);
///         println!("{}", result);
///     },
///     None => { println!("No."); },
/// }
/// // Prints: loves(Leonard, Penny)
/// ```
pub fn next_solution<'a>(sn: Rc<RefCell<SolutionNode<'a>>>)
                         -> Option<Rc<SubstitutionSet<'a>>> {

    let mut sn_ref = sn.borrow_mut(); // Get a mutable reference.
    if sn_ref.no_backtracking { return None; };

    let goal = sn_ref.goal.clone();

    match goal {

        Goal::OperatorGoal(op) => {

            match op {

                Operator::And(_) => {
                    return next_solution_and(Rc::clone(&sn), sn_ref);
                },
                Operator::Or(_) => {
                    return next_solution_or(Rc::clone(&sn), sn_ref);
                },
                Operator::Time(goals) => {

                    if goals.len() < 1 { return None; }
                    let now = Instant::now();

                    match sn_ref.head_sn {
                        None => {
                            let goal = goals[0].clone();
                            let parent_solution =
                                       Rc::clone(&sn_ref.parent_solution);
                            let sn = goal.get_sn(sn_ref.kb, parent_solution,
                                                 Rc::clone(&sn));
                            sn_ref.head_sn = Some(sn);
                        },
                        Some(_) => {},
                    } // match

                    match &sn_ref.head_sn {
                        Some(head_sn) => {
                            let solution = next_solution(Rc::clone(&head_sn));
                            let elapsed = now.elapsed();
                            let micro = elapsed.subsec_nanos() / 1000;
                            println!("{} seconds {} microseconds", elapsed.as_secs(), micro);
                            return solution;
                        },
                        None => { panic!("next_solution() - \
                                  Missing solution node. Should not happen."); },
                    } // match
                },
            } // match op

        }, // Goal::OperatorGoal(op)

        Goal::ComplexGoal(cmplx) => {

            // Check for a child solution.
            match &sn_ref.child {
                None => {},
                Some(child_sn) => {
                    let solution = next_solution(Rc::clone(&child_sn));
                    match solution {
                        None => {},
                        Some(ss) => { return Some(ss); },
                    }
                },
            }

            sn_ref.child = None;
            loop {

                if sn_ref.rule_index >= sn_ref.number_facts_rules { return None; }

                // The fallback_id saves the logic variable ID (LOGIC_VAR_ID),
                // in case the next rule fails. Restoring this id will keep
                // the length of the substitution set as short as possible.
                let fallback_id = get_var_id();

                let pred_name = sn_ref.goal.key();
                let rule = get_rule(sn_ref.kb, &pred_name, sn_ref.rule_index);
                sn_ref.rule_index += 1;

                let head = rule.get_head();
                let solution = head.unify(&cmplx, &Rc::clone(&sn_ref.parent_solution));

                match solution {
                    None => { set_var_id(fallback_id); },  // Restore fallback ID.
                    Some(ss) => {
                        let body = rule.get_body();
                        if body == Goal::Nil { return Some(ss); }
                        let child_sn = body.get_sn(sn_ref.kb, ss, Rc::clone(&sn));
                        sn_ref.child = Some(Rc::clone(&child_sn));
                        let child_solution = next_solution(child_sn);
                        match child_solution {
                            None => {},
                            Some(ss) => { return Some(ss); },
                        }
                    },
                } // match
            }
        },

        Goal::BuiltInGoal(built_in_predicate) => {
            return next_solution_bip(built_in_predicate, sn_ref);
        },

        _ => panic!("next_solution() - Implement this."),

    } // match self

} // next_solution()


/// Displays a summary of a solution node for debugging purposes.<br>
/// KB, parent_solution and tail_sn are excluded. For example:
/// <pre>
/// ----- Solution Node -----
/// 	goal: grandfather($X, $Y)
/// 	parent_node: None
/// 	no_back_tracking: false
/// 	rule_index: 0
/// 	number_facts_rules: 2
/// 	head_sn: None
/// 	operator_tail: None
/// -------------------------
/// </pre>

impl fmt::Display for SolutionNode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = "----- Solution Node -----\n".to_string();
        out += &format!("\tgoal: {}\n", self.goal);
        match &self.parent_node {
            Some(parent) => {
               let parent = parent.borrow();
               out += &format!("\tparent_node (goal only): {}\n", parent.goal);
            },
            None => { out += "\tparent_node: None\n"},
        }
        out += &format!("\tno_back_tracking: {}\n", self.no_backtracking);
        out += &format!("\trule_index: {}\n", self.rule_index);
        out += &format!("\tnumber_facts_rules: {}\n", self.number_facts_rules);
        match &self.head_sn {
            Some(head_sn) => {
               let h = head_sn.borrow();
               out += &format!("\thead_sn (goal only): {}\n", h.goal);
            },
            None => { out += "\thead_sn: None\n"},
        }
        match &self.operator_tail {
            Some(operator_tail) => {
               let tail = operator_tail.clone();
               out += &format!("\toperator_tail: {}\n", tail);
            },
            None => { out += "\toperator_tail: None\n"},
        }
        out += "-------------------------";
        write!(f, "{}", out)
    } // fmt
} // fmt::Display


#[cfg(test)]
mod test {

    use crate::*;
    use std::rc::Rc;
    use serial_test::serial;

    // Test the set_no_back() function.
    // Create two solution nodes. The parent of sn2 is sn1.
    // Setting the no_back_tracking flag on sn2 should also
    // set it on sn1.
    #[test]
    #[serial]
    fn test_set_no_back() {

        // Set up a solution node.
        let kb = KnowledgeBase::new();
        let query = parse_query("goal1()").unwrap();
        let sn1 = query.base_node(&kb);

        // Set up another solution node. The parent node is sn1.
        let ss = empty_ss!();
        let query = parse_query("goal2()").unwrap();
        let sn2 = query.get_sn(&kb, ss, Rc::clone(&sn1));

        assert_eq!(false, sn1.borrow().no_back_tracking);

        // Set the no_back_tracking flag on the child node.
        sn2.borrow_mut().set_no_back();

        assert_eq!(true, sn1.borrow().no_back_tracking);

    }  // test_set_no_back()

    // The test knowledge base has two predicates named love/2.
    // This test function makes a query about who loves whom, and
    // a corresponding solution node. The method next_solution()
    // is called three times to confirm that all valid solutions
    // can be found.
    #[test]
    #[serial]
    fn test_next_solution1() {

        start_query();  // SUIRON_STOP_QUERY = false, LOGIC_VAR_ID = 0

        let kb = test_kb();

        // Make a solution node for love.
        let query = parse_query("loves($Who, $Whom)").unwrap();
        let sn = query.base_node(&kb);

        // Who loves whom?
        let ss = next_solution(Rc::clone(&sn)).unwrap();
        let result = query.replace_variables(&ss);
        let s = format!("{}", result);
        assert_eq!("loves(Leonard, Penny)", s);

        // Who loves whom?
        let ss = next_solution(Rc::clone(&sn)).unwrap();
        let result = query.replace_variables(&ss);
        let s = format!("{}", result);
        assert_eq!("loves(Penny, Leonard)", s);

        // All solutions found?
        match next_solution(sn) {
            Some(_) => { panic!("The love predicate should be exhausted."); },
            None => { }, // I'm all outta love.
        }

    } // test_next_solution1()

    // The test knowledge base has two rules which define a grandfather.
    //    grandfather($X, $Y) :- father($X, $Z), father($Z, $Y).
    //    grandfather($X, $Y) :- father($X, $Z), mother($Z, $Y).
    // This test function makes a query to find grandfathers, and
    // a corresponding solution node. The method next_solution()
    // is called twice times to confirm that all valid solutions
    // can be found.
    #[test]
    #[serial]
    fn test_next_solution2() {

        start_query();  // SUIRON_STOP_QUERY = false, LOGIC_VAR_ID = 0
        let kb = test_kb();

        // Make a solution node to find grandfathers.
        let query = parse_query("grandfather($Who, $Whom)").unwrap();
        let sn = query.base_node(&kb);

        match next_solution(Rc::clone(&sn)) {
            Some(ss) => {
                let s1 = format!("{}", query.replace_variables(&ss));
                assert_eq!("grandfather(Alfred, Aethelstan)", s1);
            },
            None => { panic!("Cannot find grandfather."); },
        }

        match next_solution(Rc::clone(&sn)) {
            Some(_) => { panic!("Solutions should be exhausted."); },
            None => { },
        }

    } // test_next_solution2()

} // test
