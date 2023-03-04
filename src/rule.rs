//! Defines a fact or rule.
//!
//! In Suiron source code, rules have the form:<br>
//! <blockquote>
//! head :- body.
//! </blockquote>
//! Facts are defined as rules without a body:
//! <blockquote>
//! grandfather($X, $Y) :- father($X, $Z), father($Z, $Y).
//! &nbsp; &nbsp; % This is a rule.<br>
//! father(John, Kaitlyn). &nbsp; &nbsp; % This is a fact.
//! </blockquote>
//!
// Cleve Lendon  2023

use std::fmt;

use crate::str_to_chars;
use crate::chars_to_string;

use super::goal::Goal;
use super::parse_goals::*;
use super::s_complex::*;
use super::logic_var::*;
use super::unifiable::Unifiable;
use super::tokenizer::*;

/// Defines a fact or rule.
///
/// A rule consists of a head and a body.
/// The head must be a
/// [complex](../unifiable/enum.Unifiable.html#variant.SComplex) term,
/// and the body is a [goal](../goal/enum.Goal.html).<br>
/// For facts, the body is set to [Nil](../goal/enum.Goal.html#variant.Nil).
#[derive(Debug, Clone)]
pub struct Rule {
    pub head: Unifiable, // Must be a Unifiable::SComplex term.
    pub body: Goal,      // For facts, body is Goal::Nil
}

/// Finds the index of the neck operator (:-) in a vector of characters.
/// # Arguments
/// * `characters`
/// # Return
/// * `Option` - Some(index) or None
fn index_of_neck(chrs: &[char]) -> Option<usize> {
    let mut previous_colon = false;
    for (i, ch) in chrs.iter().enumerate() {
       if *ch == '-' {
           if previous_colon == true { return Some(i - 1); }
       }
       if *ch == ':' { previous_colon = true; }
       else { previous_colon = false; }
    }
    return None;
} // index_of_neck()


/// Create a fact or rule from a string representation.
///
/// # Arguments
/// * `to_parse` - &str
/// # Return
/// * `Result` - Ok([Rule](../rule/struct.Rule.html)) or Err(message)
/// # Usage
/// ```
/// use suiron::*;
///
/// match parse_rule("male(Harold).") {
///     Ok(fact) => { println!("{}", fact); },
///     Err(msg) => { println!("{}", msg); },
/// }
/// // Prints: male(Harold).
///
/// match parse_rule("father($X, $Y) :- parent($X, $Y), male($X).") {
///     Ok(rule) => { println!("{}", rule); },
///     Err(msg) => { println!("{}", msg); },
/// }
/// // Prints: father($X, $Y) :- parent($X, $Y), male($X).
/// ```
pub fn parse_rule(to_parse: &str) -> Result<Rule, String> {

    let s = to_parse.trim();

    // Create vector of characters.
    let mut chrs = str_to_chars!(s);

    let mut length = chrs.len();
    if length < 4 {
        let err = pr_error("Invalid string.", s);
        return Err(err);
    }

    // Remove final period.
    let ch = chrs[length - 1];
    if ch == '.' {
        chrs = chrs[0..length - 1].to_vec();
        length = length - 1;
    }

    match index_of_neck(&chrs[..]) {

        Some(index) => {

            let head_chrs = &chrs[0..index];
            let body_chrs = &chrs[index + 2..length];

            // Make sure there is not a second ':-'.
            if let Some(_) = index_of_neck(&body_chrs) {
                let err = pr_error("Invalid rule.", s);
                return Err(err);
            }

            let head: Unifiable;
            match parse_subgoal(&chars_to_string!(head_chrs)) {
                Ok(sg) => {
                    match sg {
                        Goal::ComplexGoal(h) => { head = h; },
                        _ => { panic!("parse_rule() - \
                               Head of rule must be complex term."); },
                    }
                },
                Err(err) => { return Err(err); },
            }

            match generate_goal(&chars_to_string!(body_chrs)) {
                Ok(body) => { return Ok( Rule{head, body}); },
                Err(err) => { return Err(err); },
            }
        },
        None => {  // Must be a fact, no body.
            let fact: Unifiable;
            let s = chars_to_string!(chrs);
            match parse_complex(&s) {
                Ok(f) => { fact = f; },
                Err(err) => { return Err(err); },
            }
            return Ok(Rule{head: fact, body: Goal::Nil});
        },

    } // match index_of_neck(chrs)...

} // parse_rule

