// NOTE: derive Debug to allow using "{:?}" formatter and PartialEq for == and != operations
// PartialEq: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
// Debug: https://doc.rust-lang.org/std/fmt/trait.Debug.html
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Illegal, // Illegal Characters
    EOL,     // End of Line

    // Number
    Number(f64),

    // Operators
    To,
    In,
    As,

    // Metric
    Millimeter,
    Centimeter,
    Meter,
    Kilometer,

    // Temperature
    Celcius,
    Fahrenheit,

    // Command Keywords
    CopyCmd,
    HelpCmd,
}
