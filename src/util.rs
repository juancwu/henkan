use crate::token::Token;

/// Tries to match a unit in the given token.
/// # Example
/// ````
/// if is_metric_unit(&token) {
///     // do something...
/// }
/// ````
pub fn is_metric_unit(token: &Token) -> bool {
    matches!(
        token,
        Token::Millimeter | Token::Centimeter | Token::Meter | Token::Kilometer
    )
}
