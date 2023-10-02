mod lox;
mod token;
mod scanner;
use lox::Lox;
use std::env;

fn main() {
    // The first args[0] is going to be /target/*/lox
    let args: Vec<String> = env::args().collect();

    let mut lox = Lox::new();
    if args.len() == 1 {
        lox.run_prompt()
    } else if args.len() == 2 {
        lox.run_file(args.get(1).unwrap().clone());
    } else {
        println!("More than 1 arg was given, can't proceed, exitting...");
    }
}
