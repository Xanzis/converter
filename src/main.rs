mod lexer;

use lexer::lex;

fn main() {
    println!("Let's try ... 3 / 4*5.8");
    println!("Got this:\n{:?}", lex("3 / 4*5.8").unwrap());
}