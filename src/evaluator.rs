use crate::parser::Expression;
use crate::parser::Expression::*;
use crate::units::SiValue;

use std::fmt;
use std::error;

#[derive(Clone, Debug)]
pub struct EvaluateError {
	message: String,
}

impl EvaluateError {
	fn new(x: &str) -> EvaluateError {
		EvaluateError {message: format!("evaluate_error: {}", String::from(x))}
	}
}

impl fmt::Display for EvaluateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for EvaluateError {}

pub fn evaluate(input: Box<Expression>) -> Result<SiValue, EvaluateError> {
	match *input {
		PrimaryInt(i) => Ok(SiValue::from(i as f64)),
		PrimaryFloat(f) => Ok(SiValue::from(f)),
		Unit(u) => Ok(SiValue::from(u)),
		UnitPow(u, i) => Ok(SiValue::from(u).pow(i)),
		Quantity(x, y) => Ok(evaluate(x)? * evaluate(y)?),
		Binary(x, o, y) => {
			match o {
				'+' => Ok(evaluate(x)? + evaluate(y)?),
				'-' => Ok(evaluate(x)? + (evaluate(y)? * -1.0)),
				'*' => Ok(evaluate(x)? * evaluate(y)?),
				'/' => Ok(evaluate(x)? * evaluate(y)?.pow(-1)),
				_ => Err(EvaluateError::new("bad operator"))
			}
		}
		NoExp => Err(EvaluateError::new("encountered no_exp"))
	}
}