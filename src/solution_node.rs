//! A SolutionNode is a node in a proof tree.
//!
//! It contains references to the [goal](../goal/enum.Goal.html) to be
//! solved, the [knowledge base](../suiron/knowledge_base/index.html),
//! the [SubstitutionSet](../substitution_set/index.html), and other
//! relevant data.
//!
//! The function
//! [next_solution()](../solution_node/fn.next_solution.html),
//! accepts a solution node as its argument, and initiates the search
//! for a solution. When a solution is found, the search stops.
//!
//! Each solution node preserves its state. Calling next_solution() again
//! will continue the search for alternative solutions.
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
/// A solution node holds the goal to be resolved, various parameters
/// concerning that goal, and a reference to the substitution set,
/// which represents the state of the search so far.
///
#[derive(Debug, Clone)]
pub struct SolutionNode<'a> {

    /// The goal which this solution node seeks to resolve.
    pub goal: Rc<Goal>,
    /// Reference to the Knowledge Base.
    pub kb: &'a KnowledgeBase,

    /// Reference to the parent node in the proof tree.
    pub parent_node: Option<Rc<RefCell<SolutionNode<'a>>>>,
    /// Substitution Set - holds the complete or partial solution.
    pub ss: Rc<SubstitutionSet<'a>>,
    /// Flag used by the Cut operator (!) to prevent backtracking.
    pub no_backtracking: bool,

    // For Complex Solution Nodes.
    /// Refers to the solution node of a rule's body. (For Complex goals.)
    pub child: Option<Rc<RefCell<SolutionNode<'a>>>>,
    /// The index of a fact or rule. (For Complex goals.)
    pub rule_index: usize,
    /// The number of facts and rules for the goal above. (For Complex goals.)
    pub number_facts_rules: usize,

    // For And/Or Solution Nodes.
    /// Head solution node.
    pub head_sn: Option<Rc<RefCell<SolutionNode<'a>>>>,
    /// Tail solution node. (For And/Or goals.)
    pub tail_sn: Option<Rc<RefCell<SolutionNode<'a>>>>,
    /// Tail of And/Or operator. (For And/Or goals.)
    pub operator_tail: Option<Operator>,

    /// Flag for built-in predicates, which have only 1 solution.
    pub more_solutions: bool,

} // SolutionNode

impl<'a> SolutionNode<'a> {

    /// Creates a new SolutionNode struct, with default values.
    ///
    /// The parent_node is set to None, and the solution (ss)
    /// is initialized to an empty substitution set.
    ///
    /// # Usage
    /// ```
    /// use std::rc::Rc;
    /// use suiron::*;
    ///
    /// let goal = parse_query("test($X)").unwrap();
    /// let kb   = test_kb();
    /// let node = SolutionNode::new(Rc::new(goal), &kb);
    /// ```
    #[inline]
    pub fn new(goal: Rc<Goal>, kb: &'a KnowledgeBase) -> Self {
        SolutionNode {
            goal, kb,
            parent_node: None,
            ss: empty_ss!(),
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

    /// Sets the no_backtracking flag to true.
    ///
    /// The Cut operator (!) calls this method to disable backtracking
    /// on the current node and all of its ancestors.
    ///
    /// # Note
    /// In order to avoid weeks of whack-a-mole with compiler errors,
    /// this method was implemented with 'unsafe' code.
    ///
    /// # Usage
    /// ```
    /// use std::rc::Rc;
    /// use suiron::*;
    ///
    /// let kb = test_kb();
    /// let query = parse_query("test").unwrap();
    /// let solution_node = make_base_node(Rc::new(query), &kb);
    ///
    /// solution_node.borrow_mut().set_no_backtracking();
    /// ```
    pub fn set_no_backtracking(&mut self) {

        self.no_backtracking = true;
        let mut option_parent = &self.parent_node;
        loop {
            match option_parent {
                None => { return; },
                Some(pn) => {
                    let raw_ptr = pn.as_ptr();
                    unsafe {
                        // Set no_backtracking on parent.
                        (*raw_ptr).no_backtracking = true;
                        // If there is a head solution node, set
                        // the no_backtracking flag there also.
                        if let Some(head_node) = &(*raw_ptr).head_sn {
                            let raw_ptr2 = head_node.as_ptr();
                            (*raw_ptr2).no_backtracking = true;
                        }
                        // Get the next parent.
                        option_parent = &(*raw_ptr).parent_node;
                    }
                },
            }
        } // loop

    } // set_no_backtracking()

} // impl SolutionNode

/// Gets the goal from a reference to a solution node.
fn get_goal(sn: &Rc<RefCell<SolutionNode>>) -> Rc<Goal> {
    let g = &sn.borrow().goal;
    Rc::clone(g)
}

/// Gets the no_backtracking flag from a reference to a solution node.
fn no_backtracking(sn: &Rc<RefCell<SolutionNode>>) -> bool {
    sn.borrow().no_backtracking
}

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
///
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
/// let q = Rc::new(query);
/// let solution_node = make_base_node(Rc::clone(&q), &kb);
///
/// // Get a solution.
/// match next_solution(solution_node) {
///     Some(ss) => {
///         let result = q.replace_variables(&ss);
///         println!("{}", result);
///     },
///     None => { println!("No."); },
/// }
/// // Prints: loves(Leonard, Penny)
/// ```
pub fn next_solution<'a>(sn: Rc<RefCell<SolutionNode<'a>>>)
                         -> Option<Rc<SubstitutionSet<'a>>> {

