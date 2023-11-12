use std::fs;
use std::io;
use std::io::Write;
use std::process;

use crate::expressions::expression::Expression;
use crate::expressions::printer::Printer;
use crate::interpreter::interpreter::Interpreter;
use crate::parser::parser::Parser;
use crate::scanner::scanner::Scanner;
use crate::token::token::Token;
use crate::token::token_type::Token_Type;

pub struct Lox {}

#[allow(dead_code)]
impl Lox {
    pub fn new() -> Lox {
        return Lox {};
    }

    pub fn run_file(&mut self, file_name: String) {
        let file_contents = fs::read_to_string(file_name.clone())
            .expect(&format!("Unable to read file {}", file_name));
        match self.run(&file_contents) {
            Ok(_) => {}
            Err(_) => process::exit(65),
        }
    }

    pub fn run_prompt(&mut self) {
        let stdin = io::stdin();
        let mut input = String::new();

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            match stdin.read_line(&mut input) {
                Ok(n) => {
                    if n == 0 {
                        process::exit(1);
                    }
                    self.run(&input).ok(); // We do not care about errors
                    input.clear();
                }
                Err(e) => {
                    println!("Some Error occured {}, exitting...", e);
                    process::exit(1);
                }
            };
        }
    }

    fn run(&mut self, input: &String) -> Result<String, String> {
        let mut scanner = Scanner::new(input);
        let tokens: Vec<Token> = match scanner.scan_tokens() {
            Ok(tokens) => tokens,
            Err(tokens) => {
                println!("Ran into some scanner issues, continuing");
                tokens
            }
        };

        let mut parser = Parser::new(&tokens);
        let expr: Box<dyn Expression> = match parser.parse() {
            Ok(expr) => expr,
            Err(m) => return Err(m),
        };

        let interpreter = Interpreter {};
        let result = match interpreter.evaluate(&expr) {
            Ok(literal) => literal,
            Err(m) => return Err(m),
        };

        let printer = Printer {};
        let string: String = printer.convert(Box::new(result));
        println!("{}", string);
        Ok(string)
    }

    pub fn error(line: usize, message: &str) {
        Lox::report(line, "", message);
    }

    pub fn error_token(token: &Token, message: &str) {
        if token.token_type == Token_Type::EOF {
            Lox::report(token.line, " at end", message);
            return;
        }
        Lox::report(token.line, &format!(" at '{}'", token.lexem), message);
    }

    fn report(line: usize, context: &str, message: &str) {
        println!("[line {}] Error {} {}", line, context, message);
    }
}
