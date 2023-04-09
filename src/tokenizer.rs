//! Functions which tokenize text strings to a generate goals.
//!
// Cleve Lendon 2023

use crate::str_to_chars;
use crate::chars_to_string;

use super::goal::Goal;
use super::parse_goals::*;
use super::operator::Operator;
use super::token::{*, Token, TokenType};
use super::parse_stack::*;

/// letter_number_hyphen()
/// Determines whether the given character is a letter,
/// number or hyphen.
/// # Arguments
/// * `ch` - character
/// # Return
/// * `true or false`
fn letter_number_hyphen(ch: char) -> bool {
    if ch >= 'a' && ch <= 'z' { return true; }
    if ch >= 'A' && ch <= 'Z' { return true; }
    if ch >= '0' && ch <= '9' { return true; }
    if ch == '_' || ch == '-' ||
       ch == '\u{ad}' { return true; }
    if ch >= '\u{c0}' && ch < '\u{2c0}' { return true; }
    if ch >= '\u{380}' && ch < '\u{510}' { return true; }
    return false;
} // letter_number_hyphen()

/// invalid_between_terms()
/// Quote, hash and 'at' are invalid between terms.
/// # Arguments
/// * `ch` - character
/// # Return
/// * `true or false` - True means invalid.
fn invalid_between_terms(ch: char) -> bool {
    if ch == '"' { return true; }
    if ch == '#' { return true; }
    if ch == '@' { return true; }
    return false;
} // invalid_between_terms()

/// Generates a goal from a text string.
///
/// # Arguments
/// * `to_parse` - string slice
/// # Return
/// * `Result` -
/// Ok([Goal](../goal/index.html)) or Err(message)
/// # Usage
/// ```
/// use suiron::*;
///
/// let s = "can_swim($X), can_fly($X)";
/// match generate_goal(s) {
///     Ok(goal) => { println!("{}", goal); },
///     Err(err) => { println!("{}", err); },
/// }
/// ```
///
/// The above should print: `can_swim($X), can_fly($X")`
pub fn generate_goal(to_parse: &str) -> Result<Goal, String> {
    match tokenize(to_parse) {
        Ok(tokens) => {
            // For debugging.
            let mut base_token = group_tokens(&tokens, 0);
            base_token = group_and_tokens(base_token);
            base_token = group_or_tokens(base_token);
            return token_tree_to_goal(base_token);
        },
        Err(err) => { Err(err) },
    }
} // generate_goal()


