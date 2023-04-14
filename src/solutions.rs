//! Functions which search for and format solutions.

use std::env;
use std::rc::Rc;
use std::cell::RefCell;

use super::goal::Goal;
use super::time_out::*;
use super::solution_node::*;
use super::unifiable::Unifiable;

const S_TIMEOUT: u64 = 1000; // milliseconds
const NO_MORE: &str = "No more.";

/// Finds one solution for the given solution node.
///
/// # Arguments
/// * `sn` - reference to a [SolutionNode](../solution_node/struct.SolutionNode.html)
/// # Return
/// * `solution` - String
/// # Usage
/// ```
/// use std::rc::Rc;
/// use suiron::*;
///
/// let mut kb = test_kb();
/// let query = parse_query("loves($Who, $Whom)").unwrap();
/// let sn = make_base_node(Rc::new(query), &kb); // solution node
///
/// println!("{}", solve(Rc::clone(&sn)));
/// // Prints: $Who = Leonard, $Whom = Penny
/// println!("{}", solve(Rc::clone(&sn)));
/// // Prints: $Who = Penny, $Whom = Leonard
/// println!("{}", solve(Rc::clone(&sn)));
/// // Prints: No.
/// ```
pub fn solve<'a>(sn: Rc<RefCell<SolutionNode<'a>>>) -> String {

    let timer = start_query_timer(S_TIMEOUT);

    let solution = next_solution(Rc::clone(&sn));
    cancel_timer(timer);

    if query_stopped() {
        return format!("Query timed out after {} \
                        milliseconds.", S_TIMEOUT);
    }

    match solution {
        Some(ss) => {
            let query = sn.borrow().goal.clone();
            let result = query.replace_variables(&ss);
            return format_solution(&query, &result);
        },
        None => { return NO_MORE.to_string(); },
    } // match solution

} // solve()

/// Finds all solutions for the given query.
///
/// # Arguments
/// * `sn` - reference to a [SolutionNode](../solution_node/struct.SolutionNode.html)
/// # Return
/// * `solutions` - vector of Strings
/// # Usage
/// ```
/// use std::rc::Rc;
/// use suiron::*;
///
/// let mut kb = test_kb();
/// let query = parse_query("loves($Who, $Whom)").unwrap();
/// let sn = make_base_node(Rc::new(query), &kb); // solution node
///
/// let results = solve_all(Rc::clone(&sn));
/// for result in results { println!("{}", result); }
/// // Prints:
/// // $Who = Leonard, $Whom = Penny
/// // $Who = Penny, $Whom = Leonard
/// // No.
/// ```
pub fn solve_all<'a>(sn: Rc<RefCell<SolutionNode<'a>>>) -> Vec<String> {

    let mut results: Vec<String> = vec![];

    let query = sn.borrow().goal.clone();
    let timer = start_query_timer(S_TIMEOUT);

    loop {

        let solution = next_solution(Rc::clone(&sn));
        if query_stopped() { break; }

        match solution {
            Some(ss) => {
                let result = query.replace_variables(&ss);
                let s = format_solution(&query, &result);
                results.push(s);
            },
            None => { break; }
        } // match solution

    } // loop

    cancel_timer(timer);
    if query_stopped() {
        let s = format!("Query timed out after {} milliseconds.", S_TIMEOUT);
        results.push(s);
    }

    return results;

} // solve_all()

/// Formats the results of a query for display.
///
/// For example, if the query were `loves(Leonard, $Whom)`, and
/// the result were `loves(Leonard, Penny)`, then the function would
/// return: `$Whom = Penny`
///
/// This function iterates through the query's terms. For every
/// logic variable in the query, it prints the variable's name
/// and the corresponding term from the result.
///
/// # Arguments
/// * `query` - [Goal](../goal/enum.Goal.html)
/// * `result` - [Unifiable](../unifiable/enum.Unifiable.html) term
/// # Return
/// * `formatted solution` - String
/// # Usage
/// ```
/// use std::rc::Rc;
/// use suiron::*;
///
/// let kb = test_kb();
/// let query = parse_query("loves(Leonard, $Whom)").unwrap();
/// let q = Rc::new(query);
/// let sn = make_base_node(Rc::clone(&q), &kb); // solution node
///
/// if let Some(ss) = next_solution(Rc::clone(&sn)) {
///     let result = q.replace_variables(&ss);
///     println!("{}", format_solution(&q, &result));
/// }
/// // Prints: $Whom = Penny
/// ```
pub fn format_solution(query: &Goal, result: &Unifiable) -> String {
    let mut out = "".to_string();
    // Deconstruct the query.
    if let Goal::ComplexGoal(Unifiable::SComplex(q_terms)) = query {
        // Deconstruct the result.
        if let Unifiable::SComplex(r_terms) = result {
            let mut first = true;
            for i in 1..q_terms.len() {
                // Scan for logic variables.
                match &q_terms[i] {
                    Unifiable::LogicVar{id: _, name} => {
                        // Output logic variable name and result.
                        if first {
                            out += &format!("{} = {}", name, r_terms[i]);
                            first = false;
                        }
                        else {
                            out += &format!(", {} = {}", name, r_terms[i]);
                        }
                    },
                    _ => {},
                } // match
            } // for
        } // if
    } // if
    return out;
} // format_solution()


