pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {
	use crate::lexer::Token::*;
	use crate::lexer::lex;
	use crate::parser::Expression::*;
	use crate::parser::parse;

	#[test]
	fn simple_lex() {
		let lex_str = "3/4.7+2";
		let exp = vec![Integer(3), FactOp('/'), Float(4.7), AddOp('+'), Integer(2)];
		assert_eq!(lex(lex_str).unwrap(), exp);
	}

	#[test]
	fn string_lex() {
		let lex_str = "3 lbf+  2";
		let exp = vec![Integer(3), UnitString("lbf".to_string()), AddOp('+'), Integer(2)];
		assert_eq!(lex(lex_str).unwrap(), exp);
	}

	#[test]
	fn simple_parse() {
		let in_str = "3 + 0.27";
		let lexed = lex(in_str).unwrap();
		let parsed = parse(lexed).unwrap();
		let exp = Binary(Box::new(PrimaryInt(3)), '+', Box::new(PrimaryFloat(0.27)));
		assert_eq!(parsed, exp);
	}

	#[test]
	fn paren_parse() {
		let in_str = "1 / (2 + 3)";
		let lexed = lex(in_str).unwrap();
		let parsed = parse(lexed).unwrap();
		let lower = Binary(Box::new(PrimaryInt(2)), '+', Box::new(PrimaryInt(3)));
		let exp = Binary(Box::new(PrimaryInt(1)), '/', Box::new(lower));
		assert_eq!(exp, parsed);
	}
}