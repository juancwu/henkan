// NOTE: derive Debug to allow using "{:?}" formatter and PartialEq for == and != operations
// PartialEq: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
// Debug: https://doc.rust-lang.org/std/fmt/trait.Debug.html
#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal, // Illegal Characters
    EOL,     // End of Line

    // Number
    Int(i64),
    Float(f64),

    Operator(String), // verb operator

    // Metric
    Millimeter,
    Centimeter,
    Meter,
    Kilometer,

    // Temperature
    Celcius,
    Fahrenheit,

    // Identifier for commands and arguments
    Identifier(String),

    // Command Keywords
    CopyCmd,
    HelpCmd,
}
