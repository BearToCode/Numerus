use super::out::{ErrorType, EvalResult};
use super::token::tokentype::TokenType;
use super::value::*;
use super::{
    function::Function,
    token::Token,
    value::{valuetype::ValueType, Value},
};
use num::complex::Complex;

pub struct Identifier(String);

pub enum Expression {
    /// A binary operation between two expression.
    Binary(Box<Expression>, TokenType, Box<Expression>),
    /// An unary operation to an expression.
    Unary(TokenType, Box<Expression>),
    /// A variable.
    Var(Identifier),
    /// A function call and its parameters.
    Func(Function, Vec<Expression>),
    /// A literal value.
    Value(Value),
    // TODO: An equation.
    // Equation(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn eval(&self) -> EvalResult<Value> {
        match self {
            Self::Binary(left_expr, token_type, right_expr) => {
                let left_value = (**left_expr).eval()?;
                let right_value = (**right_expr).eval()?;
                Ok(match token_type {
                    // Sum
                    TokenType::Plus => Value::add(left_value, right_value)?,
                    TokenType::Minus => Value::sub(left_value, right_value)?,
                    TokenType::Star => Value::mul(left_value, right_value)?,
                    TokenType::Slash => Value::div(left_value, right_value)?,
                    _ => return Err(ErrorType::InvalidTokenAtPosition { token: *token_type }),
                })
            }
            _ => unimplemented!(),
        }
    }
}