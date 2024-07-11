use crate::{parser::ASTNode, token::Token};

pub fn eval_expression_ast(ast: &ASTNode) -> f64 {
    match ast {
        ASTNode::Expr {
            value,
            value_unit,
            operator,
            unit,
        } => {
            let n = match *value {
                Token::Number(v) => v,
                _ => 0.0,
            };
            // Skippping operator because for now all operators are uni-directional
            let op = match *operator {
                _ => "",
            };
            println!("WARN: Skipping operatore for expression. All operators are uni-directional now. OP: {}", op);
            match (value_unit, unit) {
                (Token::Metric { unit: from_unit }, Token::Metric { unit: to_unit }) => {
                    convert_metric_unit(n, &from_unit, &to_unit)
                }
                (Token::Temperature { unit: _ }, Token::Temperature { unit: to_unit }) => {
                    convert_temperature(n, &to_unit)
                }
                (_, _) => 0.0,
            }
        }
    }
}

/// Converts temperature values between Celsius and Fahrenheit.
/// # Example
/// ````
/// let celsius = 24;
/// let fahrenheit = convert_temperature(celsius, &Token::Fahrenheit);
/// ````
fn convert_temperature(value: f64, to: &Token) -> f64 {
    match to {
        Token::Celsius => {
            let celsius = (value - 32.0) * 5.0 / 9.0;
            celsius
        }
        Token::Fahrenheit => {
            let fahrenheit = value * 9.0 / 5.0 + 32.0;
            fahrenheit
        }
        _ => 0.0,
    }
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
