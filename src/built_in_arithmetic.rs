//! Suiron's arithmetic functions: add, subtract, multiply, divide.
//!
//! The functions defined in this module support Suiron's built-in
//! arithmetic functions.<br>
//! They are called from
//! [unify_sfunction()](../built_in_functions/fn.unify_sfunction.html#)
//! in built_in_functions.rs.

use std::rc::Rc;
use super::substitution_set::*;
use super::unifiable::Unifiable;

/// Add arguments together.
///
/// If all the arguments are SIntegers, the function returns an SInteger.<br>
/// If there is at least 1 SFloat in the list of argument, the function
/// returns an SFloat.
///
/// This method is called by
/// [unify_sfunction()](../built_in_functions/fn.unify_sfunction.html#).
///
/// # Arguments
/// * `arguments` - list of [Unifiable](../unifiable/enum.Unifiable.html) terms
/// * `ss` - [SubstitutionSet](../substitution_set/index.html)
/// # Returns
/// * [SInteger](../unifiable/enum.Unifiable.html#variant.SInteger) or
///   [SFloat](../unifiable/enum.Unifiable.html#variant.SFloat)
/// # Panics
/// * If a logic variable in the list of terms is not grounded.
/// * If one of the ground terms is not an SInteger or SFloat.
/// # Usage
/// ```
/// use std::rc::Rc;
/// use suiron::*;
///
/// let ss = empty_ss!();
/// let arguments = parse_arguments("1, 2, 3").unwrap();
/// let result = evaluate_add(&arguments, &ss);
/// println!("{}", result);
/// // Prints: 6
/// ```
pub fn evaluate_add<'a>(arguments: &'a Vec<Unifiable>,
                    ss: &'a Rc<SubstitutionSet<'a>>) -> Unifiable {

    let (numbers, has_float) = get_numbers(arguments, ss);
    if has_float {
        let f = get_floats(&numbers);
        let sum = f.iter().fold(0.0, |mut sum, &x| {sum += x; sum});
        return Unifiable::SFloat(sum);
    }
    else {
        let i = get_integers(&numbers);
        let sum = i.iter().fold(0, |mut sum, &x| {sum += x; sum});
        return Unifiable::SInteger(sum);
    }
} // evaluate_add


/// Subtract arguments from the first argument in a list.
///
/// If all the arguments are SIntegers, the function returns an SInteger.<br>
/// If there is at least 1 SFloat in the list of argument, the function
/// returns an SFloat.
///
/// This method is called by
/// [unify_sfunction()](../built_in_functions/fn.unify_sfunction.html#).
///
/// # Arguments
/// * `arguments` - list of [Unifiable](../unifiable/enum.Unifiable.html) terms
/// * `ss` - [SubstitutionSet](../substitution_set/index.html)
/// # Returns
/// * [SInteger](../unifiable/enum.Unifiable.html#variant.SInteger) or
///   [SFloat](../unifiable/enum.Unifiable.html#variant.SFloat)
/// # Panics
/// * If a logic variable in the list of terms is not grounded.
/// * If one of the ground terms is not an SInteger or SFloat.
/// # Usage
/// ```
/// use std::rc::Rc;
/// use suiron::*;
///
/// let ss = empty_ss!();
/// let arguments = parse_arguments("5.2, 1, 2").unwrap();
/// let result = evaluate_subtract(&arguments, &ss);
/// println!("{}", result);
/// // Prints: 2.2
/// ```
pub fn evaluate_subtract<'a>(arguments: &'a Vec<Unifiable>,
                    ss: &'a Rc<SubstitutionSet<'a>>) -> Unifiable {

    let (numbers, has_float) = get_numbers(arguments, ss);
    if has_float {
        let mut f = get_floats(&numbers);
        let first = f.remove(0);
        let result = f.iter().fold(first, |mut result, &x| {result -= x; result});
        return Unifiable::SFloat(result);
    }
    else {
        let mut i = get_integers(&numbers);
        let first = i.remove(0);
        let result = i.iter().fold(first, |mut result, &x| {result -= x; result});
        return Unifiable::SInteger(result);
    }
} // evaluate_subtract


