use convert::lexer::lex;
use convert::parser::parse;
use convert::evaluator::evaluate;
use convert::units::SiValue;

use std::env;
use std::string::String;
use std::io;
use std::error;
use std::fmt;

#[derive(Clone, Debug)]
struct ConvertError {
	message: String,
}

impl ConvertError {
	fn new(x: &str) -> ConvertError {
		ConvertError {message: format!("convert_error: {}", String::from(x))}
	}
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<T: error::Error> From<T> for ConvertError {
	fn from(e: T) -> Self {
		ConvertError { message: format!("convert_error: {}", e) }
	}
}

fn run(input: &str) -> Result<SiValue, ConvertError> {
	let toks = lex(input)?;
	// println!("lexed tokens:\n{:?}", toks);
	let exp = parse(toks)?;
	// println!("parsed expression:\n{:?}", exp);
	Ok(evaluate(exp)?)
}

fn main() -> Result<(), ConvertError> {
	let args: Vec<String> = env::args().collect();

	if args.len() != 2 { 
		return Err(ConvertError::new("please supply exactly one argument (-i for interactive mode)")) 
	}

	if args.iter().any(|i| i=="-i") {
    	// interative mode - accept input and evaluate in a loop
    	loop {
    		println!("enter expression ('break' to terminate):");
    		let mut input = String::new();
    		io::stdin()
    			.read_line(&mut input)
    			.expect("read failure");
    		let input = input.trim();
    		if input == "break" {
    			break
    		}
    		println!("result is:\n{}", run(input)?);
    	}
	}
	else {
		let input = args.get(1).unwrap();
		println!("result is:\n{}", run(input)?);
	}

	Ok(())
}