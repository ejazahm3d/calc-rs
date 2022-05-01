use calc::{CalcError, Calculator};

fn main() -> Result<(), CalcError> {
    println!("\nHello, world! Write any mathematical expression\n");

    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        if let Ok(tokens) = Calculator::parse(input) {
            let expr = Calculator::expression(tokens);

            let result = Calculator::evaluate(expr);
            println!("Answer is: {}", result.unwrap());
        } else {
            println!("Invalid Input");
        }
    }
}