/// Multiply arguments together.
///
/// If all the arguments are SIntegers, the function returns an SInteger.<br>
/// If there is at least 1 SFloat in the list of argument, the function
/// returns an SFloat.
///
/// This method is called by
/// [unify_sfunction()](../built_in_functions/fn.unify_sfunction.html#).
///
/// # Arguments
/// * `arguments` - list of [Unifiable](../unifiable/enum.Unifiable.html) terms
/// * `ss` - [SubstitutionSet](../substitution_set/index.html)
/// # Returns
/// * [SInteger](../unifiable/enum.Unifiable.html#variant.SInteger) or
///   [SFloat](../unifiable/enum.Unifiable.html#variant.SFloat)
/// # Panics
/// * If a logic variable in the list of terms is not grounded.
/// * If one of the ground terms is not an SInteger or SFloat.
/// # Usage
/// ```
/// use std::rc::Rc;
/// use suiron::*;
///
/// let ss = empty_ss!();
/// let arguments = parse_arguments("3, 4, 0.25").unwrap();
/// let result = evaluate_multiply(&arguments, &ss);
/// println!("{}", result);
/// // Prints: 3
/// ```
///
pub fn evaluate_multiply<'a>(arguments: &'a Vec<Unifiable>,
                    ss: &'a Rc<SubstitutionSet<'a>>) -> Unifiable {

    let (numbers, has_float) = get_numbers(arguments, ss);
    if has_float {
        let f = get_floats(&numbers);
        let result = f.iter().fold(1.0, |mut result, &x| {result *= x; result});
        return Unifiable::SFloat(result);
    }
    else {
        let i = get_integers(&numbers);
        let result = i.iter().fold(1, |mut result, &x| {result *= x; result});
        return Unifiable::SInteger(result);
    }
} // evaluate_multiply


/// Divide the first argument by the following arguments.
///
/// If all the arguments are SIntegers, the function returns an SInteger.<br>
/// If there is at least 1 SFloat in the list of argument, the function
/// returns an SFloat.
///
/// If all the arguments are SIntegers, the function does an integer divide.<br>
/// That is, all remainders are discarded:<br>
/// <blockquote>
/// 7 / 3 => 2
/// </blockquote>
///
/// This method is called by
/// [unify_sfunction()](../built_in_functions/fn.unify_sfunction.html#).
///
/// # Arguments
/// * `arguments` - list of [Unifiable](../unifiable/enum.Unifiable.html) terms
/// * `ss` - [SubstitutionSet](../substitution_set/index.html)
/// # Returns
/// * [SInteger](../unifiable/enum.Unifiable.html#variant.SInteger) or
///   [SFloat](../unifiable/enum.Unifiable.html#variant.SFloat)
/// # Panics
/// * If a logic variable in the list of terms is not grounded.
/// * If one of the ground terms is not an SInteger or SFloat.
/// # Usage
/// ```
/// use std::rc::Rc;
/// use suiron::*;
///
/// // Floating point division.
/// let ss = empty_ss!();
/// let arguments = parse_arguments("12.6, 3, 3").unwrap();
/// let result = evaluate_divide(&arguments, &ss);
/// println!("{}", result);
/// // Prints: 1.4000000000000001
///
/// // Integer division.
/// let ss = empty_ss!();
/// let arguments = parse_arguments("13, 3, 3").unwrap();
/// let result = evaluate_divide(&arguments, &ss);
/// println!("{}", result);
/// // Prints: 1
/// ```
pub fn evaluate_divide<'a>(arguments: &'a Vec<Unifiable>,
                    ss: &'a Rc<SubstitutionSet<'a>>) -> Unifiable {

    let (numbers, has_float) = get_numbers(arguments, ss);
    if has_float {
        let mut f = get_floats(&numbers);
        let first = f.remove(0);
        let result = f.iter().fold(first, |mut result, &x| {result /= x; result});
        return Unifiable::SFloat(result);
    }
    else {
        let mut i = get_integers(&numbers);
        let first = i.remove(0);
        let result = i.iter().fold(first, |mut result, &x| {result /= x; result});
        return Unifiable::SInteger(result);
    }
} // evaluate_divide


