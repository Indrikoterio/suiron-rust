//! Functions to support logical And and Or operators.
//!
//! This module contains the functions next_solution_and() and next_solution_or(),
//! which are called by next_solution() in solution_node.rs.
//!
// Cleve Lendon 2023

use std::rc::Rc;
use std::cell::RefMut;
use std::cell::RefCell;

use crate::*;

use super::goal::Goal;
use super::substitution_set::*;

/// Calls next_solution() on all subgoals of the And operator.
///
/// All subgoals must succeed for the And solution node to succeed.
///
/// # Arguments
/// * `sn` -
/// [SolutionNode](../solution_node/struct.SolutionNode.html)
/// * `sn_ref` - reference to
/// [SolutionNode](../solution_node/struct.SolutionNode.html)
/// # Return
/// `Option` -
/// Some([SubstitutionSet](../substitution_set/type.SubstitutionSet.html))
/// or None
pub fn next_solution_and<'a>(sn: Rc<RefCell<SolutionNode<'a>>>,
                             sn_ref: RefMut<SolutionNode<'a>>)
                             -> Option<Rc<SubstitutionSet<'a>>> {

    // Check for the tail solution.
    match &sn_ref.tail_sn {
        None => {},
        Some(tail_sn) => {
            if let Some(ss) = next_solution(Rc::clone(&tail_sn)) {
                return Some(ss);
            }
        },
    }

    let mut solution: Option<Rc<SubstitutionSet>>;

    match &sn_ref.head_sn {
        None => { return None; },
        Some(head_sn) => { solution = next_solution(Rc::clone(&head_sn)); },
    }

    loop {

        match solution {
            None => { return None; },
            Some(sol) => {

                // print_ss(&sol); // For debugging.
                match &sn_ref.operator_tail {
                    None => { Some(sol); },
                    Some(tail) => {
                        if tail.len() == 0 { return Some(sol); }

                        // Tail solution node has to be an And solution node.
                        let tail_goal = Goal::OperatorGoal(tail.clone());
                        let tail_sn = tail_goal.get_sn(sn_ref.kb, sol, Rc::clone(&sn));
                        let tail_solution = next_solution(Rc::clone(&tail_sn));
                        match tail_solution {
                            None => {},
                            Some(tail_solution) => { return Some(tail_solution); },
                        }
                    },
                }
            },
        }
        match &sn_ref.head_sn {
            None => { return None; },
            Some(head_sn) => { solution = next_solution(Rc::clone(&head_sn))},
        }

    } // loop

} // next_solution_and()

/// Calls next_solution() on subgoals of the Or operator.
///
/// Checks subgoals until a success is found.
///
/// # Arguments
/// * `sn` -
/// [SolutionNode](../solution_node/struct.SolutionNode.html)
/// * `sn_ref` - reference to
/// [SolutionNode](../solution_node/struct.SolutionNode.html)
/// # Return
/// `Option` -
/// Some([SubstitutionSet](../substitution_set/type.SubstitutionSet.html))
/// or None
pub fn next_solution_or<'a>(sn: Rc<RefCell<SolutionNode<'a>>>,
                            sn_ref: RefMut<SolutionNode<'a>>)
                            -> Option<Rc<SubstitutionSet<'a>>> {

    // Check for the tail solution.
    match &sn_ref.tail_sn {
        None => {},
        Some(tail_sn) => {
            return next_solution(Rc::clone(&tail_sn));
        },
    }

    let solution: Option<Rc<SubstitutionSet>>;

    match &sn_ref.head_sn {
        None => { return None; },
        Some(head_sn) => { solution = next_solution(Rc::clone(&head_sn)); },
    }

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
            let parent_solution = Rc::clone(&sn_ref.parent_solution);
            let tail_sn = tail_goal.get_sn(sn_ref.kb, parent_solution, Rc::clone(&sn));
            return next_solution(Rc::clone(&tail_sn));
        },
    }

} // next_solution_or()
