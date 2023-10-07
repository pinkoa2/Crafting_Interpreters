use std::fs;
use std::io;
use std::io::Write;
use std::process;

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
        println!("{}", input.trim());
        Ok("Not yet connected".to_string())
    }

    pub fn error(line: usize, message: String) {
        Lox::report(line, String::from(""), message);
    }

    fn report(line: usize, context: String, message: String) {
        println!("[line {}] Error {} {}", line, context, message);
    }
}
