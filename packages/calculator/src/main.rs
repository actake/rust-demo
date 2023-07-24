use std::io::{self, Write};

use calculator::interpreter::Interpreter;

fn main() {
    let mut text = String::new();

    loop {
        print!("Calc> ");
        io::stdout().flush().expect("Flush clear failed");

        match io::stdin().read_line(&mut text) {
            Ok(_) => {
                print!("text: {}", text);
            }
            Err(_) => {
                print!("Invalid input");
            }
        }

        let mut interpreter = Interpreter::new(text.trim().to_string());
        interpreter
            .expr()
            .map(|result| {
                println!("Result: {}", result);
            })
            .expect("expr func error");

        text.clear()
    }
}