    if no_backtracking(&sn) { return None; }
    let goal = get_goal(&sn);

    match &*goal {

        Goal::OperatorGoal(op) => {

            match op {

                Operator::And(_) => {
                    return next_solution_and(sn);
                },
                Operator::Or(_) => {
                    return next_solution_or(sn);
                },

                Operator::Time(_) => {
                    let sn_ref = sn.borrow_mut();
                    match &sn_ref.head_sn {
                        Some(head_sn) => {
                            let now = Instant::now();
                            let solution = next_solution(Rc::clone(&head_sn));
                            print_elapsed(now);
                            return solution;
                        },
                        None => { panic!("next_solution() - \
                                  Missing solution node. Should not happen."); },
                    } // match
                }, // Time

                Operator::Not(_) => {
                    let sn_ref = sn.borrow_mut();
                    match &sn_ref.head_sn {
                        Some(head_sn) => {
                            let solution = next_solution(Rc::clone(&head_sn));
                            match solution {
                                Some(_) => return None,
                                None => return Some(Rc::clone(&sn_ref.ss)),
                            }
                        },
                        None => { panic!("next_solution() - \
                                  Missing solution node. Should not happen."); },
                    } // match
                }, // Not

            } // match op

        }, // Goal::OperatorGoal(op)

        Goal::ComplexGoal(cmplx) => {

            let mut sn_ref = sn.borrow_mut();

            // Check for a child solution.
            match &sn_ref.child {
                None => {},
                Some(child_sn) => {
                    let solution = next_solution(Rc::clone(&child_sn));
                    if solution.is_some() { return solution; }
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
                let solution = head.unify(&cmplx, &Rc::clone(&sn_ref.ss));

                match solution {
                    None => { set_var_id(fallback_id); },  // Restore fallback ID.
                    Some(ss) => {
                        let body = rule.get_body();
                        if body == Goal::Nil { return Some(ss); }
                        let child_sn = make_solution_node(Rc::new(body),
                                                          sn_ref.kb, ss,
                                                          Rc::clone(&sn));
                        sn_ref.child = Some(Rc::clone(&child_sn));
                        let child_solution = next_solution(child_sn);
                        if child_solution.is_some() { return child_solution; }
                    },
                } // match
            }
        },

        Goal::BuiltInGoal(built_in_predicate) => {
            return next_solution_bip(sn, built_in_predicate.clone());
        },

        _ => panic!("next_solution() - Implement this."),

    } // match self

} // next_solution()


/// A utility for printing elapsed time.
/// # Usage
/// ```
/// use std::rc::Rc;
/// use std::time::Instant;
/// use suiron::*;
///
/// let now = Instant::now();  // Mark start time.
///
/// // Do some stuff.
/// let kb = test_kb();
/// let query = parse_query("loves($Who, $Whom)").unwrap();
/// let solution_node = make_base_node(Rc::new(query), &kb);
/// next_solution(solution_node);
///
/// print_elapsed(now);  // Output elapsed time.
/// ```
pub fn print_elapsed(time: Instant) {
    let elapsed = time.elapsed();
    let seconds = elapsed.as_secs();
    let micro = elapsed.subsec_nanos() / 1000;
    if seconds == 1 {
        print!("{} second {} microseconds ", seconds, micro);
    }
    else {
        print!("{} seconds {} microseconds ", seconds, micro);
    }
} // print_elapsed()


/// Displays a summary of a solution node for debugging purposes.<br>
/// KB, the substitution set (ss) and tail_sn are excluded. For example:
/// <pre>
/// ----- Solution Node -----
/// 	goal: grandfather($X, $Y)
/// 	parent_node: None
/// 	no_backtracking: false
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
        out += &format!("\tno_backtracking: {}\n", self.no_backtracking);
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
               out += &format!("\toperator_tail: {}\n", operator_tail);
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