/// Gets the environment variable RUST_MIN_STACK.
///
/// If RUST_MIN_STACK is not set, returns 0.
///
/// # Return
/// * `stack size` - i32
pub fn get_stack_size() -> i32 {
    let rust_min_stack = match env::var("RUST_MIN_STACK") {
        Ok(val) => val,
        Err(_e) => "0".to_string(),
    };
    match rust_min_stack.parse() {
        Ok(i)  => { return i; },
        Err(_) => { return 0; },
    }
} // get_stack_size()


#[cfg(test)]
mod test {

    use std::rc::Rc;
    use serial_test::serial;

    use crate::*;
    use super::*;

    #[test]
    #[serial]
    fn test_format_solution() {

        let kb = test_kb();
        let query = parse_query("loves($Who, $Whom)").unwrap();
        let q = Rc::new(query.clone());
        let sn = make_base_node(q, &kb); // solution node

        let mut ss = next_solution(Rc::clone(&sn));

        match ss {
            Some(ss2) => {
                let result = query.replace_variables(&ss2);
                let result = format_solution(&query, &result);
                assert_eq!("$Who = Leonard, $Whom = Penny", result);
                ss = next_solution(Rc::clone(&sn));
            },
            None => {
                panic!("Missing Solution 1.");
            },
        }

        match ss {
            Some(ss2) => {
                let result = query.replace_variables(&ss2);
                let result = format_solution(&query, &result);
                assert_eq!("$Who = Penny, $Whom = Leonard", result);
                ss = next_solution(Rc::clone(&sn));
            },
            None => {
                panic!("Missing Solution 2.");
            },
        }

        match ss {
            Some(_) => { panic!("The solutions should be exhausted."); },
            None => {},
        }

    } // test_format_solution()

    #[test]
    #[serial]
    fn test_solve() {

        let kb = test_kb();
        let query = parse_query("loves(Leonard, $Whom)").unwrap();
        let sn = make_base_node(Rc::new(query), &kb); // solution node
        let solution = solve(Rc::clone(&sn));
        assert_eq!("$Whom = Penny", solution);

    } // test_solve()

    #[test]
    #[serial]
    fn test_solve_all() {

        clear_id();
        let kb = test_kb();
        let query = parse_query("loves($Who, $Whom)").unwrap();
        let sn = make_base_node(Rc::new(query), &kb); // solution node
        let results = solve_all(Rc::clone(&sn));

        assert_eq!(results.len(), 2, "There should be 2 solutions.");
        assert_eq!("$Who = Leonard, $Whom = Penny", results[0]);
        assert_eq!("$Who = Penny, $Whom = Leonard", results[1]);

    } // test_solve()

    // NOTE: The default stack for Rust is 2 MB.
    // The following test contains an endless loop, which will
    // overrun the stack before timeout, if the stack size is
    // not increased. See: RUST_MIN_STACK in test.
    // ALSO NOTE: None of this works.
    /*
    #[test]
    #[serial]
    fn test_solve_timeout() {

        let stack_size = get_stack_size();
        if stack_size < 8000000 { return; }

        let mut kb = KnowledgeBase::new();

        // Make an infinite loop rule.
        let rule = parse_rule("loop :- loop.").unwrap();
        add_rules!(&mut kb, rule);

        let query = parse_query("loop").unwrap();
        let sn = query.base_node(&kb); // solution node

        let actual = solve(Rc::clone(&sn));
        let expected = format!("Query timed out after {} milliseconds.", S_TIMEOUT);
        assert_eq!(expected, actual);

    } // test_solve_timeout()
    */

    // NOTE: The default stack for Rust is 2 MB.
    // The following test contains an endless loop, which will
    // overrun the stack before timeout, if the stack size is
    // not increased. See: RUST_MIN_STACK in test.
    // ALSO NOTE: None of this works.
    /*
    #[test]
    #[serial]
    fn test_solve_all_timeout() {

        let stack_size = get_stack_size();
        if stack_size < 8000000 { return; }

        let mut kb = KnowledgeBase::new();

        // Make an infinite loop rule.
        let rule = parse_rule("loop :- loop.").unwrap();
        add_rules!(&mut kb, rule);

        let query = parse_query("loop").unwrap();
        let sn = query.base_node(&kb); // solution node

        let actual = solve_all(Rc::clone(&sn));
        let expected = format!("Query timed out after {} milliseconds.", S_TIMEOUT);
        assert_eq!(expected, actual[0]);

    } // test_solve_all_timeout()
    */

} // test
