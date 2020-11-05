use crate::parser::Expression;
use crate::parser::Expression::*;
use crate::units::SiValue;

pub fn evaluate(input: Box<Expression>) -> SiValue {
	match *input {
		PrimaryInt(i) => SiValue::from(i as f64),
		PrimaryFloat(f) => SiValue::from(f),
		Unit(u) => SiValue::from(u),
		UnitPow(u, i) => SiValue::from(u).pow(i),
		Quantity(x, y) => evaluate(x) * evaluate(y),
		Binary(x, o, y) => {
			match o {
				'+' => evaluate(x) + evaluate(y),
				'-' => evaluate(x) + (evaluate(y) * -1.0),
				'*' => evaluate(x) * evaluate(y),
				'/' => evaluate(x) * evaluate(y).pow(-1),
				_ => { panic!("evaluate: bad operator") }
			}
		}
		NoExp => { panic!("evaluate: encountered NoExp") }
	}
}