    // Test the set_no_backtracking() function.
    // Some rules:
    //    get_value($X) :- $X = 1.
    //    get_value($X) :- $X = 2.
    //    test1($X) :- get_value($X), $X == 2.    // one solution
    //    test2($X) :- get_value($X), !, $X == 2. // no solutions
    #[test]
    #[serial]
    fn test_set_no_backtracking() {

        start_query();
        let mut kb = KnowledgeBase::new();
        let rule1 = parse_rule("get_value($X) :- $X = 1.").unwrap();
        let rule2 = parse_rule("get_value($X) :- $X = 2.").unwrap();
        let rule3 = parse_rule("test1($X) :- get_value($X), $X == 2.").unwrap();
        let rule4 = parse_rule("test2($X) :- get_value($X), !, $X == 2.").unwrap();
        add_rules!(&mut kb, rule1, rule2, rule3, rule4);

        // Make a query.
        let query = parse_query("test1($X)").unwrap();
        let sn = make_base_node(Rc::new(query), &kb);

        let solution = next_solution(Rc::clone(&sn));
        match solution {
            Some(_ss) => {},
            None => { panic!("There should be 1 solution."); },
        }

        // Second query.
        let query = parse_query("test2($X)").unwrap();
        let sn = make_base_node(Rc::new(query), &kb);

        let solution = next_solution(Rc::clone(&sn));
        match solution {
            None => {},
            Some(_ss) => { panic!("There should be no solutions."); },
        }

    }  // test_set_no_backtracking()

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
        let q = Rc::new(query);
        let sn = make_base_node(Rc::clone(&q), &kb);

        // Who loves whom?
        let ss = next_solution(Rc::clone(&sn)).unwrap();
        let result = q.replace_variables(&ss);
        let s = format!("{}", result);
        assert_eq!("loves(Leonard, Penny)", s);

        // Who loves whom?
        let ss = next_solution(Rc::clone(&sn)).unwrap();
        let result = q.replace_variables(&ss);
        let s = format!("{}", result);
        assert_eq!("loves(Penny, Leonard)", s);

        // All solutions found?
        match next_solution(sn) {
            Some(_) => { panic!("The love predicate should be exhausted."); },
            None => { }, // I'm all outta love.
        }

    } // test_next_solution1()

    // The test knowledge base has two rules which define a grandfather.
    //     grandfather($X, $Y) :- father($X, $Z), father($Z, $Y).
    //     grandfather($X, $Y) :- father($X, $Z), mother($Z, $Y).
    // This test function makes a query to find grandfathers, and
    // a corresponding solution node. The method next_solution() is
    // called twice to confirm that all valid solutions can be found.

    #[test]
    #[serial]
    fn test_next_solution2() {

        start_query();  // SUIRON_STOP_QUERY = false, LOGIC_VAR_ID = 0
        let kb = test_kb();

        // Make a solution node to find grandfathers.
        let query = parse_query("grandfather($Who, $Whom)").unwrap();
        let q = Rc::new(query);
        let sn = make_base_node(Rc::clone(&q), &kb);

        match next_solution(Rc::clone(&sn)) {
            Some(ss) => {
                let s1 = format!("{}", q.replace_variables(&ss));
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
