use std::fs;
use std::io;
use std::io::Write;
use std::process;

pub struct Lox {}
impl Lox {
    pub fn run_file(file_name: String) {
        let file_contents = fs::read_to_string(file_name.clone())
            .expect(&format!("Unable to read file {}", file_name));
        Lox::run(&file_contents)
    }

    pub fn run_prompt() {
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
                    Lox::run(&input); // Use of clone here.. Needed?
                    input.clear();
                }
                Err(e) => {
                    println!("Some Error occured {}, exitting...", e);
                    process::exit(1);
                }
            };
        }
    }

    fn run(input: &String) {
        println!("{}", input.trim());
    }
}