/// In Suiron, a number can be an SInteger (i64) or an SFloat (f64).
#[derive(Debug)]
pub enum SNumber {
    SFloat(f64),
    SInteger(i64),
} // SNumeric

/// Gets the numbers (integers and floats) from a vector of unifiable terms.
///
/// This function gets the ground terms from the given list of unifiable terms,
/// and saves them in a vector of numbers (SNumber). It returns the vector of
/// numbers, and a flag called has_float, which indicates that at least one of
/// the numbers is a floating point number.
///
/// # Arguments
/// * `terms` - vector of Unifiable terms
/// * `ss` - SubstitutionSet
/// # Return
/// * (vector of SNumbers, has_float)
/// # Panics
/// * If a logic variable in the list of terms is not grounded.
/// * If one of the terms is not an SInteger or SFloat.
fn get_numbers<'a>(terms: &Vec<Unifiable>, ss: &'a Rc<SubstitutionSet<'a>>)
                   -> (Vec<SNumber>, bool) {

    let mut numbers: Vec<SNumber> = vec![];
    let mut has_float = false;

    // Get ground terms.
    for term in terms {
        match get_ground_term(term, ss) {
            Some(gt) => {
                match gt {
                    Unifiable::SInteger(i) => {
                        numbers.push(SNumber::SInteger(*i));
                    },
                    Unifiable::SFloat(f) => {
                        has_float = true;
                        numbers.push(SNumber::SFloat(*f));
                    },
                    _ => {
                        number_panic("Argument is not a number", term);
                    },
                } // match
            },
            None => {
                number_panic("Argument is not grounded", term);
            },
        } // match
    } // for
    return (numbers, has_float);
} // get_numbers()

/// Gets the integers (i64) from a list of numbers.
///
/// # Argument
/// * `numbers` - SNumber
/// # Return
/// * `vector of i64`
fn get_integers<'a>(numbers: &Vec<SNumber>) -> Vec<i64> {
    let mut ints: Vec<i64> = vec![];
    for n in numbers {
        if let SNumber::SInteger(i) = n { ints.push(*i); }
    }
    return ints;
} // get_integers()

/// Gets the floating point numbers (f64) from a list of numbers.
///
/// # Argument
/// * `numbers` - SNumber
/// # Return
/// * `vector of f64`
fn get_floats<'a>(numbers: &Vec<SNumber>) -> Vec<f64> {
    let mut floats: Vec<f64> = vec![];
    for n in numbers {
        if let SNumber::SFloat(f) = n { floats.push(*f); }
        else {
            // Convert int to float.
            if let SNumber::SInteger(i) = n { floats.push(*i as f64); }
        }
    }
    return floats;
}  // get_floats()

/// Creates an error message for get_numbers() and panics.
///
/// # Arguments
/// * err - error description
/// * term - term which caused the error
fn number_panic(err: &str, term: &Unifiable) {
    let msg = format!("get_numbers() - {}: {}", err, term);
    panic!("{}", msg);
}

#[cfg(test)]
mod test {

    use std::rc::Rc;
    use crate::*;
    use super::*;

    // Variables for testing.
    fn w() -> Unifiable { logic_var!(1, "$W") }
    fn x() -> Unifiable { logic_var!(2, "$X") }
    fn y() -> Unifiable { logic_var!(3, "$Y") }
    fn z() -> Unifiable { logic_var!(4, "$Z") }

