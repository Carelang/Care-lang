mod lang;

use std::env;
//use std::fs;

use lang::lexer::*;
use lang::parser::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file: String = String::new();

    match args.len() {
        2 => (file = args[1].to_string()),
        _ => (),
    }

    let par: Parser = Parser::new();
    let lex: Lexer = Lexer::new(file, par);
    lex.print();
}