/// tokenize()
/// Divides the given string into a series of tokens.
///
/// Note:
/// Parentheses can be used to define a complex term,
/// such as likes(Charles, Gina), or to group terms:
/// (father($X, $Y); mother($X, $Y))
///
/// # Arguments
/// * `to_parse` - string to parse
/// # Return
/// * `Result` - Ok(tokens) or Err(message)
fn tokenize(to_parse: &str) -> Result<Vec<Token>, String> {

    let mut tokens: Vec<Token> = vec![];
    let mut parse_stk = ParseStack::new();
    let s = to_parse.trim();

    if s.len() == 0 {
        let msg = "tokenize() - String is empty.".to_string();
        return Err(msg);
    }

    let mut start_index = 0;
    let chrs = str_to_chars!(s);
    let length = chrs.len();

    // Find a separator (comma, semicolon), if there is one.
    let mut previous = '#';  // random

    let mut i = start_index;
    while i < length {

        // Get the top of the stack.
        let mut top = peek(&mut parse_stk);

        let mut ch = chrs[i];
        if ch == '"' { // Ignore characters between quotes.
            let mut j = i + 1;
            while j < length {
                ch = chrs[j];
                if ch == '"' {
                    i = j;
                    break;
                }
                j += 1;
            }
        }
        else if ch == '(' {
            // Is the previous character valid in a functor?
            if letter_number_hyphen(previous) {
                parse_stk.push(TokenType::Complex);
            } else {
                parse_stk.push(TokenType::Group);
                tokens.push(make_leaf_token("("));
                start_index = i + 1;
            }
        }
        else if ch == ')' {
            if top == TokenType::Empty {
                let msg = format!("tokenize() - Unmatched parenthesis: {}", s);
                return Err(msg);
            }
            top = pop(&mut parse_stk);
            if top == TokenType::Group {
                let subgoal = chars_to_string!(chrs[start_index..i]);
                tokens.push(make_leaf_token(&subgoal));
                tokens.push(make_leaf_token(")"));
            } else if top != TokenType::Complex {
                let msg = format!("tokenize() - Unmatched parenthesis: {}", s);
                return Err(msg);
            }
        } else if ch == '[' {
            parse_stk.push(TokenType::LinkedList);
        } else if ch == ']' {
            if top == TokenType::Empty {
                let msg = format!("tokenize() - Unmatched bracket: {}", s);
                return Err(msg);
            }
            top = pop(&mut parse_stk);
            if top != TokenType::LinkedList {
                let msg = format!("Tokenize() - Unmatched bracket: {}", s);
                return Err(msg);
            }
        }
        else {
            // If not inside complex term or linked list...
            if top != TokenType::Complex && top != TokenType::LinkedList {
                if invalid_between_terms(ch) {
                    let msg = format!("tokenize() - Invalid character: {}", s);
                    return Err(msg);
                }
                if ch == ',' {   // And
                    let subgoal = chars_to_string!(chrs[start_index..i]);
                    tokens.push(make_leaf_token(&subgoal));
                    tokens.push(make_leaf_token(","));
                    start_index = i + 1;
                } else if ch == ';' {   // Or
                    let subgoal = chars_to_string!(chrs[start_index..i]);
                    tokens.push(make_leaf_token(&subgoal));
                    tokens.push(make_leaf_token(";"));
                    start_index = i + 1;
                }
            }
        } // else

        previous = ch;
        i += 1;
    } // while

    if parse_stk.len() > 0 {
        let msg = format!("tokenize() - Invalid term: {}", s);
        return Err(msg);
    }

    if length - start_index > 0 {
        let subgoal = chars_to_string!(chrs[start_index..length]);
        tokens.push(make_leaf_token(&subgoal));
    }

    return Ok(tokens);

} // tokenize()


/// group_tokens()
///
/// Collects tokens within parentheses into groups.
/// Converts a flat array of tokens into a tree of tokens.
///
/// For example, this:   SUBGOAL SUBGOAL ( SUBGOAL  SUBGOAL )
/// becomes:
///          GROUP
///            |
/// SUBGOAL SUBGOAL GROUP
///                   |
///            SUBGOAL SUBGOAL
///
/// There is a precedence order in subgoals. From highest to lowest.
///
///    Group
///    And
///    Or
///
/// # Argument
/// * `tokens`
/// * `index`
/// # Return
/// * `Token`
///
fn group_tokens(tokens: &Vec<Token>, mut index: usize) -> Token {

    let mut new_tokens: Vec<Token> = vec![];
    let size = tokens.len();

    while index < size {

        let token = tokens[index].clone();
        let the_type = token.get_type();

        if the_type == TokenType::LParen {
            index += 1;
            // Make a GROUP token.
            let t = group_tokens(tokens, index);
            // Skip past tokens already processed.
            // +1 for right parenthesis
            index += t.number_of_children() + 1;
            new_tokens.push(t);
        } else if the_type == TokenType::RParen {
            // Add all remaining tokens to the list.
            return make_branch_token(TokenType::Group, new_tokens);
        } else {
            new_tokens.push(token);
        }
        index += 1;

    } // for

    return make_branch_token(TokenType::Group, new_tokens)

} // group_tokens


