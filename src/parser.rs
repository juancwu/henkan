use crate::token::Token;
use std::fmt::{self};

pub enum ASTNode {
    Expr {
        value: Token,
        value_unit: Token,
        operator: Token,
        unit: Token,
    },
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ASTNode")
    }
}

/// This is a simple parsing function to only parse expressions.
/// Only for getting the parsing started.
pub fn parse_expression(tokens: &[Token]) -> Option<ASTNode> {
    if tokens.is_empty() || tokens.len() < 4 {
        return None;
    }

    let left = if let Token::Number(v) = tokens[0] {
        Token::Number(v)
    } else {
        return None;
    };

    let left_unit = tokens[1].clone();

    let op = match match_op(&tokens[2]) {
        Some(op) => op,
        None => return None,
    };

    let right_unit = tokens[3].clone();

    Some(ASTNode::Expr {
        value: left,
        value_unit: left_unit,
        operator: op,
        unit: right_unit,
    })
}

/// Tries to match an operator token for the given token.
/// # Example
/// ````
/// let op = match_op(token) {
///     Some(t) => t,
///     None => None,
/// }
/// ````
fn match_op(token: &Token) -> Option<Token> {
    match token {
        Token::To => Some(Token::To),
        Token::In => Some(Token::In),
        Token::As => Some(Token::As),
        _ => None,
    }
}
