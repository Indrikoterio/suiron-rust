//! Unifiable terms (atoms, numbers, logic variables, complex terms, etc.).
//!
//! * Two terms can unify if they are identical, or if one of them is
//! an unbound logic variable.
//! * Some names of terms have an S prefix, to distinguish them from
//! reserved words in implementing languages.
//! * A Unifiable term 'owns' its data.
// Cleve Lendon 2023

use std::fmt;
use std::rc::Rc;

use super::goal::Goal;
use super::logic_var::*;
use super::s_linked_list::*;
use super::built_in_functions::*;
use super::built_in_predicates::*;
use super::substitution_set::*;

static NOT_A_NODE_ERR: &str = "unify(): Not an SLinkedList node.";
static UNKNOWN_UNIFIABLE_ERR: &str = "unifiable.rs - Unknown unifiable.";
static VAR_ID_0_ERR: &str =
       "unify(): Logic variable has an ID of 0. See: recreate_variables().";

#[derive(Debug, Clone, PartialEq)]
pub enum Unifiable {
    /// In a [linked list](../s_linked_list/index.html),
    /// Nil indicates that a node has no next node (i.e. is last).
    Nil,
    /// The anonymous variable, $_, unifies with anything.
    /// Use [anon!](../macro.anon.html) to create.
    Anonymous,
    /// A string constant. Use [atom!](../macro.atom.html) to construct.
    Atom(String),
    /// 64-bit floating point number.
    SFloat(f64),
    /// 64-bit integer.
    SInteger(i64),
    /// Logic variables have an ID number, which is used as an index
    /// into a [substitution set](../substitution_set/index.html).<br>
    /// A logic variable name should start with a dollar sign,
    /// followed by a letter. For example: $X.<br>
    /// Use [logic_var!](../macro.logic_var.html) to construct.
    LogicVar{id: usize, name: String},
    /// Complex (or compound) term. Eg. symptom(flu, $Symp).<br>
    /// Implemented as a vector of unifiable terms.
    /// Use [scomplex!](../macro.scomplex.html) to construct.<br>
    SComplex(Vec<Unifiable>),
    /// Represents a node in a linked list of unifiable terms.<br>
    ///
    /// `term` - Holds a value (any Unifiable term).<br>
    /// `next` - Points to next node, must be `SLinkedList` or `Nil`.<br>
    /// `count` - The number of nodes in the list.<br>
    /// `tail_var` - Boolean . True indicates that the last term is a tail variable.<br>
    ///
    /// Examples of lists: [a, b, c, d], [$Head | $Tail]<br>
    /// Use [slist!](../macro.slist.html) to construct.
    SLinkedList{
        term: Box<Unifiable>,   // The node's item.
        next: Box<Unifiable>,   // Link to another SLinkedList or Nil.
        count: usize,
        tail_var: bool,         // tail variable flag
    },
    /// This variant defines built-in functions, such as add(), join(), etc.<br>
    /// Built-in functions produce a unifiable term from a list of arguments.
    SFunction{name: String, terms: Vec<Unifiable>},
}

impl Unifiable {

    /// Creates a key (= predicate name) for indexing into a
    /// [knowledge base](../knowledge_base/index.html).
    ///
    /// The name of a predicate consists of its functor plus its arity,
    /// separated by a slash. For example, for the fact `loves(Chandler, Monica)`,
    /// the functor is `loves` and the arity is 2, therefore the name of the
    /// predicate is `loves/2`.
    ///
    /// # Arguments
    /// * `self` - [SComplex](../unifiable/enum.Unifiable.html#variant.SComplex)
    /// # Return
    /// * `key` - String
    /// # Panics
    /// * If self is not a
    /// [complex](../unifiable/enum.Unifiable.html#variant.SComplex) term.
    /// # Usage
    /// ```
    /// use suiron::*;
    ///
    /// let cmplx = parse_complex("loves(Chandler, Monica)").unwrap();
    /// let key = cmplx.key();
    /// println!("{}", key);  // Should print: loves/2
    /// ```
    pub fn key(&self) -> String {
        match self {
            Unifiable::SComplex(terms) => {
                let functor = &terms[0];
                let arity = terms.len() - 1;
                return format!("{}/{}", functor, arity);
            },
            _ => { panic!("key() - Argument must be SComplex."); }
        }
    } // key()

