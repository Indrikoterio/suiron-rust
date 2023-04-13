//! Defines infixes for built-in predicates and functions.
//!
// Cleve Lendon 2023

use std::fmt;

//-----------Infixes-----------
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Infix {
    None,
    /// = Unification operator.
    Unify,
    /// == Equal. No unification. Simply compares.
    Equal,
    /// &gt;
    GreaterThan,
    /// &lt;
    LessThan,
    /// &gt;=
    GreaterThanOrEqual,
    /// &lt;=
    LessThanOrEqual,
    /// &plus;
    Plus,
    /// &minus;
    Minus,
    /// &#42;
    Multiply,
    /// &#47;
    Divide,
}

/// Determines whether a string contains an infix: >=, ==, etc.
///
/// This function returns the type and index of the
/// [infix](../parse_goals/enum.Infix.html).<br>
/// For example, `$X < 6` contains Infix::LessThan at index 3.
///
/// This function does not check for arithmetic infixes: `+ - * /`<br>
/// Arithmetic is done with built-in functions.
/// See [check_arithmetic_infix()](../parse_terms/fn.check_arithmetic_infix.html).
///
/// # Arguments
/// * vector of chars
/// # Return
/// * ([Infix](../parse_goals/enum.Infix.html), index)
///
/// # Notes
/// * An infix must be preceded and followed by a space. This is invalid: `$X<6`
/// * The function ignores characters between double quotes and parentheses.<br>
/// For example, for the the string of characters `" <= "` (double quotes included),<br>
/// the function will return (Infix::None, 0).
///
/// # Usage
/// ```
/// use suiron::*;
///
/// let chrs = str_to_chars!("$Age >= 22");
/// let (infix, index) = check_infix(&chrs);
/// println!("{infix}, {index}");  // Prints: >=, 5
/// ```
pub fn check_infix(chrs: &Vec<char>) -> (Infix, usize) {

    let length = chrs.len();
    let mut prev   = '#';  // not a space

    let mut i = 0;
    while i < length {

        // Skip past quoted text: ">>>>>"
        let c1 = chrs[i];
        if c1 == '"' {
            let mut j = i + 1;
            while j < length  {
                let c2 = chrs[j];
                if c2 == '"' {
                    i = j;
                    break;
                }
                j += 1;
            }
        }
        else if c1 == '(' {
            // Skip past text within parentheses: (...)
            let mut j = i + 1;
            while j < length {
                let c2 = chrs[j];
                if c2 == ')' {
                    i = j;
                    break;
                }
                j += 1;
            }
        }
        else {
            // Previous character must be space.
            if prev != ' ' {
                prev = c1;
                i += 1;
                continue;
            }
            // Bad:  $X =1
            // Good: $X = 1
            if i >= (length - 2) { return (Infix::None, 0); }
            if c1 == '<' {
                let c2 = chrs[i + 1];
                if c2 == '=' {
                    // Bad:  $X <=1
                    // Good: $X <= 1
                    if i >= length - 3 { return (Infix::None, 0); }
                    let c3 = chrs[i + 2];
                    if c3 == ' ' {
                        return (Infix::LessThanOrEqual, i);
                    }
                }
                else if c2 == ' ' {
                    return (Infix::LessThan, i);
                }
            }
            else if c1 == '>' {
                let c2 = chrs[i + 1];
                if c2 == '=' {
                    // Bad:  $X >=1
                    // Good: $X >= 1
                    if i >= length - 3 { return (Infix::None, 0); }
                    let c3 = chrs[i + 2];
                    if c3 == ' ' {
                        return (Infix::GreaterThanOrEqual, i);
                    }
                }
                else if c2 == ' ' {
                    return (Infix::GreaterThan, i);
                }
            }
            else if c1 == '=' {
                let c2 = chrs[i + 1];
                if c2 == '=' {
                    // Bad:  $X ==1
                    // Good: $X == 1
                    if i >= length - 3 { return (Infix::None, 0); }
                    let c3 = chrs[i + 2];
                    if c3 == ' ' {
                        return (Infix::Equal, i);
                    }
                }
                else if c2 == ' ' {
                    return (Infix::Unify, i);
                }
            }

        } // else

        prev = c1;
        i += 1;

    } // while

    return (Infix::None, 0);  // failed to find infix

} // check_infix

/// Determines whether a string contains an arithmetic infix: +, -, *, /
///
/// This function returns the type and index of the arithmetic infix.<br>
/// For example, <code>$X * 6</code> contains Infix::Multiply, at index 3.
///
/// # Arguments
/// * vector of chars
/// # Return
/// * ([Infix](../parse_goals/enum.Infix.html), index)
///
/// # Notes
/// * An infix must be preceded and followed by a space. This is invalid:  `$X*6`
/// * The function ignores characters between double quotes and parentheses.<br>
/// For example, for the the string of characters `" * "` (double quotes included),<br>
/// the function will return (Infix::None, 0).
///
pub fn check_arithmetic_infix(chrs: &Vec<char>) -> (Infix, usize) {

    let length = chrs.len();
    let mut prev   = '#';  // not a space

    let mut i = 0;
    while i < length {

        // Skip past quoted text: ">>>>>"
        let c1 = chrs[i];
        if c1 == '"' {
            let mut j = i + 1;
            while j < length  {
                let c2 = chrs[j];
                if c2 == '"' {
                    i = j; break;
                }
                j += 1;
            }
        }
        else if c1 == '(' {
            // Skip past text within parentheses: (...)
            let mut j = i + 1;
            while j < length {
                let c2 = chrs[j];
                if c2 == ')' {
                    i = j; break;
                }
                j += 1;
            }
        }
        else {

            // Previous character must be space.
            if prev != ' ' {
                prev = c1;
                i += 1;
                continue;
            }

            // Bad:  $X =1
            // Good: $X = 1
            if i >= (length - 2) { return (Infix::None, 0); }
            if c1 == '+' { if chrs[i + 1] == ' ' { return (Infix::Plus, i); } }
            else
            if c1 == '-' { if chrs[i + 1] == ' ' { return (Infix::Minus, i); } }
            else
            if c1 == '*' { if chrs[i + 1] == ' ' { return (Infix::Multiply, i); } }
            else
            if c1 == '/' { if chrs[i + 1] == ' ' { return (Infix::Divide, i); } }
        } // else

        prev = c1;
        i += 1;

    } // while

    return (Infix::None, 0);  // failed to find infix

} // check_arithmetic_infix

impl fmt::Display for Infix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match &self {
            Infix::None => write!(f, "None"),
            Infix::Unify => write!(f, "="),
            Infix::Equal => write!(f, "=="),
            Infix::GreaterThan => write!(f, ">"),
            Infix::LessThan => write!(f, "<"),
            Infix::GreaterThanOrEqual => write!(f, ">="),
            Infix::LessThanOrEqual => write!(f, "<="),
            Infix::Plus => write!(f, "+"),
            Infix::Minus => write!(f, "-"),
            Infix::Multiply => write!(f, "*"),
            Infix::Divide => write!(f, "/"),
        };
    } // fmt
} // fmt::Display
