use calculator::eval_expression;
use std::io::{self, Write};

fn main() {
    println!("Enter an expression to evaluate.");
    println!("NOTE: All tokens must be separated by spaces.");
    loop {
        let mut input = String::new();
        print!("-> ");
        let _ = io::stdout().flush(); // Ensure that -> is printed
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let exp = input.trim();
        println!(" = {}", eval_expression(exp));
    }
}