    /// Tries to unify two terms.
    ///
    /// Two terms can be unified if they are identical, if one of the
    /// terms is an unbound variable, or if one of the terms is a bound
    /// variable whose ground term matches the other term. For example:
    ///
    /// 1: `verb = verb` &nbsp; &nbsp; Terms are identical. Unification succeeds.<br>
    /// 2: `$X = verb` &nbsp; &nbsp; If $X is unbound, unification succeeds.<br>
    /// 3: `$Y = verb` &nbsp; &nbsp; If $Y was previously bound to `verb`,
    /// unification succeeds.<br>
    /// 4: `$Z = verb` &nbsp; &nbsp; If $Z was previously bound to `pronoun`,
    /// unification fails.<br>
    ///
    /// In examples 1, 2 and 3 above, unification succeeds, so the method will
    /// return a substitution set. For example 4, the method will return None.
    ///
    /// In examples 1 and 3, the substitution set is returned unchanged.
    ///
    /// In example 2, a new binding must be registered. This method will create
    /// a new substitution set, copy in the previous bindings, add the new
    /// binding ($X to verb), and return the new substitution set.
    ///
    /// # Arguments
    /// * `self`  - a unifiable term
    /// * `other` - the other unifiable term
    /// * `ss`    - [SubstitutionSet](../substitution_set/type.SubstitutionSet.html)
    /// # Returns
    /// * `Option` -
    /// Some([SubstitutionSet](../substitution_set/type.SubstitutionSet.html))
    /// or None.
    /// # Usage
    /// ```
    /// use std::rc::Rc;
    /// use suiron::*;
    ///
    /// let x = logic_var!(next_id(), "$X");
    /// let age = SInteger(37);
    /// let ss = empty_ss!();
    /// match x.unify(&age, &ss) {
    ///     Some(ss) => { print_ss(&ss); },
    ///     None => { println!("Cannot unify."); },
    /// }
    /// ```
    ///
    /// The above should print:
    ///
    /// <pre>
    /// ----- Substitution Set -----
    /// 0 &nbsp; &nbsp; &nbsp; Nil
    /// 1 &nbsp; &nbsp; &nbsp; 37
    /// ----------------------------
    /// </pre>
    ///
    pub fn unify<'a>(&'a self, other: &'a Unifiable,
                     ss: &'a Rc<SubstitutionSet<'a>>)
                     -> Option<Rc<SubstitutionSet<'a>>> {

        if self == other {  // could happen
            return Some(Rc::clone(ss));
        }

        // Anonymous variable $_ unifies with everything.
        if Unifiable::Anonymous == *other { Some(Rc::clone(ss)); }

        match self {

            // $_ unifies with everything.
            Unifiable::Anonymous => { Some(Rc::clone(ss)) },
            Unifiable::Atom(self_str) => {

                match other {
                    Unifiable::Atom(other_str) => {
                        if self_str.eq(other_str) { return Some(Rc::clone(ss)); }
                        None
                    },
                    Unifiable::LogicVar{id: _, name: _} => { other.unify(&self, ss) },
                    Unifiable::Anonymous => { return Some(Rc::clone(ss)); },
                    _ => None,
                }
            },
            Unifiable::SFloat(self_float) => {
                match other {
                    Unifiable::SFloat(other_float) => {
                        if self_float == other_float { return Some(Rc::clone(ss)); }
                        None
                    },
                    Unifiable::LogicVar{id: _, name: _} => { other.unify(&self, ss) },
                    Unifiable::Anonymous => { return Some(Rc::clone(ss)); },
                    _ => None,
                }
            },
            Unifiable::SInteger(self_int) => {
                match other {
                    Unifiable::SInteger(other_int) => {
                        if self_int == other_int { return Some(Rc::clone(ss)); }
                        None
                    },
                    Unifiable::LogicVar{id: _, name: _} => { other.unify(&self, ss) },
                    Unifiable::Anonymous => { return Some(Rc::clone(ss)); },
                    _ => None,
                }
            },
            Unifiable::LogicVar{id, name: _} => {

                let id = *id;

                // The function make_logic_var() creates variables with an ID of 0.
                // This is OK for rules stored in the knowledge base, because when
                // a rule is fetched from the kb, the variables are recreated with
                // new unique IDs.

                // If a variable has an ID of 0 here, something has gone wrong.
                // The following statement prevents endless loops, which occur when
                // a substitution set has a variable with ID = 0 at location 0.
                if id == 0 { panic!("{}", VAR_ID_0_ERR); }

                // The unify method of a function evaluates the function.
                // If the other term is a function, call its unify method.
                if let Unifiable::SFunction{name: _, terms: _} = other {
                    return other.unify(self, ss);
                }

                let length_src = ss.len();

                // If variable is bound.
                if id < length_src && ss[id] != None {
                    if let Some(term) = &ss[id] {
                        let u = &*term;
                        return u.unify(other, ss);
                    }
                }

                let mut length_dst = length_src;
                if id >= length_dst { length_dst = id + 1 }

                let mut new_ss: Vec<Option<Rc<Unifiable>>> = vec![None; length_dst];

                for (i, item) in ss.iter().enumerate() {
                    if let Some(item) = item {
                        new_ss[i] = Some(Rc::clone(&item));
                    }
                }

                new_ss[id] = Some(Rc::new(other.clone()));
                return Some(Rc::new(new_ss));

            },
            Unifiable::SComplex(self_terms) => {

                match other {
                    Unifiable::SComplex(other_terms) => {
                        let other_len = other_terms.len();
                        if self_terms.len() != other_len { return None; }

                        let mut new_ss = ss;  // borrowed Rc
                        let mut ss2: Rc<SubstitutionSet<'a>>;

                        // Unify all terms.
                        let mut i = 0;
                        while i < other_len {
                            let left  = &self_terms[i];
                            let right = &other_terms[i];
                            if *left  == Unifiable::Anonymous {
                                i += 1;
                                continue;
                            }
                            if *right == Unifiable::Anonymous {
                                i += 1;
                                continue;
                            }
                            if let Some(ss) = left.unify(&right, new_ss) {
                                ss2 = ss;
                            }
                            else { return None; }
                            new_ss = &ss2;
                            i += 1;
                        } // while
                        return Some(Rc::clone(new_ss));
                    },
                    Unifiable::LogicVar{id: _, name: _} => {
                        return other.unify(self, ss);
                    },
                    Unifiable::Anonymous => { return Some(Rc::clone(ss)); },
                    _ => None,
                }
            },
            Unifiable::SLinkedList{term: _, next: _, count: _, tail_var: _} => {

                match other {
                    Unifiable::SLinkedList{term: _, next: _, count: _, tail_var: _} => {

                        let mut new_ss = ss;
                        let mut ss2: Rc<SubstitutionSet<'a>>;

                        let mut this_list = self;
                        let mut other_list = other;

                        while *this_list  != Unifiable::Nil &&
                              *other_list != Unifiable::Nil {

                            if let Unifiable::SLinkedList {
                                         term: this_term,
                                         next: this_next, count: _,
                                         tail_var: this_tail_var} = this_list {
                                if let Unifiable::SLinkedList {
                                             term: other_term,
                                             next: other_next, count: _,
                                             tail_var: other_tail_var} = other_list {
                                    if *this_tail_var && *other_tail_var {
                                         if **other_term == Unifiable::Anonymous {
                                             return Some(Rc::clone(new_ss));
                                         }
                                         if **this_term  == Unifiable::Anonymous {
                                             return Some(Rc::clone(new_ss));
                                         }
                                         return this_term.unify(&other_term, new_ss);
                                    }
                                    else if *this_tail_var {
                                         return this_term.unify(&other_list, new_ss);
                                    }
                                    else if *other_tail_var {
                                         return other_term.unify(&this_list, new_ss);
                                    }
                                    else {
                                        if **this_term  == Unifiable::Nil &&
                                           **other_term == Unifiable::Nil {
                                            return Some(Rc::clone(new_ss));
                                        }
                                        if *this_list  == Unifiable::Nil ||
                                           *other_list == Unifiable::Nil {
                                            return None;
                                        }
                                        let ss = Rc::clone(&new_ss);
                                        if let Some(ss) =
                                               this_term.unify(&other_term, &ss) {
                                            ss2 = ss;
                                        }
                                        else { return None; }
                                    }
                                    this_list  = this_next;
                                    other_list = other_next;
                                }
                                else { panic!("{}", NOT_A_NODE_ERR); }
                            }
                            else { panic!("{}", NOT_A_NODE_ERR); }

                            new_ss = &ss2;

                        } // while not Nil
                        return None;

                    }, // SLinkedList
                    Unifiable::LogicVar{id: _, name: _} => {
                        return other.unify(self, ss);
                    },
                    Unifiable::Anonymous => { return Some(Rc::clone(ss)); },
                    _ => None,

                } // match other

            },  // SLinkedList
            Unifiable::SFunction{name, terms} => {
                return unify_sfunction(name, terms, other, ss);
            },
            _ => None,

        } // match self

    }  // unify()


    /// Recreates logic variables to give them unique IDs.
    ///
    /// The scope of a logic variable is the rule in which it is defined.
    /// For example, the two rules below both have a variable named $X,
    /// but the $X in father is different from the $X in mother.
    /// <blockquote>
    /// father($X, $Y) :- parent($X, $Y), male($X).<br>
    /// mother($X, $Y) :- parent($X, $Y), female($X).
    /// </blockquote>
    ///
    /// For rules stored in the [knowledge base](../knowledge_base/index.html),
    /// the ID of all variables is 0, the default. When a rule is fetched from
    /// the knowledge base, its variables must be replaced (recreated) with
    /// variables which have unique IDs.
    ///
    /// A variable may appear more than once in a rule. In the rule father/2, above,
    /// the variable $X appears 3 times, but each instance must have the same ID.
    /// The map recreated_vars holds the IDs of previously recreated variables.
    /// Thus, the first appearance of $X is given an ID number, and each subsequent
    /// occurrence of $X is given the same ID.
    ///
    /// # Arguments
    /// * `self`
    /// * `recreated_vars` - set of previously recreated variable IDs
    /// # Return
    /// * `new term`
    /// # Usage
    /// ```
    /// use suiron::*;
    ///
    /// let s = "mother($X, $Y) :- parent($X, $Y), female($X).";
    /// let rule = parse_rule(s).unwrap();
    /// // IDs of $X and $Y are 0.
    /// let recreated_rule = rule.recreate_variables(&mut VarMap::new());
    /// println!("{}", recreated_rule);
    /// // Should print:
    /// // mother($X_1, $Y_2) :- parent($X_1, $Y_2), female($X_1).
    /// ```
    pub fn recreate_variables(self, recreated_vars: &mut VarMap) -> Unifiable {

        match self {

            Unifiable::LogicVar{id: _, name} => {
                if let Some(id) = recreated_vars.get(&name) {
                    Unifiable::LogicVar{id: *id, name: name}
                }
                else {
                    let next_id = next_id();
                    recreated_vars.insert(name.clone(), next_id);
                    Unifiable::LogicVar{id: next_id, name: name}
                }
            },
            Unifiable::SComplex(terms) => {
                let mut new_terms = vec![];
                for term in terms {
                    let term = term.recreate_variables(recreated_vars);
                    new_terms.push(term);
                }
                Unifiable::SComplex(new_terms)
            },
            Unifiable::SLinkedList{term: _, next: _, count: _, tail_var: _} => {
                let mut this_list = self;
                let mut new_terms = vec![];
                let mut vbar = false;  // vertical bar |
                while let Unifiable::SLinkedList{term: t, next: n,
                                     count: c, tail_var: tf} = this_list {
                    new_terms.push(t.recreate_variables(recreated_vars));
                    if c == 1 && tf { vbar = true; }
                    this_list = *n;
                    if this_list == Unifiable::Nil { break; }
                }
                return make_linked_list(vbar, new_terms);
            },
            Unifiable::SFunction{name, terms} => {
                let mut new_terms: Vec<Unifiable> = vec![];
                for term in terms {
                    let term = term.recreate_variables(recreated_vars);
                    new_terms.push(term);
                }
                return Unifiable::SFunction{name, terms: new_terms};
            },
            _ => self,

        } // match

    } // recreate_variables()

    /// Replaces logic variables in an expression with the constants
    /// (atoms and numbers) which they are ultimately bound to, in order
    /// to display solutions.
    ///
    /// # Arguments
    /// * `self`
    /// * `ss` - [SubstitutionSet](../substitution_set/type.SubstitutionSet.html)
    /// # Return
    /// * `new term`
    /// # Usage
    /// ```
    /// use std::rc::Rc;
    /// use suiron::*;
    ///
    /// let loves = atom!("loves");
    /// let alf   = atom!("Alfalfa");
    /// let x = logic_var!(20, "$X");
    /// // Make query: loves(Alfalfa, $X)
    /// let c = scomplex!(loves, alf, x.clone());
    ///
    /// let mut ss = empty_ss!();
    /// let dar = atom!("Darla");
    /// ss = x.unify(&dar, &ss).unwrap();
    /// let c2 = c.replace_variables(&ss);
    /// println!("{}", c2);  // Prints: loves(Alfalfa, Darla)
    /// ```
    pub fn replace_variables(&self, ss: &SubstitutionSet) -> Unifiable {

        match self {
            Unifiable::Nil => { Unifiable::Nil },
            Unifiable::Anonymous => { Unifiable::Anonymous },
            Unifiable::Atom(s) => { Unifiable::Atom(s.to_string()) },
            Unifiable::SFloat(f) => { Unifiable::SFloat(*f) },
            Unifiable::SInteger(i) => { Unifiable::SInteger(*i) },
            Unifiable::LogicVar{id, name} => {
                let ss_length = ss.len();
                // If variable is bound.
                if *id < ss_length && ss[*id] != None {
                    if let Some(term) = &ss[*id] {
                        let u = &*term;
                        return u.replace_variables(ss);
                    }
                };
                return Unifiable::LogicVar{id: *id, name: name.to_string()};
            },
            Unifiable::SComplex(terms) => {
                let mut new_terms: Vec<Unifiable> = vec![];
                for term in terms {
                    let new_term = term.replace_variables(ss);
                    new_terms.push(new_term);
                }
                Unifiable::SComplex(new_terms)
            },
            Unifiable::SLinkedList{term: t, next: n, count: c, tail_var: tf} => {
                let t2 = t.replace_variables(ss);
                let n2 = n.replace_variables(ss);
                Unifiable::SLinkedList{term: Box::new(t2),
                                       next: Box::new(n2),
                                       count: *c,
                                       tail_var: *tf}
            },
            _ => { panic!("{}", UNKNOWN_UNIFIABLE_ERR); }
        }

    } // replace_variables()

} // impl Unifiable