impl Rule {

    /// Creates a key (predicate name) for indexing into the
    /// [knowledge base](../knowledge_base/index.html).
    ///
    /// The name of a predicate consists of its functor and its arity,
    /// separated by a slash. For example, for the fact
    /// `loves(Chandler, Monica)`, the functor is `loves` and the arity
    /// is 2, therefore the name of the predicate is `loves/2`.
    ///
    /// # Arguments
    /// * `self`
    /// # Return
    /// * `key` - String
    /// # Usage
    /// ```
    /// use suiron::*;
    ///
    /// let query = parse_query("parse($In, $Out, $ErrIn, $ErrOut)");
    /// match query {
    ///     Ok(q) => { println!("{}", q.key()); },
    ///     Err(msg) => { println!("{}", msg); },
    /// }
    /// // Prints: parse/4
    /// ```
    pub fn key(&self) -> String { return self.head.key(); }

    /// Returns the head of this rule.
    ///
    /// # Arguments
    /// * `self`
    /// # Return
    /// * `head term` -
    /// ([SComplex](../unifiable/enum.Unifiable.html#variant.SComplex))
    /// ```
    /// use suiron::*;
    ///
    /// clear_id();
    /// let kb = test_kb();
    /// // Get grandfather rule.
    /// let rule = get_rule(&kb, "grandfather/2", 0);
    /// let head = rule.get_head();
    /// println!("{}", head); // Prints: grandfather($X_1, $Y_2)
    /// ```
    pub fn get_head(&self) -> Unifiable { return self.head.clone(); }

    /// Returns the body of this rule, which is a goal.
    ///
    /// # Arguments
    /// * `self`
    /// # Return
    /// * `body` - ([Goal](../goal/enum.Goal.html))
    /// # Usage
    /// ```
    /// use suiron::*;
    ///
    /// clear_id();
    /// let kb = test_kb();
    /// // Get grandfather rule.
    /// let rule = get_rule(&kb, "grandfather/2", 0);
    /// let body = rule.get_body();
    /// println!("{}", body);  // father($X_1, $Z_3), father($Z_3, $Y_2)
    /// ```
    pub fn get_body(&self) -> Goal { return self.body.clone(); }

    /// The scope of a logic variable is the rule or goal in which it is defined.
    ///
    /// When the inference algorithm tries to solve a goal, it calls this method
    /// to ensure that the variables are unique.
    ///
    /// # Argument
    /// * `self`
    /// * `recreated_vars` - logic variables already recreated
    /// # Return
    /// * `Rule`
    /// # Usage
    /// ```
    /// use suiron::*;
    ///
    /// clear_id();
    /// match parse_rule("parent($X, $Y) :- mother($X, $Y).") {
    ///     Ok(rule) => {
    ///         let mut var_map = VarMap::new();
    ///         let rule = rule.recreate_variables(&mut var_map);
    ///         println!("{}", rule);
    ///     },
    ///     Err(msg) => { println!("{}", msg); },
    /// }
    /// // Prints: parent($X_1, $Y_2) :- mother($X_1, $Y_2).
    /// ```
    pub fn recreate_variables(self, recreated_vars: &mut VarMap) -> Rule {
        let new_head = self.head.recreate_variables(recreated_vars);
        let new_body: Goal;
        match self.body {
            Goal::OperatorGoal(op) => {
                new_body = Goal::OperatorGoal(op.recreate_variables(recreated_vars));
            },
            Goal::ComplexGoal(comp) => {
                new_body = Goal::ComplexGoal(comp.recreate_variables(recreated_vars));
            },
            Goal::BuiltInGoal(bip) => {
                new_body = Goal::BuiltInGoal(bip.recreate_variables(recreated_vars));
            },
            Goal::Nil => { new_body = Goal::Nil; },
        }
        return Rule{ head: new_head, body: new_body };
    } // recreate_variables

}

// Creates an error message for parse_rule() function.
// Arguments:
//    err - error description
//    bad - string which caused the error
// Return:
//    error message (String)
fn pr_error(err: &str, bad: &str) -> String {
    format!("parse_rule() - {}: >{}<", err, bad)
}

// Display trait, to display facts and rules.
impl fmt::Display for Rule {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.body == Goal::Nil {
            // Display fact.
            write!(f, "{}.", self.head)
        }
        else {
            // Display rule.
            write!(f, "{} :- {}.", self.head, self.body)
        }
    } // fmt

} // fmt::Display
