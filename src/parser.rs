use crate::lexer::Token;
use std::fmt;
use std::error;

// Using the following grammar:
// expression -> term
// term       -> factor (("-" | "+") term)?
// factor     -> quantity (("*" | "/") factor)?
// quantity   -> primary unit_pow*
// primary    -> i64 | f64 | "(" expression ")" | unit_pow
// unit_pow   -> String (i64)?

#[derive(Debug, PartialEq)]
pub enum Expression {
	NoExp,
	Binary(Box<Expression>, char, Box<Expression>),
	Quantity(Box<Expression>, Box<Expression>),
	PrimaryInt(i64),
	PrimaryFloat(f64),
	Unit(String),
	UnitPow(String, i64),
}

#[derive(Clone, Debug)]
pub struct ParseError {
	message: String,
}

impl ParseError {
	fn new(x: &str) -> ParseError {
		ParseError {message: format!("parse_error: {}", String::from(x))}
	}
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for ParseError {}

pub fn parse(mut input: Vec<Token>) -> Result<Box<Expression>, ParseError> {
	// takes a vector of tokens, parses into a single nested expression
	input.reverse();
	let exp = expression(&mut input)?;
	// all tokens should have been consumed
	if input.len() == 0 {
		return Ok(Box::new(exp))
	}
	println!("{:?}", exp);
	Err(ParseError::new("parser did not consume all tokens"))
}

// for all of these parsing functions, the order of tokens should be reversed

fn expression(tokens: &mut Vec<Token>) -> Result<Expression, ParseError> {
	// parses an expression in a vector of tokens
	term(tokens)
}

fn term(tokens: &mut Vec<Token>) -> Result<Expression, ParseError> {
	// parses a term in a vector of tokens
	let mut res_inter = factor(tokens)?;

	while let Some(x) = tokens.pop() {
		match x {
			Token::AddOp(c) => {
				// recurse, generating a nested expression
				let res_lower = factor(tokens)?;
				// return the obtained expression
				res_inter = Expression::Binary(Box::new(res_inter), c, Box::new(res_lower));
			}
			_ => {
				// replace the removed token and return
				tokens.push(x);
				return Ok(res_inter)
			}
		}
	}

	// out of tokens - return the current expression
	Ok(res_inter)
}

fn factor(tokens: &mut Vec<Token>) -> Result<Expression, ParseError> {
	let mut res_inter = quantity(tokens)?;

	while let Some(x) = tokens.pop() {
		// same overall logic as term()
		match x {
			Token::FactOp(c) => {
				let res_lower = quantity(tokens)?;
				res_inter = Expression::Binary(Box::new(res_inter), c, Box::new(res_lower))
			}
			_ => {
				tokens.push(x);
				return Ok(res_inter)
			}
		}
	}
	Ok(res_inter)
}

fn quantity(tokens: &mut Vec<Token>) -> Result<Expression, ParseError> {
	// consume a primary plus an optional unit_pow
	// TODO allow arbitrarily many unit_pows
	let mut res_inter = primary(tokens)?;

	// consume any trailing unit_pows
	loop {
		match tokens.last() {
			None => break Ok(res_inter),
			Some(Token::UnitString(_)) => {
				// next expression is a unit_pow
				if let Some(res_unit) = unit_pow(tokens) {
					res_inter = Expression::Quantity(Box::new(res_inter), Box::new(res_unit));
				}
				else { break Err(ParseError::new("began with unitstring but wasn't unit_pow")) }
			}
			_ => break Ok(res_inter)
		}
	}
}

fn unit_pow(tokens: &mut Vec<Token>) -> Option<Expression> {
	let cur = tokens.pop();
	if let Some(Token::UnitString(s)) = cur {
		let pow = tokens.pop();
		match pow {
			Some(Token::Integer(i)) => {
				// return a unit with a power
				return Some(Expression::UnitPow(s, i))
			}
			None => {
				// no more tokens - return a bare unit
				return Some(Expression::Unit(s))
			}
			_ => {
				// non-power token - push pow and return a bare unit
				tokens.push(pow.unwrap());
				return Some(Expression::Unit(s))
			}
		}
	}
	// can't consume a unit
	None
}

fn primary(tokens: &mut Vec<Token>) -> Result<Expression, ParseError> {
	// parse a primary value, recursively calling expression if ()s are encountered
	match tokens.pop() {
		Some(Token::Integer(i)) => Ok(Expression::PrimaryInt(i)),
		Some(Token::Float(f)) => Ok(Expression::PrimaryFloat(f)),
		Some(Token::OpenParen) => {
			// parse the expression, consume the trailing close paren
			let res_lower = expression(tokens)?;
			if let Some(Token::CloseParen) = tokens.pop() {
				Ok(res_lower)
			}
			else { Err(ParseError::new("primary: expected )")) }
		}
		Some(Token::UnitString(s)) => {
			// return a unit_pow after returning the popped value
			tokens.push(Token::UnitString(s));
			Ok(unit_pow(tokens).unwrap()) // unwrap is ok because the none state is unreachable
		}
		None => Err(ParseError::new("primary: ran out of tokens")),
		_ => Err(ParseError::new("primary: unexpected token")),
	}
}