/// Recreate logic variables in a vector of goals.
///
/// # Arguments
/// * goals - vector of goals
/// * vars - previously recreated logic variables
/// # Return
/// * recreated goals
pub fn recreate_vars_goals(goals: Vec<Goal>, vars: &mut VarMap) -> Vec<Goal> {
    let mut new_goals: Vec<Goal> = vec![];
    for g in goals { new_goals.push(g.recreate_variables(vars)); }
    return new_goals;
} // recreate_vars_goals()

/// Recreate logic variables in a vector of unifiable terms.
///
/// # Arguments
/// * terms
/// * vars - previously recreated logic variables
/// # Return
/// * recreated terms
pub fn recreate_vars_terms(terms: Vec<Unifiable>, vars: &mut VarMap) -> Vec<Unifiable> {
    let mut new_terms: Vec<Unifiable> = vec![];
    for t in terms { new_terms.push(t.recreate_variables(vars)); }
    return new_terms;
} // recreate_vars_terms()

// Display trait, to display unifiable terms.
impl fmt::Display for Unifiable {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Unifiable::Nil => { write!(f, "Nil") },
            Unifiable::Anonymous => { write!(f, "$_") },
            Unifiable::Atom(s) => { write!(f, "{}", s) },
            Unifiable::SFloat(fl) => { write!(f, "{}", fl) },
            Unifiable::SInteger(i) => { write!(f, "{}", i) },
            Unifiable::LogicVar{id, name} => {
                if *id == 0 { write!(f, "{}", name) }
                else { write!(f, "{}_{}", name, id) }
            },
            Unifiable::SComplex(args) => {
                let mut out = "".to_string();
                let mut functor = true;
                let mut comma = false;
                for arg in args {
                    if functor {
                        out = out + &arg.to_string() + "(";
                        functor = false;
                    }
                    else { // args
                        if comma { out += ", "; }
                        else { comma = true; }
                        out += &arg.to_string();
                    }
                }
                write!(f, "{})", out)
            },
            Unifiable::SLinkedList{ term, next, count: _, tail_var } => {
                let mut out = "[".to_string();
                let mut t = term;
                let mut n = next;
                let mut tf = tail_var;
                let mut first = true;
                while **t != Unifiable::Nil {
                    if first { first = false; }
                    else {
                        if *tf { out += " | "; }
                        else { out += ", "; }
                    }
                    out = out + &*t.to_string();
                    if let Unifiable::SLinkedList{term: t2, next: n2,
                                      count: _, tail_var: tf2} = &**n {
                        t = &t2;
                        n = &n2;
                        tf = &tf2;
                    }
                    else { break; }
                }
                write!(f, "{}]", out)
            },
            Unifiable::SFunction{name, terms} => {
                let out = format_built_in(name, terms);
                write!(f, "{}", out)
            },
        } // match
    } // fmt
} // fmt::Display


