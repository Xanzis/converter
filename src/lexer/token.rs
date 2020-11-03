use std::string::String;
use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct FloatInProg {
	pub value: f64,
	depth: i64,
}

#[derive(Debug, PartialEq)]
pub enum Token {
	UnitString(String),
	Integer(i64),
	Float(f64),
	FloatIp(FloatInProg),
	FactOp(char), // * /
	AddOp(char), // + -
	OpenParen, // (
	CloseParen, // )
	NoToken,
}

impl Token {
	pub fn is_continuation(&self, c: char) -> bool {
		// returns whether c is a valid continuation of the token
		match self {
			Token::UnitString(_) => c.is_alphabetic(),
			Token::Integer(_) => c.is_digit(10) || (c == '.'), // will become float is + is used
			Token::FloatIp(_) => c.is_digit(10), // no . allowed if value is already float
			Token::FactOp(_) | Token::AddOp(_) | Token::OpenParen | Token::CloseParen => false,
			_ => panic!("unexpected token")
		}
	}
}

impl From<char> for Token {
	fn from(c: char) -> Self {
		match c {
			'a' ..= 'z' | 'A' ..= 'Z' => Token::UnitString(String::from(c)),
			'0' ..= '9' => Token::Integer(c.to_digit(10).expect("bad digit??") as i64),
			'+' | '-' => Token::AddOp(c),
			'*' | '/' => Token::FactOp(c),
			'(' => Token::OpenParen,
			')' => Token::CloseParen,
			_ => panic!("bad character: {}", c)
		}
	}
}

impl Add<char> for Token {
	type Output = Token;

	fn add(self, rhs: char) -> Token {
		match self {
			Token::UnitString(x) => Token::UnitString(x + &rhs.to_string()),

			Token::Integer(x) => {
				// shift and add for digits, convert to float for .
				if let Some(r) = rhs.to_digit(10) { Token::Integer((x * 10) + (r as i64)) }
				else if rhs == '.' { Token::FloatIp(FloatInProg {value: x as f64, depth: 1}) }
				else { panic!("bad continuation of Integer: {}", rhs); }
			}

			Token::FloatIp(x) => {
				// add the to value in x at depth specified by x
				let base: i64 = 10;
				let oom: f64 = 1.0 / (base.pow(x.depth as u32) as f64);
				if let Some(r) = rhs.to_digit(10) {
					let inc = oom * (r as f64);
					Token::FloatIp(FloatInProg {value: x.value + inc, depth: x.depth + 1})
				}
				else { panic!("bad continuation of Float: {}", rhs); }
			}

			_ => panic!("invalid enum variant for method add")
		}
	}
}