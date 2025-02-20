use std::io::{self, Write};

mod token;
mod scanner;
mod parser;
mod expression;
mod ast_printer;
mod interpreter;
use crate::scanner::Scanner;
use crate::parser::Parser;
use crate::ast_printer::AstPrinter;
use crate::interpreter::Interpreter;

fn run_prompt() {
    let mut buffer = String::new();
    loop {
        io::stdout().write(b"> ").unwrap();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();
        io::stdout().write(format!("You type: {buffer}").as_bytes()).unwrap();
        run(&buffer);
        buffer.clear();
    }
}

fn run(source_code: &str) {
    let mut scanner = Scanner::new(source_code);
    scanner.scan_tokens();
    println!("Tokens: {:?}", scanner.tokens);
    
    let mut parser = Parser::new(scanner.tokens);
    let expr = parser.parse();
    println!("expr: {:?}", expr);

    let ast_printer = AstPrinter{};
    let text = ast_printer.print(&expr);
    println!("ast: {}", text);

    let interpreter = Interpreter{};
    let eval = interpreter.interpret(&expr);
    println!("interpreter: {}", eval);
}

fn main() {
    run_prompt();
}