/// group_and_tokens()
///
/// Groups child tokens which are separated by commas.
///
/// # Arguments
/// * `token`
/// # Return
/// * `token`
/// # Panics
/// * If given token is not a branch token.
fn group_and_tokens(token: Token) -> Token {

    match token {
        Token::Leaf{ token_type: tt, token_str: _ } => {
            let err = format!(
                "group_and_tokens() - Requires branch token: {tt}"
            );
            panic!("{}", err);
        },
        Token::Branch{ token_type, children } => {

            let mut new_children: Vec<Token> = vec![];
            let mut and_list: Vec<Token> = vec![];

            for child in children {
                let child_type = child.get_type();
                if child_type == TokenType::Subgoal {
                    and_list.push(child);
                }
                else if child_type == TokenType::Comma {
                    // Nothing to do.
                }
                else if child_type == TokenType::Semicolon {
                    // Must be end of comma separated list.
                    let size = and_list.len();
                    if size == 1 {
                        new_children.push(and_list[0].clone());
                    } else {
                        new_children.push(
                            make_branch_token(TokenType::And, and_list)
                        );
                    }
                    new_children.push(child);
                    and_list = vec![];
                }
                else if child_type == TokenType::Group {
                    let mut t = group_and_tokens(child);
                    t = group_or_tokens(t);
                    and_list.push(t);
                }
            } // for

            let size = and_list.len();
            if size == 1 {
                new_children.push(and_list[0].clone());
            } else if size > 1 {
                new_children.push(
                    make_branch_token(TokenType::And, and_list)
                );
            }

            make_branch_token(token_type, new_children)

        } // Token::Branch
    } // match
} // group_and_tokens


/// group_or_tokens()
///
/// Groups child tokens which are separated by semicolons into
/// an Or token. The given token must be a branch token.
///
/// # Arguments
/// * `token`
/// # Return
/// * `token`
/// # Panics
/// * If given token is not a branch token.
fn group_or_tokens(token: Token) -> Token {

    match token {
        Token::Leaf{ token_type: tt, token_str: _ } => {
            let err = format!(
                "group_or_tokens() - Requires branch token: {tt}"
            );
            panic!("{}", err);
        },
        Token::Branch{ token_type, children } => {

            let mut new_children: Vec<Token> = vec![];
            let mut or_list: Vec<Token> = vec![];

            for child in children {
                let child_type = child.get_type();
                if child_type == TokenType::Subgoal ||
                   child_type == TokenType::And ||
                   child_type == TokenType::Group {
                    or_list.push(child);
                }
                else if child_type == TokenType::Semicolon {
                    // Nothing to do.
                }
            } // for

            let size = or_list.len();
            if size == 1 {
                new_children.push(or_list[0].clone());
            }
            else if size > 1 {
                new_children.push(
                    make_branch_token(TokenType::Or, or_list)
                );
            }

            make_branch_token(token_type, new_children)

        } // Branch
    } // match
} // group_or_tokens


/// token_tree_to_goal()
///
/// Returns a goal for the given token tree, or an error message
///
/// # Argument
/// * `token` - base of token tree
/// # Return
/// * `Result` - Ok(Goal) or Err(message)
/// # Panics
/// * If leaf token is not a Subgoal.
/// * If branch token Group does not have 1 child.
fn token_tree_to_goal(token: Token) -> Result<Goal, String> {

    match token {

        Token::Leaf{ token_type, token_str } => {
            if token_type == TokenType::Subgoal {
                return parse_subgoal(&token_str);
            };
            let msg = tttg_error("Invalid. Leaf token must be Subgoal.", "");
            panic!("{}", msg);
        }, // Leaf

        Token::Branch{ token_type, children: _ } => {

            if token_type == TokenType::And {

                let mut operands: Vec<Goal> = vec![];
                let children = token.get_children();

                for child in children {
                    let child_type = child.get_type();
                    if child_type == TokenType::Subgoal {
                        let s = child.get_token_str();
                        match parse_subgoal(&s) {
                            Ok(g) => { operands.push(g); },
                            Err(err) => { return Err(err); },
                        }
                    }
                    else if child_type == TokenType::Group {
                        match token_tree_to_goal(child) {
                            Ok(g) => { operands.push(g); },
                            Err(err) => { return Err(err); },
                        }
                    }
                } // for child...

                let op = Operator::And(operands);
                return Ok(Goal::OperatorGoal(op));
            }; // token_type == And

            if token_type == TokenType::Or {

                let mut operands: Vec<Goal> = vec![];
                let children = token.get_children();

                for child in children {
                    let child_type = child.get_type();
                    if child_type == TokenType::Subgoal {
                        let s = child.get_token_str();
                        match parse_subgoal(&s) {
                            Ok(g) => { operands.push(g); },
                            Err(err) => { return Err(err); },
                        }
                    }
                    else if child_type == TokenType::Group {
                        match token_tree_to_goal(child) {
                            Ok(g) => { operands.push(g); },
                            Err(err) => { return Err(err); },
                        }
                    }
                } // for child...
                let op = Operator::Or(operands);
                return Ok(Goal::OperatorGoal(op));
            };

            if token_type == TokenType::Group {

                if token.number_of_children() != 1 {
                    let msg = tttg_error("Group should have 1 child.", "");
                    panic!("{}", msg);
                }

                let children = token.get_children();
                let child = children[0].clone();
                return token_tree_to_goal(child);
            };

            let tt = token_type.to_string();
            let msg = tttg_error("Invalid token type:", &tt);
            return Err(msg);

        }, // Branch

    } // match

}  // token_tree_to_goal()


