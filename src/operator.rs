//! A base type for And and Or logic operators, etc.
//!
//! Methods for accessing and displaying operands
//! (which are [Goals](../goal/index.html)).
//!
// Cleve Lendon 2023

use std::fmt;

use super::goal::*;
use super::unifiable::*;
use super::logic_var::*;

/// Defines logical And, Or, etc. An operator holds a vector of goals.
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    /// Logical And operator.
    And(Vec<Goal>),
    /// Logical Or operator.
    Or(Vec<Goal>),
    /// Time - Measures elapsed time.
    // Note: time() only takes one goal, but indirection is needed.
    // Therefore, it is necessary to enclose the goal in a vector.
    Time(Vec<Goal>),
    /// Not - Succeeds if goal argument cannot be proven true.
    Not(Vec<Goal>),
}

impl Operator {

    /// Splits the operands into head and tail.
    ///
    /// The head is the first [Goal](../goal/index.html).
    /// The tail is an Operator (same variant) which holds
    /// the remaining Goals.
    ///
    /// # Arguments
    /// * `self`
    /// # Return
    /// * `(head, tail)` - [Goal](../goal/index.html), Operator
    /// # Panics
    /// * If operator is not And or Or.
    /// * If there are no operands.
    /// # Usage
    /// ```
    /// use suiron::*;
    ///
    /// let m = parse_subgoal("mother($X, $Y)").unwrap();
    /// let f = parse_subgoal("father($X, $Y)").unwrap();
    /// let s = parse_subgoal("sibling($X, $Y)").unwrap();
    /// let mfs = operator_or!(m, f, s); // mother or father or sibling
    ///
    /// let (head, tail) = mfs.split_head_tail();
    /// // Head is a Goal: mother
    /// // Tail is an Operator: father or sibling
    /// ```
    pub fn split_head_tail(&self) -> (Goal, Operator) {

        match &self {
            Operator::And(op) => {
                if op.len() == 0 { panic!("split_head_tail() - No operands."); }
                let mut operands = op.clone();
                let head = operands.remove(0);
                let tail = Operator::And(operands);
                return (head, tail);
            },
            Operator::Or(op) => {
                if op.len() == 0 { panic!("split_head_tail() - No operands."); }
                let mut operands = op.clone();
                let head = operands.remove(0);
                let tail = Operator::Or(operands);
                return (head, tail);
            },
            _ => { panic!("split_head_tail() - Valid for And and Or operators only."); },
        }
    } // split_head_tail()


    /// Give logic variables unique IDs.
    ///
    /// Logic variables in the knowledge base have an ID of 0, but
    /// when a rule is fetched from the knowledge base, the logic
    /// variables must be given unique IDs.
    ///
    /// # Arguments
    /// * `self`
    /// * `vars` - set of previously recreated variable IDs
    /// # Return
    /// * `Operator`
    /// # Usage
    /// ```
    /// use suiron::*;
    ///
    /// // Make an And operator: parent($X, $Y), female($X)
    /// let parent = parse_subgoal("parent($X, $Y)").unwrap();
    /// let female = parse_subgoal("female($X)").unwrap();
    /// let op = operator_and!(parent, female);
    ///
    /// let mut var_map = VarMap::new();
    /// let op2 = op.recreate_variables(&mut var_map);
    /// println!("{}", op2); // Prints: parent($X_1, $Y_2), female($X_1)
    /// ```
    pub fn recreate_variables(self, vars: &mut VarMap) -> Operator {
        match self {
            Operator::And(goals) => {
                Operator::And(recreate_vars_goals(goals, vars))
            },
            Operator::Or(goals) => {
                Operator::Or(recreate_vars_goals(goals, vars))
            },
            Operator::Time(goals) => {
                Operator::Time(recreate_vars_goals(goals, vars))
            },
        }
    }

    /// Counts the number of subgoals in the operator.
    ///
    /// # Arguments
    /// * `self`
    /// # Return
    /// * number of subgoals
    pub fn len(&self) -> usize {
        match self {
            Operator::And(goals) |
            Operator::Or(goals) |
            Operator::Time(goals) => { return goals.len(); },
        }
    }

