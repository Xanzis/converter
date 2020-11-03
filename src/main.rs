use convert::lexer::lex;
use convert::parser::parse;

fn main() {
    println!("Let's try ... 3 / 4*5.8");
    let toks = lex("3 / 4*5.8").unwrap();
    println!("Got this:\n{:?}", toks);
    let exp = parse(toks);
    println!("Expression is:\n{:?}", exp);
}