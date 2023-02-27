//! A leaf or branch node of a token tree. Used for parsing.
//!
//! Leaf tokens have the following types:<br>
//!
//! <blockquote>
//! Subgoal, Comma, Semicolon, LParen, RParen
//! </blockquote>
//!
//! Branch tokens contain child tokens. Valid types are: Group, And, Or
//!
//! The precedence of branch tokens, from highest to lowest, is:
//! <blockquote>
//!    Group<br>
//!    And<br>
//!    Or
//! </blockquote>
//!
//! For the goal `(mother($X, $Y); father($X, $Y))`,<br>
//! the corresponding token types would be:
//! <blockquote>
//! LParen Subgoal Semicolon Subgoal RParen
//! </blockquote>
//!
// Cleve Lendon 2023

use std::fmt;

/// Identifies tokens in the token tree and on the parse stack.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    /// Represents an empty parse stack.
    Empty,
    /// Any subgoal, such as `$NP = [$H | $T]`.
    Subgoal,
    Comma,
    Semicolon,
    /// Left parenthesis
    LParen,
    /// Right parenthesis
    RParen,
    /// Groups goals between parentheses: (goal1, goal2, goal3)
    Group,
    /// Conjunction - goal and goal
    And,
    /// Disjunction - goal or goal
    Or,
    /// Complex term on the parse stack.
    Complex,
    /// List on the parse stack.
    LinkedList,
}

/// Used for tokenizing Suiron source.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {

    /// * Valid `token_type`s are:<br>Subgoal, Comma, Semicolon, LParen, RParen<br>
    /// * `token_str` holds the item being parsed,<br>
    /// such as `(`, `;`, or `a_subgoal(term1, term2)`.
    Leaf {
        token_type: TokenType,
        token_str: String,
    },

    /// This is a parent node.<br>
    /// * Valid `token_type`s are: Group, And, Or<br>
    /// * `children` is a vector of child tokens
    Branch {
        token_type: TokenType,
        children: Vec<Token>,
    },
}

/// Makes a leaf token from the given string.
///
/// Valid leaf token types are: Subgoal, Comma, Semicolon, LParen, RParen.
///
/// If the given symbol is not a comma, semicolon, left parenthesis
/// or right parenthesis, it is assumed to represent a subgoal.
///
/// # Arguments
/// * `symbol` - string slice
/// # Return
/// * `leaf token`
/// # Usages
/// ```
/// use suiron::*;
///
/// let l1 = make_leaf_token("brother($X, $Y)");
/// let l2 = make_leaf_token(";");
/// let l3 = make_leaf_token("sister($X, $Y)");
/// println!("{} {} {}", l1, l2, l3);
/// // Prints: SUBGOAL(brother($X, $Y)) SEMICOLON SUBGOAL(sister($X, $Y))
/// ```
///
pub fn make_leaf_token(symbol: &str) -> Token {
    let s = symbol.trim();
    match s {
        "," => {
            return Token::Leaf{token_type: TokenType::Comma,
                               token_str: s.to_string(),};
        },
        ";" => {
            return Token::Leaf{token_type: TokenType::Semicolon,
                               token_str: s.to_string(),};
        },
        "(" => {
            return Token::Leaf{token_type: TokenType::LParen,
                               token_str: s.to_string(),};
        },
        ")" => {
            return Token::Leaf{token_type: TokenType::RParen,
                               token_str: s.to_string(),};
        },
        _ => {
            return Token::Leaf{token_type: TokenType::Subgoal,
                               token_str: s.to_string(),};
        },
    } // match
} // make_leaf_token()

/// Makes a branch token from a vector of child tokens.
///
/// Valid branch tokens types are: Group, And, Or.
///
/// # Arguments
/// * `token_type` - TokenType
/// * `children` - vector of Tokens
/// # Return
/// * `branch token`
/// # Panics
/// * If token type is invalid.
/// # Usages
/// ```
/// use suiron::*;
///
/// let l1 = make_leaf_token("brother($X, $Y)");
/// let l2 = make_leaf_token("sister($X, $Y)");
/// let b1 = make_branch_token(TokenType::Or, vec![l1, l2]);
/// println!("{}", b1);
/// // Prints: OR > SUBGOAL(brother($X, $Y)) SUBGOAL(sister($X, $Y))
/// ```
///
pub fn make_branch_token(token_type: TokenType, children: Vec<Token>)
                         -> Token {
    if token_type != TokenType::And &&
       token_type != TokenType::Or &&
       token_type != TokenType::Group {
        let msg = format!("make_branch_token() - Invalid token type: {}", token_type);
        panic!("{}", msg);
    }
    Token::Branch{ token_type, children }
} // make_branch_token()


