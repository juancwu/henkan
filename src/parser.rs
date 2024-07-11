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

    let left_unit = match match_unit(&tokens[1]) {
        Some(u) => u,
        None => return None,
    };

    let op = match match_op(&tokens[2]) {
        Some(op) => op,
        None => return None,
    };

    let right_unit = match match_unit(&tokens[3]) {
        Some(u) => u,
        None => return None,
    };

    Some(ASTNode::Expr {
        value: left,
        value_unit: left_unit,
        operator: op,
        unit: right_unit,
    })
}

/// Tries to match a unit in the given token.
/// # Example
/// ````
/// let unit = match match_unit(token) {
///     Some(t) => t,
///     None => None,
/// };
/// ````
fn match_unit(token: &Token) -> Option<Token> {
    let t = match token {
        Token::Centimeter => Token::Centimeter,
        Token::Millimeter => Token::Millimeter,
        Token::Meter => Token::Meter,
        Token::Kilometer => Token::Kilometer,
        Token::Celsius => Token::Celsius,
        Token::Fahrenheit => Token::Fahrenheit,
        _ => {
            return None;
        }
    };
    Some(t)
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
    let op = match token {
        Token::To => Token::To,
        Token::In => Token::In,
        Token::As => Token::As,
        _ => {
            return None;
        }
    };
    Some(op)
}
