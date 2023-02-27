//! Defines the unify predicate.
//!
//! `Unify` is represented in source code as an equal sign.
//! <blockquote>
//! $X = 7
//! </blockquote>
//!
//! If the left-hand side and the right-hand side of the equal sign can
//! be unified, the predicate succeeds. Otherwise the predicate fails.
//!
//! In the example above, if the variable $X is unbound, unification
//! succeeds. The binding, $X to the integer 7, will be registered in
//! a substitution set.
//!
//! If the variable $X is already bound, the inference engine will
//! check the binding of $X. If $X is bound to 7, unification succeeds,
//! because 7 = 7. If the variable $X is bound to anything other than
//! 7, the predicate will fail.
//!
//!
//! In Rust code, the unification predicate can be defined as follows:
//! <blockquote>
//! let left = logic_var!("$X");
//! let right = SInteger(7);
//! let uni_pred = Unify(left, right);
//! </blockquote>
//!
// Cleve Lendon 2023
