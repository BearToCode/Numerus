mod display;

use super::{
    token::{tokentype::TokenType, Token},
    value::{valuetype::ValueType, Value},
};

pub type EvalResult<T> = Result<T, ErrorType>;

#[derive(Debug)]
pub enum ErrorType {
    /// A mismatched type.
    TypeError {
        expected: ValueType,
        given: ValueType,
    },
    /// An unknown token found while parsing the string.
    UnknownToken { token: String },
    /// A known token placed in an invalid position.
    InvalidTokenAtPosition { token: TokenType },
    /// A failed cast due to data loss.
    FailedCast {
        value: Value,
        from: ValueType,
        To: ValueType,
    },
    /// Two arrays with different lengths.
    MismatchedArrayLengths {
        first: usize,
        second: usize,
        operation_name: &'static str,
    },
    /// Trying to divide by zero.
    DivideByZero { numerator: Value },

    /// An error wrapper to add additional information.
    ErrorDuring {
        operation_name: &'static str,
        error: Box<ErrorType>,
    },
}

impl std::error::Error for ErrorType {}