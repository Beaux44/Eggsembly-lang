mod lexer;
mod parser;
mod compiler;

use std::{env, fs};
use lexer::Lexer;
use parser::Parser;
use compiler::Compiler;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }

    let input = fs::read_to_string(args[1].to_owned()).unwrap();

    let mut lexer = Lexer::new(&input);
    let toks: Vec<_> = lexer.into_iter().collect();
    println!("Tokens: {:?}", toks);

    let mut lexer = Lexer::new(&input);
    let parser = Parser::new(&mut lexer);
    let ast = parser.parse();
    println!("AST:\n{:#?}\n", ast);

    let compiler = Compiler::new();
    let code = compiler.compile(&ast);
    println!("Bytecode: {:?}", code);
}


