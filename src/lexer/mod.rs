pub mod token;

pub use token::{Token, TokenError};
use std::string::String;
use std::str;
use std::fmt;

//type Result<T> = std::result::Result<T, LexError>;
#[derive(Clone, Debug)]
pub struct LexError {
	message: String,
}
impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<TokenError> for LexError {
	fn from(e: TokenError) -> Self {
		// format will use tokenerror's display, which prepends "token_error"
		LexError { message: format!("lex_error: {}", e) }
	}
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
	// lex the input for future use in the parser

	let mut res: Vec<Token> = Vec::new();
	let mut ongoing_token: Option<Token> = None;

	for c in input.chars() {
		// clean out whitespace
		if c.is_whitespace() {continue;}

		if let Some(ot) = ongoing_token {
			if ot.is_continuation(c) {
				// add to the ongoing token, skip to next loop
				ongoing_token = Some((ot + c)?);
				continue;
			}
			else {
				// push the completed token, clear it
				// next match will deal with creating the new token
				res.push(ot);
				ongoing_token = None;
			}
		}

		// make a new token, push it if it's a one-character token
		// otherwise, set ongoing_token and loop back
		let temp_token = Token::from(c);
		match temp_token {
			Token::FactOp(_) | Token::AddOp(_) | Token::OpenParen | Token::CloseParen => { 
				res.push(temp_token); 
			},
			_ => {
				ongoing_token = Some(temp_token);
			}
		}
	}

	if let Some(ot) = ongoing_token {
		res.push(ot);
	}

	// swap out FloatIp for Float
	for (_, e) in res.iter_mut().enumerate() {
		if let Token::FloatIp(f) = e {
			*e = Token::Float(f.value);
		}
	}

	Ok(res)
}