#[cfg(test)]
mod test {

    use std::rc::Rc;
    use crate::*;

    /// Tests that the Display trait prints unifiable terms correctly.
    #[test]
    fn test_display_unifiable() {

        let s = Nil.to_string();
        assert_eq!("Nil", s);
        let s = Anonymous.to_string();
        assert_eq!("$_", s);
        let s = atom!("Saltwater").to_string();
        assert_eq!("Saltwater", s);
        let s = SFloat(3.14159).to_string();
        assert_eq!("3.14159", s);
        let s = SInteger(67).to_string();
        assert_eq!("67", s);

        // Test formatting of logic variables.
        let s = logic_var!("$X").to_string();
        assert_eq!("$X", s);
        let s = logic_var!(10, "$X").to_string();
        assert_eq!("$X_10", s);

        // Test formatting of complex terms.
        let func = atom!("pronoun");
        let word = atom!("I");
        let case = atom!("subject");
        let person = atom!("first");
        let sing_plur = atom!("singular");
        let pronoun = scomplex!(func, word, case, person, sing_plur);
        let pr_str = "pronoun(I, subject, first, singular)";
        assert_eq!(pr_str, pronoun.to_string());

        // Test formatting of lists.
        let t1 = SInteger(1);
        let t2 = SInteger(2);
        let t3 = SInteger(3);
        let list1 = slist!(false, t1, t2, t3);
        let list1_str = list1.to_string();
        assert_eq!(list1_str, "[1, 2, 3]");

        // Test list with pipe.
        let t1 = SInteger(1);
        let t2 = SInteger(2);
        let t3 = SInteger(3);
        let x  = logic_var!(0, "$X");
        let list2 = slist!(true, t1, t2, t3, x);
        let list2_str = list2.to_string();
        assert_eq!(list2_str, "[1, 2, 3 | $X]");

    } // test_display_unifiable()