// Formats an error message for token_tree_to_goal().
// Arguments:
//   err - error description
//   bad - string which caused the error
// Return:
//   error message (String)
fn tttg_error(err: &str, bad: &str) -> String {
    format!("token_tree_to_goal() - {} {}", err, bad)
}


#[cfg(test)]
mod test {

    use crate::*;
    use super::*;

    /// tokens_to_string()
    ///
    /// Makes a string representation of a vector of tokens,
    /// for debugging purposes.
    ///
    /// # Argument
    /// * `tokens` - vector of tokens
    /// # Return
    /// * `String`
    fn tokens_to_string(tokens: &Vec<Token>) -> String {
        let mut first = true;
        let mut out = "".to_string();
        for token in tokens {
            if !first { out += " "; }
            else { first = false; }
            let token_type = token.get_type();
            if token_type == TokenType::Subgoal {
                out += &token.get_token_str();
            }
            else {
                let s = format!("{}", token_type);
                out += &s.to_uppercase();
            }
        }
        return out;
    } // tokens_to_string()

    /// make_test_tokens()
    /// Returns a vector of tokens for testing.
    /// The tokens are: c1, c2, ( c3; c4 ), c5
    fn make_test_tokens() -> Vec<Token> {

        let c1 = make_leaf_token("a(1)");
        let c2 = make_leaf_token("b(2)");
        let c3 = make_leaf_token("c(3)");
        let c4 = make_leaf_token("d(4)");
        let c5 = make_leaf_token("d(5)");

        let com1 = make_leaf_token(",");
        let com2 = make_leaf_token(",");
        let com3 = make_leaf_token(",");

        let sem = make_leaf_token(";");
        let lp  = make_leaf_token("(");
        let rp  = make_leaf_token(")");
        return vec![c1, com1, c2, com2, lp, c3, sem, c4, rp, com3, c5];
    } // make_test_tokens()

    #[test]
    fn test_tokens_to_string() {
        let tokens = make_test_tokens();
        let s1 = "a(1) COMMA b(2) COMMA LPAREN c(3) \
                  SEMICOLON d(4) RPAREN COMMA d(5)";
        let s2 = tokens_to_string(&tokens);
        assert_eq!(s1, s2);
    } // test_tokens_to_string()


    #[test]
    fn test_group_tokens() {

        let test_tokens = make_test_tokens();

        let token = group_tokens(&test_tokens, 0);
        assert_eq!(token.to_string(),
        "GROUP > SUBGOAL(a(1)) COMMA SUBGOAL(b(2)) COMMA GROUP SUBGOAL(d(5))");

        let token = group_and_tokens(token);
        assert_eq!(token.to_string(), "GROUP > AND");

        let child = &token.get_children()[0];
        assert_eq!(child.to_string(),
        "AND > SUBGOAL(a(1)) SUBGOAL(b(2)) GROUP SUBGOAL(d(5))");

        let children = child.get_children();
        let child = &children[2];
        assert_eq!(child.to_string(), "GROUP > OR");

        let child = &child.get_children()[0];
        assert_eq!(child.to_string(),
        "OR > SUBGOAL(c(3)) SUBGOAL(d(4))");

    } // test_group_tokens()

