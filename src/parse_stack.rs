//! LIFO stack for parsing. Used by the tokenizer.
//!
//! The parse stack holds [token types](../token/enum.TokenType.html).
//!
// Cleve Lendon 2023

use super::token::*;

pub type ParseStack = Vec<TokenType>;

/// Peeks at the top (last in item) of the parse stack.
/// # Arguments
/// * `stack` - ParseStack
/// # Return
/// * `token type`
pub fn peek(stack: &mut ParseStack) -> TokenType {
    let length = stack.len();
    if length == 0 { return TokenType::Empty; }
    return stack[length - 1];
}

/// Pops the top item from the stack.
/// # Arguments
/// * `stack` - ParseStack
/// # Return
/// * `token type`
pub fn pop(stack: &mut ParseStack) -> TokenType {
    let p = stack.pop();
    match p {
        Some(tt) => tt,
        None => TokenType::Empty,
    }
}