    /// It's OK for a variable in the knowledge base to have an ID of 0,
    /// but after a rule is fetched, the variable IDs must not be 0.
    /// Unify() should panic if it is called on a variable with an ID of 0.
    ///     a = $X
    #[test]
    #[should_panic]
    fn test_when_var_id_is_0() {
        let ss  = empty_ss!();
        let a  = atom!("a");
        let x  = logic_var!("$X");  // default ID is 0
        if let Some(_ss2) = x.unify(&a, &ss) {}
    }

    /// Terms should unify with themselves.
    ///     3.14159 = 3.14159
    #[test]
    fn test_unify_with_self() {
        let ss  = empty_ss!();
        let pi  = SFloat(3.14159);
        let pi2 = SFloat(3.14159);
        assert_ne!(None, pi.unify(&pi2, &ss));
    }

    /// Test unify() with unbound variable.
    ///     $X = f(a, b)
    #[test]
    fn test_unify_with_unbound_var() {
        let ss = empty_ss!();
        let a = atom!("a");
        let b = atom!("b");
        let f = atom!("f");
        let c = scomplex!(f, a, b);
        let x  = logic_var!(1, "$X");
        assert_ne!(None, c.unify(&x, &ss));
    }

    /// Test unify() with bound variables.
    ///     $Y = a, $X = $Y, $X = a
    #[test]
    fn test_unify_with_bound_vars() {

        let ss = empty_ss!();
        let a = atom!("a");
        let x  = logic_var!(1, "$X");
        let y  = logic_var!(2, "$Y");

        if let Some(ss) = y.unify(&a, &ss) {
            if let Some(ss) = x.unify(&y, &ss) {
                assert_ne!(None, a.unify(&x, &ss),
                          "Failed to unify: a with $X -> $Y -> a");
            }
            else { panic!("Failed to unify: $X = $Y"); }
        }
        else { panic!("Failed to unify: $Y = a"); }
    } // test_unify_with_bound_vars()

