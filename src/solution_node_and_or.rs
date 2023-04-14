//! Functions to support logical And and Or operators.
//!
//! This module contains the functions next_solution_and() and next_solution_or(),
//! which are called by next_solution() in solution_node.rs.
//!
// Cleve Lendon 2023

use std::rc::Rc;
use std::cell::RefCell;

use crate::*;

use super::goal::Goal;
use super::substitution_set::*;

/// Calls next_solution() on all subgoals of the And operator.
///
/// All subgoals must succeed for the And solution node to succeed.
///
/// # Arguments
/// * [SolutionNode](../solution_node/struct.SolutionNode.html)
/// * reference to [SolutionNode](../solution_node/struct.SolutionNode.html)
/// # Return
/// [SubstitutionSet](../substitution_set/type.SubstitutionSet.html) or None
pub fn next_solution_and<'a>(sn: Rc<RefCell<SolutionNode<'a>>>)
                             -> Option<Rc<SubstitutionSet<'a>>> {

    let mut sn_ref = sn.borrow_mut(); // Get a mutable reference.

    // Check for the tail solution.
    if let Some(tail_sn) = &sn_ref.tail_sn {
        if let Some(ss) = next_solution(Rc::clone(&tail_sn)) {
            return Some(ss);
        }
    }

    let mut solution = match &sn_ref.head_sn {
        None => { return None; },
        Some(head_sn) => { next_solution(Rc::clone(&head_sn)) },
    };

    loop {

        match solution {

            None => { return None; },
            Some(ss) => {

                // print_ss(&ss); // For debugging.
                match &sn_ref.operator_tail {
                    None => { return Some(ss); },
                    Some(tail) => {

                        if tail.len() == 0 { return Some(ss); }

                        // Tail solution node has to be an And solution node.
                        let tail_goal = Goal::OperatorGoal(tail.clone());
                        let tail_sn = make_solution_node(Rc::new(tail_goal),
                                                         sn_ref.kb, ss,
                                                         Rc::clone(&sn));
                        sn_ref.tail_sn = Some(Rc::clone(&tail_sn));
                        let tail_solution = next_solution(tail_sn);
                        if tail_solution.is_some() { return tail_solution; }
                    },
                } // match
            },
        } // match solution

        // Try another solution.
        solution = match &sn_ref.head_sn {
            None => { return None; },
            Some(head_sn) => { next_solution(Rc::clone(&head_sn)) },
        };

    } // loop

} // next_solution_and()

/// Calls next_solution() on subgoals of the Or operator.
///
/// Checks subgoals until a success is found.
///
/// # Arguments
/// * [SolutionNode](../solution_node/struct.SolutionNode.html)
/// * reference to [SolutionNode](../solution_node/struct.SolutionNode.html)
/// # Return
/// [SubstitutionSet](../substitution_set/type.SubstitutionSet.html) or None
pub fn next_solution_or<'a>(sn: Rc<RefCell<SolutionNode<'a>>>)
                            -> Option<Rc<SubstitutionSet<'a>>> {

    let mut sn_ref = sn.borrow_mut(); // Get a mutable reference.

    // Check for the tail solution.
    if let Some(tail_sn) = &sn_ref.tail_sn {
        return next_solution(Rc::clone(&tail_sn));
    }

    let solution = match &sn_ref.head_sn {
        None => { return None; },
        Some(head_sn) => { next_solution(Rc::clone(&head_sn)) },
    };

    match solution {
        None => {
            match &sn_ref.operator_tail {
                None => { return solution; },
                Some(tail) => {
                    if tail.len() == 0 { return solution; }
                },
            }
        },
        Some(_) => { return solution; },
    }

    match &sn_ref.operator_tail {
        None => { return None; },
        Some(tail) => {
            let tail_goal = Goal::OperatorGoal(tail.clone());
            let ss = Rc::clone(&sn_ref.ss);
            let tail_sn = make_solution_node(Rc::new(tail_goal),
                                             sn_ref.kb, ss,
                                             Rc::clone(&sn));
            sn_ref.tail_sn = Some(Rc::clone(&tail_sn));
            return next_solution(tail_sn);
        },
    }

} // next_solution_or()
