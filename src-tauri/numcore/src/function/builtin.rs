use std::collections::HashMap;

use crate::{
    create_func, decl_func,
    function::*,
    function::{Arguments, Function},
    out::ErrorType,
    value::Value,
    EvalResult, ValueType,
};
use lazy_static::*;
use rand::Rng;
use tuple_conv::RepeatedTuple;

lazy_static! {
    pub static ref CONSTANTS: HashMap<&'static str, Value> = {
        let mut m = HashMap::new();
        use std::f64::consts;
        // Math constants
        m.insert("pi", Value::Float(consts::PI));
        m.insert("e", Value::Float(consts::E));
        m.insert("tau", Value::Float(consts::TAU));
        m.insert("phi", Value::Float(1.618_033_988_749_894));

        // Literal values
        m.insert("true", Value::Bool(true));
        m.insert("false", Value::Bool(false));
        m.insert("i", Value::Complex(num::Complex::i()));

        m
    };
    #[derive(Debug)]
    pub static ref BUILT_IN_FUNCTIONS: Vec<Function> = vec![
        create_func!(min, Arguments::Dynamic),
        create_func!(max, Arguments::Dynamic),
        create_func!(floor, Arguments::Const(1)),
        create_func!(ceil, Arguments::Const(1)),
        create_func!(round, Arguments::Const(1)),
        create_func!(ln, Arguments::Const(1)),
        create_func!(log, Arguments::Const(2)),
        create_func!(exp, Arguments::Const(1)),
        create_func!(rand, Arguments::Const(2)),

        create_func!(branch, Arguments::Const(3)),

        create_func!(sin, Arguments::Const(1)),
        create_func!(cos, Arguments::Const(1)),
        create_func!(tan, Arguments::Const(1)),
        create_func!(asin, Arguments::Const(1)),
        create_func!(acos, Arguments::Const(1)),
        create_func!(atan, Arguments::Const(1)),
        create_func!(sinh, Arguments::Const(1)),
        create_func!(cosh, Arguments::Const(1)),
        create_func!(tanh, Arguments::Const(1)),
        create_func!(asinh, Arguments::Const(1)),
        create_func!(acosh, Arguments::Const(1)),
        create_func!(atanh, Arguments::Const(1)),

        create_func!(re, Arguments::Const(1)),
        create_func!(im, Arguments::Const(1)),
        create_func!(polar, Arguments::Const(1)),
        create_func!(arg, Arguments::Const(1)),
        create_func!(norm, Arguments::Const(1)),

    ];
}

/// Returns `Some(Function)` if the identifier matches some.
pub fn get_function(identifier: &str) -> Option<Function> {
    BUILT_IN_FUNCTIONS
        .iter()
        .find(|x| x.func_identifier == identifier)
        .cloned()
}

/// Returns `Some(Value)` if the identifier matches some.
pub fn get_const(identifier: &str) -> Option<Value> {
    CONSTANTS.get(identifier).cloned()
}

/// Returns all reserved keywords.
pub fn reserved_keywords() -> Vec<&'static str> {
    [
        CONSTANTS.keys().cloned().collect::<Vec<&'static str>>(),
        BUILT_IN_FUNCTIONS
            .iter()
            .map(|x| x.func_identifier)
            .collect(),
    ]
    .concat()
}

// UTILS

macro_rules! read_vec_values {
    ( $vec:expr, $($x:ident),* ) => {
        let vec = $vec.as_vector();
        let mut iter = vec.iter();

        $(
            let $x = match iter.next() {
                Some(value) => value,
                None => return Err(ErrorType::InternalError { message: "failed to retrieve function parameters".to_owned() })
            };
        )*
    };
}

// STD

decl_func!(
    min,
    |v| {
        let vec = v.as_vector();
        let mut min = vec[0].as_float()?;
        for elem in vec {
            if elem.as_float()? < min {
                min = elem.as_float()?;
            }
        }
        Ok(Value::Float(min))
    },
    ValueType::VectorType
);

decl_func!(
    max,
    |v| {
        let vec = v.as_vector();
        let mut max = vec[0].as_float()?;
        for elem in vec {
            if elem.as_float()? > max {
                max = elem.as_float()?;
            }
        }
        Ok(Value::Float(max))
    },
    ValueType::VectorType
);

decl_func!(floor, |v| Ok(v.as_float()?.floor()), ValueType::FloatType);

decl_func!(ceil, |v| Ok(v.as_float()?.floor()), ValueType::FloatType);

decl_func!(round, |v| Ok(v.as_float()?.round()), ValueType::FloatType);

decl_func!(ln, |v| Ok(v.as_complex()?.ln()), ValueType::ComplexType);

decl_func!(
    log,
    |v| {
        read_vec_values!(v, base, argument);
        Ok(argument.as_complex()?.log(base.as_float()?))
    },
    ValueType::VectorType
);

decl_func!(exp, |v| Ok(v.as_complex()?.exp()), ValueType::ComplexType);

decl_func!(
    rand,
    |v| {
        read_vec_values!(v, min, max);
        Ok(Value::Float(
            rand::thread_rng().gen_range(min.as_float()?..max.as_float()?),
        ))
    },
    ValueType::VectorType
);

// LOGIC

fn branch(arguments: &Vec<Box<Expression>>, context: &Context) -> EvalResult<Value> {
    let condition = arguments[0].eval(context, None)?.as_bool()?;
    if condition {
        Ok(arguments[1].eval(context, None)?)
    } else {
        Ok(arguments[2].eval(context, None)?)
    }
}

// TRIGONOMETRY

decl_func!(sin, |v| Ok(v.as_complex()?.sin()), ValueType::ComplexType);

decl_func!(cos, |v| Ok(v.as_complex()?.cos()), ValueType::ComplexType);

decl_func!(tan, |v| Ok(v.as_complex()?.tan()), ValueType::ComplexType);

decl_func!(asin, |v| Ok(v.as_complex()?.asin()), ValueType::ComplexType);

decl_func!(acos, |v| Ok(v.as_complex()?.acos()), ValueType::ComplexType);

decl_func!(atan, |v| Ok(v.as_complex()?.atan()), ValueType::ComplexType);

decl_func!(sinh, |v| Ok(v.as_complex()?.sinh()), ValueType::ComplexType);

decl_func!(cosh, |v| Ok(v.as_complex()?.cosh()), ValueType::ComplexType);

decl_func!(tanh, |v| Ok(v.as_complex()?.tanh()), ValueType::ComplexType);

decl_func!(
    asinh,
    |v| Ok(v.as_complex()?.asinh()),
    ValueType::ComplexType
);

decl_func!(
    acosh,
    |v| Ok(v.as_complex()?.acosh()),
    ValueType::ComplexType
);

decl_func!(
    atanh,
    |v| Ok(v.as_complex()?.atanh()),
    ValueType::ComplexType
);

// COMPLEX

decl_func!(re, |v| Ok(v.as_complex()?.re), ValueType::ComplexType);

decl_func!(im, |v| Ok(v.as_complex()?.im), ValueType::ComplexType);

decl_func!(
    polar,
    |v| Ok(v.as_complex()?.to_polar().to_vec()),
    ValueType::ComplexType
);

decl_func!(arg, |v| Ok(v.as_complex()?.arg()), ValueType::ComplexType);

decl_func!(norm, |v| Ok(v.as_complex()?.norm()), ValueType::ComplexType);