    // Set up substitution set.
    fn get_ss<'a>() -> Rc<SubstitutionSet<'a>> {
        let three      = SInteger(3);
        let four       = SInteger(4);
        let five_seven = SFloat(5.7);
        let mut ss = empty_ss!();
        ss = x().unify(&three, &ss).unwrap();
        ss = y().unify(&four, &ss).unwrap();
        ss = z().unify(&five_seven, &ss).unwrap();
        ss
    } // get_ss()

    #[test]
    fn test_evaluate_add() {

        let ss = get_ss();

        let arguments = vec![SInteger(2), x(), y()];
        let result = evaluate_add(&arguments, &ss);
        let result = format!("{:?}", result);
        assert_eq!("SInteger(9)", result);

        let arguments = vec![SInteger(2), x(), z()];
        let result = evaluate_add(&arguments, &ss);
        let result = format!("{:?}", result);
        assert_eq!("SFloat(10.7)", result);

    } // test_evaluate_add()

    #[test]
    fn test_evaluate_subtract() {

        let ss = get_ss();

        let arguments = vec![x(), SInteger(10), y()];
        let result = evaluate_subtract(&arguments, &ss);
        let result = format!("{:?}", result);
        assert_eq!("SInteger(-11)", result);

        let arguments = vec![z(), SInteger(10), y()];
        let result = evaluate_subtract(&arguments, &ss);
        let result = format!("{:?}", result);
        assert_eq!("SFloat(-8.3)", result);

    } // test_evaluate_subtract()

    #[test]
    fn test_evaluate_multiply() {

        let ss = get_ss();

        let arguments = vec![x(), y(), SInteger(-3)];
        let result = evaluate_multiply(&arguments, &ss);
        let result = format!("{:?}", result);
        assert_eq!("SInteger(-36)", result);

        let arguments = vec![x(), y(), SFloat(-3.0)];
        let result = evaluate_multiply(&arguments, &ss);
        let result = format!("{:?}", result);
        assert_eq!("SFloat(-36.0)", result);

    } // test_evaluate_multiply()

    #[test]
    fn test_evaluate_divide() {

        let ss = get_ss();

        let arguments = vec![SInteger(12), x(), y()];
        let result = evaluate_divide(&arguments, &ss);
        let result = format!("{:?}", result);
        assert_eq!("SInteger(1)", result);

        // Integer division truncates.
        let arguments = vec![SInteger(13), x(), y()];
        let result = evaluate_divide(&arguments, &ss);
        let result = format!("{:?}", result);
        assert_eq!("SInteger(1)", result);

        let arguments = vec![SFloat(12.0), x(), y()];
        let result = evaluate_divide(&arguments, &ss);
        let result = format!("{:?}", result);
        assert_eq!("SFloat(1.0)", result);

        let arguments = vec![SFloat(13.0), x(), y()];
        let result = evaluate_divide(&arguments, &ss);
        let result = format!("{:?}", result);
        assert_eq!("SFloat(1.0833333333333333)", result);

        // Divide by zero.
        let arguments = vec![SFloat(13.0), SInteger(0), y()];
        let result = evaluate_divide(&arguments, &ss);
        let result = format!("{:?}", result);
        assert_eq!("SFloat(inf)", result);

    } // test_evaluate_divide()

    // Test with ungrounded variable in argument list.
    #[test]
    #[should_panic]
    fn test_evaluate_ungrounded_panic() {
        let ss = get_ss();
        let arguments = vec![SInteger(12), x(), w()];
        evaluate_add(&arguments, &ss);
    }

    // Test with non-number in argument list.
    #[test]
    #[should_panic]
    fn test_evaluate_nonnumber_panic() {
        let ss = get_ss();
        let arguments = vec![SInteger(12), x(), atom!("Oh no.")];
        evaluate_add(&arguments, &ss);
    }

} // test
