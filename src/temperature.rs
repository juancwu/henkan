/// Converts celsius to fahrenheit.
///
/// # Example
/// ```
/// mod temperature;
///
/// let temp = "28c in f";
/// let f = temperature::celsius_to_fahrenheit(temp);
/// println!("28 Celsius in Fahrenheit is {}", f);
/// ```
pub fn celsius_to_fahrenheit(input: &str) -> f64 {
    0.0
}

/// Parses a string representation of a Celsius temperature
/// and returns the numeric value suitable for calculations.
///
/// # Example
/// ```
/// // #1
/// let temp = parse_celsius("28c");
/// ```
///
/// ```
/// // #2
/// let temp = parse_celsius("28C");
/// ```
fn parse_celsius(temp: &str) -> f64 {
    0.0
}

/// Parses a string representation of a Fahrenheit temperature
/// and returns the numeric value suitable for calculations.
///
/// # Example
/// ```
/// // #1
/// let temp = parse_fharenheit("78f");
/// ```
///
/// ```
/// // #2
/// let temp = parse_fharenheit("78F");
/// ```
fn parse_fahrenheit(temp: &str) -> f64 {
    0.0
}
