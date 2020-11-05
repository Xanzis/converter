use convert::lexer::lex;
use convert::parser::parse;
use convert::evaluator::evaluate;
use convert::units::SiValue;

use std::env;
use std::string::String;
use std::io;

fn run(input: &str) -> SiValue {
	let toks = lex(input).unwrap();
	println!("lexed tokens:\n{:?}", toks);
	let exp = parse(toks).unwrap();
	println!("parsed expression:\n{:?}", exp);
	evaluate(exp)
}

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 2 { panic!("please supply exactly one argument (-i for interactive mode)"); }

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
    		println!("result is:\n{}", run(input));
    	}
	}
	else {
		let input = args.get(1).unwrap();
		println!("result is:\n{}", run(input));
	}
}