impl Token {

    /// Gets the number of children of a branch (parent) token.
    /// # Arguments
    /// * `self`
    /// # Return
    /// * `number of children`
    /// # Panics
    /// * If the token is not a branch token.
    /// # Usage
    /// ```
    /// use suiron::*;
    ///
    /// let l1 = make_leaf_token("subgoal1()");
    /// let l2 = make_leaf_token("subgoal2()");
    /// let branch = make_branch_token(TokenType::And, vec![l1, l2]);
    /// let n = branch.number_of_children();  // n = 2
    /// ```
    pub fn number_of_children(&self) -> usize {
        match self {
            Token::Branch{ token_type: _, children } => { children.len() },
            Token::Leaf{token_type: _, token_str: _} => {
                panic!("number_of_children() - Token must be a branch token.");
            },
        }
    }  // number_of_children()

    /// Gets the token type.
    /// # Arguments
    /// * `self`
    /// # Return
    /// * `TokenType`
    /// # Usage
    /// ```
    /// use suiron::*;
    ///
    /// let tok = make_leaf_token("all(destiny)");
    /// let type1 = tok.get_type(); // type1 = Subgoal
    /// ```
    ///
    pub fn get_type(&self) -> TokenType {
        match self {
            Token::Leaf{ token_type, token_str: _ } => { *token_type },
            Token::Branch{ token_type, children: _ } => { *token_type },
        }
    } // get_type()

    /// Gets the token string. Valid only for leaf tokens.
    /// # Arguments
    /// * `self`
    /// # Return
    /// * `token string`
    /// # Panics
    /// * If the token is not a leaf token.
    /// # Usage
    /// ```
    /// use suiron::*;
    ///
    /// let tok = make_leaf_token("all(destiny)");
    /// let s = tok.get_token_str();  // s = "all(destiny)"
    /// ```
    pub fn get_token_str(&self) -> String {
        match self {
            Token::Leaf{ token_type: _, token_str } => { token_str.clone() },
            Token::Branch{token_type: _, children: _ } => {
                panic!("get_token_str() - Token must be a leaf token.");
            },
        }
    } // get_token_str()

    /// Gets the child tokens of a parent (branch) token.
    /// # Arguments
    /// * `self`
    /// # Return
    /// * `vector of tokens`
    /// # Panics
    /// * If the token is not a branch token.
    /// # Usage
    /// ```
    /// use suiron::*;
    ///
    /// let t1 = make_leaf_token("left");
    /// let t2 = make_leaf_token("right");
    /// let t3 = make_branch_token(TokenType::Or, vec![t1, t2]);
    /// let children = t3.get_children();
    /// println!("{:?}", children);
    /// ```
    ///
    /// <pre>
    /// The above prints:
    /// [Leaf { token_type: Subgoal, token_str: "left" }, Leaf { token_type: Subgoal, token_str: "right" }]
    /// </pre>
    pub fn get_children(&self) -> Vec<Token> {
        match self {
            Token::Leaf{ token_type: _, token_str: _ } => {
                panic!("get_token_str() - Token must be a branch token.");
            },
            Token::Branch{token_type: _, children } => {
                return children.to_vec();
            },
        }
    } // get_children()

} // Token


// Display trait for TokenType.
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    } // fmt
} // fmt::Display


/// format_leaf()
/// Formats a leaf token for display. The type should be upper case.
/// If the token type is Subgoal, display the subgoal string in parentheses.
///
/// Eg.
///    COMMA
///    SEMICOLON
///    SUBGOAL(subgoal(a, b, c))
///
/// # Arguments
/// * `token_type`
/// * `token_str`
/// # Return
/// * `String` - upper case
fn format_leaf(token_type: TokenType, token_str: String) -> String {
    let s = token_type.to_string().to_uppercase();
    if token_type == TokenType::Subgoal { format!("{}({})", s, token_str) }
    else { format!("{}", s) }
}

