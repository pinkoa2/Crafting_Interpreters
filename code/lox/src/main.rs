struct Lox {}

impl Lox {
    fn test(&self) {
        println!("Test");
    }
}

fn main() {
    let lox = Lox {};
    lox.test()
}