    /// Test recreate_variables().
    /// This test creates a few variables ($W, $X, $Y, $Z), then calls
    /// recreate_variables() on a complex term and a list which contain
    /// these variables.
    /// When first created, the variables all have an ID of 0.
    /// Calling the function will give them unique IDs, (1, 2, 3, 4).
    #[test]
    fn test_recreate_variables() {

        clear_id();

        let a = atom!("a");
        let b = atom!("b");
        let c = atom!("c");
        let d = atom!("d");
        let w = logic_var!("$W");
        let x = logic_var!("$X");
        let y = logic_var!("$Y");
        let z = logic_var!("$Z");
        let func = atom!("func");

        let complex1 = scomplex!(func, a, b, w, x);
        let list1 = slist!(true, c, d, y, z);

        let mut recreated = VarMap::new();
        let complex2 = complex1.recreate_variables(&mut recreated);
        let list2 = list1.recreate_variables(&mut recreated);

        assert_eq!("func(a, b, $W_1, $X_2)", complex2.to_string());
        assert_eq!("[c, d, $Y_3 | $Z_4]", list2.to_string());

    } // test_recreate_variables()

    /// Test replace_variables().
    ///
    /// First the test creates two complex terms. Complex1 contains constants
    /// (atoms) and complex2 contains logic variables. Complex2 is unified
    /// with complex1, which creates entries in the substitution set.
    /// (The variables in complex2 are bound to the constants in complex1.)
    /// Finally, replace_variables() is called on complex2. This function uses
    /// the substitution set to replace the variables in complex2 with the
    /// constants which they are bound to. The result of replace_variables()
    /// will look exactly like complex1: func(One, Two, Three).
    ///
    /// A similar test is done for lists. List1 contains constants, and list2
    /// contains variables. Also, list2 contains a tail variable: [$X, $Y | $Z].
    /// List2 is unified with list1, then replace_variables() is called on list2.
    /// The resulting list shows that the tail variable $Z is unified with two
    /// constants from list1: [Four, Five | [6, 7.1]].
    #[test]
    fn test_replace_variables() {

        let a = atom!("One");
        let b = atom!("Two");
        let c = atom!("Three");
        let x = logic_var!(1, "$X");
        let y = logic_var!(2, "$Y");
        let z = logic_var!(3, "$Z");
        let func  = atom!("func");
        let func2 = atom!("func");

        let ss = empty_ss!();

        let complex1 = scomplex!(func, a, b, c);
        let complex2 = scomplex!(func2, x, y, z);
        let ss = complex2.unify(&complex1, &ss).unwrap();

        let res = complex2.replace_variables(&*ss);
        assert_eq!("func(One, Two, Three)", res.to_string());

        let d = atom!("Four");
        let e = atom!("Five");
        let f = SInteger(6);
        let g = SFloat(7.1);
        let list1 = slist!(false, d, e, f, g);

        let x = logic_var!(4, "$X");
        let y = logic_var!(5, "$Y");
        let z = logic_var!(6, "$Z");
        let list2 = slist!(true, x, y, z);

        let ss = list2.unify(&list1, &ss).unwrap();
        let res = list2.replace_variables(&ss);
        assert_eq!("[Four, Five | [6, 7.1]]", res.to_string());

    } // test_replace_variables()

    /// Test key() function.
    #[test]
    fn test_key() {

        // element(Yttrium, 39) -> element/2
        let el  = atom!("element");
        let y   = atom!("Yttrium");
        let num = SInteger(39);
        let c = scomplex!(el, y, num);
        let name = c.key();
        assert_eq!(name, "element/2");

        // measure -> measure/0
        let m = atom!("measure");
        let c = scomplex!(m);
        let name = c.key();
        assert_eq!(name, "measure/0");
    }

    /// Test key() with invalid argument.
    /// Argument is not a complex term.
    #[test]
    #[should_panic]
    fn test_key_panic_1() {
        let tb = atom!("Terbium");
        //let _name = tb.key();
        tb.key();
    }

} // test