    /// Get the indexed subgoal from the operator.
    ///
    /// The operator must be And or Or.
    ///
    /// # Arguments
    /// * `self`
    /// * `index`
    /// # Return
    /// * [Goal](../goal/index.html)
    /// # Usage
    /// ```
    /// use suiron::*;
    ///
    /// // Make an And operator: parent($X, $Y), female($X)
    /// let parent = parse_subgoal("parent($X, $Y)").unwrap();
    /// let female = parse_subgoal("female($X)").unwrap();
    /// let op = operator_and!(parent, female);
    ///
    /// let goal = op.get_subgoal(1); // Get second subgoal.
    /// println!("{}", goal);  // Prints: female($X)
    /// ```
    pub fn get_subgoal(&self, index: usize) -> Goal {
        match self {
            Operator::And(goals) |
            Operator::Or(goals) |
            Operator::Time(goals)=> { return goals[index].clone(); },
        }
    } // get_subgoal()

} // impl Operator

/// Formats a list for the Display trait.
///
/// For example, assuming 'goals' contains [a, b, c],
/// format_list(goals, ", ") will produce: "a, b, c"
///
/// # Arguments
/// * operands - a vector of Goals or Unifiable terms
/// * separator - Eg. "," ";" " = "
/// # Return
/// * string representation of list
fn format_list<T>(operands: &Vec<T>, separator: &str)
                  -> String where T: std::fmt::Display {
    let mut out = "".to_string();
    let mut first = true;
    for op in operands {
        if first {
            out += &op.to_string();
            first = false;
        }
        else {
            out += separator;
            out += &op.to_string();
        }
    }
    return out;
} // format_list()

// Display trait, to display operators.
impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Operator::And(goals) => {
                write!(f, "{}", format_list(goals, ", "))
            },
            Operator::Or(goals) => {
                write!(f, "{}", format_list(goals, "; "))
            },
            Operator::Time(goals) => {
                write!(f, "{}", goals[0])
            },
        } // match
    } // fmt
} // fmt::Display


#[cfg(test)]
mod test {

    use crate::*;

    // Create logic vars for testing.
    fn x() -> Unifiable { logic_var!("$X") }
    fn y() -> Unifiable { logic_var!("$Y") }

    // Make complex terms for testing.
    fn make_parent() -> Unifiable {
        scomplex!(atom!("parent"), x(), y())  // parent($X, $Y)
    }

    fn make_male() -> Unifiable {
        scomplex!(atom!("male"), x())  // male($X)
    }

    fn make_mother() -> Unifiable {
        scomplex!(atom!("mother"), x(), y())  // mother($X, $Y)
    }

    fn make_father() -> Unifiable {
        scomplex!(atom!("father"), x(), y())  // father($X, $Y)
    }

    // Functions which make Operators.
    fn make_and() -> operator::Operator {
        let goal1 = Goal::ComplexGoal(make_parent());
        let goal2 = Goal::ComplexGoal(make_male());
        operator_and!(goal1, goal2) // parent($X, $Y), male($X)
    }

    fn make_or() -> operator::Operator {
        let goal1 = Goal::ComplexGoal(make_mother());
        let goal2 = Goal::ComplexGoal(make_father());
        operator_or!(goal1, goal2) // mother($X, $Y); father($X, $Y)
    }

    // Test creation and display of operators.
    // AND:   parent($X, $Y), male($X).
    // OR:    mother($X, $Y); father($X, $Y)
    // UNIFY: $Z = 7
    #[test]
    fn test_creation_of_operators() {

        let op1 = make_and();
        let op2 = make_or();

        let s = format!("{}", op1);
        assert_eq!("parent($X, $Y), male($X)", s);

        let s = format!("{}", op2);
        assert_eq!("mother($X, $Y); father($X, $Y)", s);
    }

    #[test]
    fn test_split_head_tail() {

        let and_op = make_and();  // parent($X, $Y), male($X)
        let (head, tail) = and_op.split_head_tail();
        assert_eq!(tail.len(), 1);
        let male = Goal::ComplexGoal(make_male());
        assert_eq!(tail.get_subgoal(0), male);
        let parent = Goal::ComplexGoal(make_parent());
        assert_eq!(head, parent);

        let (_, tail) = tail.split_head_tail();
        assert_eq!(tail.len(), 0);
    }

    // The method split_head_tail() should panic
    // if there are no operands. (Length == 0)
    #[test]
    #[should_panic]
    fn test_split_head_tail_panic2() {
        let and_op = operator_and!();
        and_op.split_head_tail();
    }

} // test