// Display trait for Tokens. Used for debugging purposes.
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Leaf{ token_type, token_str } => {
                let s = format_leaf(*token_type, token_str.to_string());
                write!(f, "{}", s)
            },
            Token::Branch{ token_type, children } => {
                let mut s = token_type.to_string().to_uppercase() + " >";
                for child in children {
                    match child {
                        Token::Leaf{ token_type, token_str } => {
                            s = s + " " +
                                &format_leaf(*token_type, token_str.to_string());
                        },
                        Token::Branch{ token_type, children: _} => {
                            s = s + " " + &token_type.to_string().to_uppercase();
                        },
                    }
                }
                write!(f, "{}", s)
            }, 
        }
    } // fmt
} // fmt::Display


#[cfg(test)]
mod test {

    use crate::*;

    #[test]
    fn test_make_leaf_token() {
        let t1 = make_leaf_token("(");
        let t2 = make_leaf_token("goal1");
        let t3 = make_leaf_token(",");
        let t4 = make_leaf_token("goal2");
        let t5 = make_leaf_token(";");
        let t6 = make_leaf_token("goal3");
        let t7 = make_leaf_token(")");

        let s = format!("{} {} {} {} {} {} {}", t1, t2, t3, t4, t5, t6, t7);
        assert_eq!(s, "LPAREN SUBGOAL(goal1) COMMA SUBGOAL(goal2) \
                       SEMICOLON SUBGOAL(goal3) RPAREN");
    }

    #[test]
    fn test_make_branch_token() {

        let t1 = make_leaf_token("goal1");
        let t2 = make_leaf_token("goal2");
        let t3 = make_leaf_token("goal3");

        let t4 = make_branch_token(TokenType::And, vec![t1, t2, t3]);
        assert_eq!(t4.to_string(),
                   "AND > SUBGOAL(goal1) SUBGOAL(goal2) SUBGOAL(goal3)");

        let t1 = make_leaf_token("goal1");
        let t2 = make_leaf_token("goal2");
        let t3 = make_leaf_token("goal3");
        let t5 = make_branch_token(TokenType::Or, vec![t1, t2, t3]);
        assert_eq!(t5.to_string(),
                   "OR > SUBGOAL(goal1) SUBGOAL(goal2) SUBGOAL(goal3)");
    }

    // Test of invalid branch token type.
    // Branch token cannot have a type of Comma.
    #[test]
    #[should_panic]
    fn test_make_branch_token_panic() {
        let t1 = make_leaf_token("goal1");
        make_branch_token(TokenType::Comma, vec![t1]);
    }

    #[test]
    fn test_get_type() {

        let t1 = make_leaf_token("all(destiny)");
        let type1 = t1.get_type();
        assert_eq!(type1, TokenType::Subgoal);

        let t2 = make_branch_token(TokenType::And, vec![t1]);
        let type2 = t2.get_type();
        assert_eq!(type2, TokenType::And);
    }

    #[test]
    fn test_get_token_str() {
        let t1 = make_leaf_token("all(destiny)");
        let s = t1.get_token_str();
        assert_eq!(s, "all(destiny)");
    }

    // token_str is invalid for branch tokens.
    #[test]
    #[should_panic]
    fn test_get_token_str_panic() {
        let t1 = make_leaf_token("all(destiny)");
        let t2 = make_branch_token(TokenType::And, vec![t1]);
        t2.get_token_str();
    }

    #[test]
    fn test_number_of_children() {
        let t1 = make_leaf_token("left");
        let t2 = make_leaf_token("right");
        let t3 = make_branch_token(TokenType::And, vec![t1, t2]);
        assert_eq!(2, t3.number_of_children());
    }

    // number_of_children() is not valid for leaf tokens.
    #[test]
    #[should_panic]
    fn test_number_of_children_panic() {
        let t1 = make_leaf_token("left");
        t1.number_of_children();
    }

    #[test]
    fn test_get_children() {
        let t1 = make_leaf_token("left");
        let t2 = make_leaf_token("right");
        let t3 = make_branch_token(TokenType::And, vec![t1, t2]);
        let s = "[Leaf { token_type: Subgoal, token_str: \"left\" }, \
                  Leaf { token_type: Subgoal, token_str: \"right\" }]";
        let s2 = format!("{:?}", t3.get_children());
        assert_eq!(s, s2);
    }

    // A leaf token has no children. Cause panic.
    #[test]
    #[should_panic]
    fn test_get_children_panic() {
        let t1 = make_leaf_token("left");
        t1.get_children();
    }

} // test
