use crate::{parser::ASTNode, token::Token, util::is_metric_unit};

pub fn eval_expression_ast(ast: &ASTNode) -> f64 {
    match ast {
        ASTNode::Expr {
            value,
            value_unit,
            // Skippping operator because for now all operators are uni-directional
            operator: _op,
            unit,
        } => {
            let n = match value {
                Token::Number(v) => *v,
                _ => 0.0,
            };
            if let Some(converted_value) = convert_unit(n, value_unit, unit) {
                converted_value
            } else {
                0.0
            }
        }
    }
}

/// Base function to call when evaluating expressions.
fn convert_unit(value: f64, from: &Token, to: &Token) -> Option<f64> {
    match (from, to) {
        (Token::Celsius, Token::Fahrenheit) => celsius_to_fahrenheit(value),
        (Token::Fahrenheit, Token::Celsius) => fahrenheit_to_celsius(value),
        (Token::Fahrenheit, Token::Fahrenheit) => Some(value),
        (Token::Celsius, Token::Celsius) => {
            println!("here");
            Some(value)
        }
        _ if is_metric_unit(from) && is_metric_unit(to) => {
            Some(convert_metric_unit(value, from, to))
        }
        _ => None,
    }
}

/// Converts celsius to fahrenheit.
fn celsius_to_fahrenheit(value: f64) -> Option<f64> {
    Some(value * 9.0 / 5.0 + 32.0)
}

/// Converts fahrentheit to celsius.
fn fahrenheit_to_celsius(value: f64) -> Option<f64> {
    Some((value - 32.0) * 5.0 / 9.0)
}

/// Find the level in the metric scale where "meter" is the based unit.
/// # Example
/// ````
/// let km_power = find_metric_unit_power(km_token);
/// println!("{}", km_power); // 3
/// ````
/// # Example
/// ````
/// let mm_power = find_metric_unit_power(mm_token);
/// println!("{}", mm_power); // -3
/// ````
fn find_metric_unit_power(token: &Token) -> i32 {
    match token {
        Token::Millimeter => -3,
        Token::Centimeter => -2,
        Token::Meter => 0,
        Token::Kilometer => 3,
        _ => 0,
    }
}

/// Converts metric value units.
/// Given a value A in unit X, converting to unit Y:
/// AX = A * 10^(pow(X), pow(Y))Y
/// # Example
/// ````
/// let value = convert_metric_unit(10.0, Token::Millimeter, Token::Meter); // 0.01m
/// ````
fn convert_metric_unit(a: f64, unit_x: &Token, unit_y: &Token) -> f64 {
    let base: f64 = 10.0;
    let power_x: i32 = find_metric_unit_power(unit_x);
    let power_y: i32 = find_metric_unit_power(unit_y);
    let multiplier: i32 = power_x - power_y;
    a * base.powi(multiplier)
}
