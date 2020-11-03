use crate::lexer::Token;
use std::fmt;

// Using the following grammar:
// expression -> term
// term       -> factor (("-" | "+") term)?
// factor     -> quantity (("*" | "/") factor)?
// quantity   -> primary unit_pow?
// primary    -> i64 | f64 | "(" expression ")"
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

#[derive(Clone)]
pub struct ParseError {
	message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error in parser")
    }
}
impl fmt::Debug for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error in parser: {}", self.message)
    }
}

pub fn parse(mut input: Vec<Token>) -> Result<Expression, ParseError> {
	// takes a vector of tokens, parses into a single nested expression
	input.reverse();
	let (fin_tokens, exp) = expression(input);
	// all tokens should have been consumed
	if fin_tokens.len() == 0 {
		return Ok(exp)
	}
	println!("{:?}", exp);
	Err(ParseError {message: "parser did not consume all tokens".to_string()})
}

// for all of these parsing functions, the order of tokens should be reversed

fn expression(tokens: Vec<Token>) -> (Vec<Token>, Expression) {
	// parses an expression in a vector of tokens
	// returns the expression and the remaining tokens
	let (new_tokens, res) = term(tokens);
	(new_tokens, res)
}

fn term(tokens: Vec<Token>) -> (Vec<Token>, Expression) {
	// parses a term in a vector of tokens
	// returns the term as an Expression and the remaining tokens
	let (mut tokens, res_inter) = factor(tokens);

	if let Some(x) = tokens.pop() {
		match x {
			Token::AddOp(c) => {
				// recurse, generating a nested expression
				let (tokens, res_lower) = term(tokens);
				// return the obtained expression
				(tokens, Expression::Binary(Box::new(res_inter), c, Box::new(res_lower)))
			}
			_ => {
				// replace the removed token and return
				tokens.push(x);
				(tokens, res_inter)
			}
		}
	}
	else {
		// tokens was empty
		(tokens, res_inter)
	}
}

fn factor(tokens: Vec<Token>) -> (Vec<Token>, Expression) {
	let (mut tokens, res_inter) = quantity(tokens);

	if let Some(x) = tokens.pop() {
		// same overall logic as term()
		match x {
			Token::FactOp(c) => {
				let (tokens, res_lower) = quantity(tokens);
				(tokens, Expression::Binary(Box::new(res_inter), c, Box::new(res_lower)))
			}
			_ => {
				tokens.push(x);
				(tokens, res_inter)
			}
		}
	}
	else {
		(tokens, res_inter)
	}
}

fn quantity(tokens: Vec<Token>) -> (Vec<Token>, Expression) {
	// consume a primary plus an optional unit_pow
	// TODO allow arbitrarily many unit_pows
	let (tokens, res_inter) = primary(tokens);

	match tokens.last() {
		None => (tokens, res_inter),
		Some(Token::UnitString(_)) => {
			// next expression is a unit_pow
			if let Some((tokens, res_unit)) = unit_pow(tokens) {
				return (tokens, Expression::Quantity(Box::new(res_inter), Box::new(res_unit)))
			}
			else { panic!("began with unitstring but wasn't unit_pow") }
		}
		_ => (tokens, res_inter)
	}
}

fn unit_pow(mut tokens: Vec<Token>) -> Option<(Vec<Token>, Expression)> {
	let cur = tokens.pop();
	if let Some(Token::UnitString(s)) = cur {
		let pow = tokens.pop();
		match pow {
			Some(Token::Integer(i)) => {
				// return a unit with a power
				return Some((tokens, Expression::UnitPow(s, i)))
			}
			None => {
				// no more tokens - return a bare unit
				return Some((tokens, Expression::Unit(s)))
			}
			_ => {
				// non-power token - push pow and return a bare unit
				tokens.push(pow.unwrap());
				return Some((tokens, Expression::Unit(s)))
			}
		}
	}
	// can't consume a unit
	None
}

fn primary(mut tokens: Vec<Token>) -> (Vec<Token>, Expression) {
	// parse a primary value, recursively calling expression if ()s are encountered
	match tokens.pop() {
		Some(Token::Integer(i)) => (tokens, Expression::PrimaryInt(i)),
		Some(Token::Float(f)) => (tokens, Expression::PrimaryFloat(f)),
		Some(Token::OpenParen) => {
			// parse the expression, consume the trailing close paren
			let (mut tokens, res_lower) = expression(tokens);
			match tokens.pop() {
				Some(Token::CloseParen) => (),
				_ => panic!("primary: expected )"),
			}
			(tokens, res_lower)
		}
		None => panic!("primary: ran out of tokens"),
		_ => panic!("primary: unexpected token"),
	}
}