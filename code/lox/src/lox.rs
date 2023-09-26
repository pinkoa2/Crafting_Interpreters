use std::fs;
use std::io;
use std::io::Write;
use std::process;

#[allow(dead_code)]
pub struct Lox {
    had_error: bool,
}

#[allow(dead_code)]
impl Lox {
    pub fn new() -> Lox {
        return Lox { had_error: false };
    }

    pub fn run_file(&mut self, file_name: String) {
        let file_contents = fs::read_to_string(file_name.clone())
            .expect(&format!("Unable to read file {}", file_name));
        self.run(&file_contents);
        if self.had_error {
            process::exit(65);
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
                    self.run(&input);
                    input.clear();
                }
                Err(e) => {
                    println!("Some Error occured {}, exitting...", e);
                    process::exit(1);
                }
            };
            self.had_error = false;
        }
    }

    fn run(&mut self, input: &String) {
        println!("{}", input.trim());
    }

    fn error(&mut self, line: i32, message: String) {
        self.report(line, String::from(""), message);
    }

    fn report(&mut self, line: i32, context: String, message: String) {
        println!("[line {}] Error {} {}", line, context, message);
        self.had_error = true;
    }
}
