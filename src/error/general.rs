use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum GeneralError {
    #[error("Division by zero in '{operator}': divisor = 0 would produce +/- infinity")]
    DivisionByZero { operator: &'static str },

    #[error("Complex result in '{operator}': combining base {base} with exponent {exponent} would produce a complex number")]
    ComplexPowerResult {
        operator: &'static str,
        base: f64,
        exponent: f64,
    },

    #[error("Undefined logarithm in '{operator}': {detail}")]
    UndefinedLogarithm {
        operator: &'static str,
        detail: String,
    },

    #[error("Complex logarithm in '{operator}': {detail}")]
    ComplexLogarithm {
        operator: &'static str,
        detail: String,
    },

    #[error("Domain error in '{operator}': value {value} is outside the supported domain [{domain_min}, {domain_max}]")]
    DomainError {
        operator: &'static str,
        value: f64,
        domain_min: f64,
        domain_max: f64,
    },

    #[error("Edge case in '{operator}': {detail}")]
    EdgeCase {
        operator: &'static str,
        detail: String,
    },

    #[error("Complex result in '{operator}': value {value} is {relation} the supported domain")]
    ComplexResult {
        operator: &'static str,
        value: f64,
        relation: &'static str,
    },

    #[error("Failed to compare {left} and {right}: values are incomparable (NaN?)")]
    IncomparableValues { left: f64, right: f64 },
}