    /// make_test_tokens2()
    /// Returns a vector of tokens for testing.
    /// The tokens are: c1, c2; c3, c4
    fn make_test_tokens2() -> Vec<Token> {

        let c1 = make_leaf_token("a(1)");
        let c2 = make_leaf_token("b(2)");
        let c3 = make_leaf_token("c(3)");
        let c4 = make_leaf_token("d(4)");

        let com1 = make_leaf_token(",");
        let com2 = make_leaf_token(",");
        let sem = make_leaf_token(";");

        return vec![c1, com1, c2, sem, c3, com2, c4];
    } // make_test_tokens2()

    #[test]
    fn test_group_tokens2() {

        let test_tokens = make_test_tokens2();

        let token = group_tokens(&test_tokens, 0);
        assert_eq!(token.to_string(),
             "GROUP > SUBGOAL(a(1)) COMMA SUBGOAL(b(2)) \
             SEMICOLON SUBGOAL(c(3)) COMMA SUBGOAL(d(4))");

        let token = group_and_tokens(token);
        assert_eq!(token.to_string(), "GROUP > AND SEMICOLON AND");

        let token = group_or_tokens(token);
        assert_eq!(token.to_string(), "GROUP > OR");

    } // test_group_tokens2()

    /// make_test_tokens3()
    /// Returns a vector of tokens for testing.
    /// The tokens are: c1; ( c2, c3; c4 )
    fn make_test_tokens3() -> Vec<Token> {

        let c1 = make_leaf_token("a(1)");
        let c2 = make_leaf_token("b(2)");
        let c3 = make_leaf_token("c(3)");
        let c4 = make_leaf_token("d(4)");

        let com  = make_leaf_token(",");
        let sem1 = make_leaf_token(";");
        let sem2 = make_leaf_token(";");

        let lp  = make_leaf_token("(");
        let rp  = make_leaf_token(")");
        return vec![c1, sem1, lp, c2, com, c3, sem2, c4, rp];

    } // make_test_tokens3()

    #[test]
    fn test_group_tokens3() {

        let test_tokens = make_test_tokens3();
        let token = group_tokens(&test_tokens, 0);
        let token = group_and_tokens(token);
        let token = group_or_tokens(token);
        assert_eq!(token.to_string(), "GROUP > OR");

        let child = &token.get_children()[0];
        assert_eq!(child.to_string(), "OR > SUBGOAL(a(1)) GROUP");

        let child = &child.get_children()[1];
        let child = &child.get_children()[0];
        assert_eq!(child.to_string(), "OR > AND SUBGOAL(d(4))");

        let child = &child.get_children()[0];
        assert_eq!(child.to_string(), "AND > SUBGOAL(b(2)) SUBGOAL(c(3))");

    } // test_group_tokens3()

    #[test]
    #[should_panic]
    fn test_group_tokens_panic() {
        let com1 = make_leaf_token(",");
        group_and_tokens(com1);
    } // test_group_tokens_panic()

    #[test]
    fn test_tokenize() {
        let s = "a(1, 2), b(3, 4); c(5, 6), c(7, 8)";
        match tokenize(s) {
            Ok(tokens) => {
                let s = tokens_to_string(&tokens);
                assert_eq!(s,
                  "a(1, 2) COMMA b(3, 4) SEMICOLON c(5, 6) COMMA c(7, 8)");
            },
            Err(err) => {
                panic!("Should create tokens: {}", err);
            },
        }

        let s = "a(1, 2), b(3, 4";
        match tokenize(s) {
            Ok(_tokens) => {
                panic!("Missing parenthesis. Should not create tokens.");
            },
            Err(err) => {
                assert_eq!(err, "tokenize() - Invalid term: a(1, 2), b(3, 4");
            },
        }

        let s = "$X = 1, 2, 3]";
        match tokenize(s) {
            Ok(_tokens) => {
                panic!("Missing parenthesis. Should not create tokens.");
            },
            Err(err) => {
                assert_eq!(err, "tokenize() - Unmatched bracket: $X = 1, 2, 3]");
            },
        }
    } // test_tokenize()

} // test
