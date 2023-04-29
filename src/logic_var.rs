//! Functions to support logic variables and the global LOGIC_VAR_ID.
//!
//! Logic variables ([LogicVar](../unifiable/enum.Unifiable.html#variant.LogicVar))
//! consist of a name and an ID.
//!
//! The name should start with a dollar sign and a letter, for example: $H.
//!
//! Logic variables which are stored in rules can have an ID of 0, but when the
//! rule is fetched from the knowledge base, its variables must be recreated
//! to give them unique IDs.
//! 
//! See [recreate_variables()](../unifiable/enum.Unifiable.html#method.recreate_variables).
//!

use std::collections::HashMap;
use super::unifiable::*;

static mut LOGIC_VAR_ID: usize = 0; // Global ID number for logic variables.

/// Increment and return the logic variable ID.
/// # Return
/// * LOGIC_VAR_ID
pub fn next_id() -> usize {
    unsafe {
        LOGIC_VAR_ID += 1;
        LOGIC_VAR_ID
    }
}

/// Gets the logic variable ID.
/// # Return
/// * LOGIC_VAR_ID
pub fn get_var_id() -> usize {
    unsafe { LOGIC_VAR_ID }
}

/// Sets the logic variable ID to the given value.
/// # Arguments
/// * id
pub fn set_var_id(id: usize) {
    unsafe {
        LOGIC_VAR_ID = id;
    }
}

/// Sets the logic variable ID to zero.
pub fn clear_id() {
    unsafe { LOGIC_VAR_ID = 0; }
}

// VarMap defines a map which is used by recreate_variables(),
// to keep track of previously recreated variables.
// Key - variable name
// Value - ID of variable
pub type VarMap = HashMap<String, usize>;

/// Creates a logic variable from a string.
///
/// Unlike the macro logic_var!, this function validates its arguments.<br>
/// The variable name must begin with a dollar sign and a letter, eg. $X<br>
/// If the name is invalid, an error message is returned.<br>
/// This function creates new variables with an ID of 0. This is OK for defining
/// rules in the knowledge base, but when a rule is fetched from the knowledge
/// base, the logic variable must be recreated with a unique ID.
/// # Arguments
/// * name
/// # Return
/// * [logic variable](../unifiable/enum.Unifiable.html#variant.LogicVar)
/// or error message
pub fn make_logic_var<'a>(name: String) -> Result<Unifiable, String> {

    let trimmed = name.trim();
    let the_chars: Vec<_> = trimmed.chars().collect();

    if the_chars.len() < 2 {
        let err = mlv_error("Variable must start with $ and a letter", trimmed);
        return Err(err);
    }
    if the_chars[0] != '$' {
        let err = mlv_error("Variable must start with $", trimmed);
        return Err(err);
    }
    if !the_chars[1].is_alphabetic() {
        let err = mlv_error("Second character must be a letter", trimmed);
        return Err(err);
    }

    let name = trimmed.to_string();
    return Ok(Unifiable::LogicVar{ id: 0, name: name });

} // make_logic_var()

/// Formats an error message for make_logic_var().
///
/// # Arguments
/// * error description
/// * string which caused the error
/// # Return
/// * error message
fn mlv_error(err: &str, bad: &str) -> String {
    format!("make_logic_var() - {}: {}", err, bad)
}
