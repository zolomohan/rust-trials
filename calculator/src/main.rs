mod calc;
mod io_custom;

fn main() {
    loop {
        let operation = io_custom::get_string("What operation would you like? (+, -, *, /, %): ");
        let num1: f64 = io_custom::get_number("Enter first number: ");
        let num2: f64 = io_custom::get_number("Enter second number: ");

        match operation.chars().nth(0).unwrap() {
            '+' => {
                println!("Result: {}", calc::Calc::add(num1, num2));
            }

            '-' => {
                println!("Result: {}", calc::Calc::sub(num1, num2));
            }

            '*' => {
                println!("Result: {}", calc::Calc::mul(num1, num2));
            }

            '/' => {
                println!("Result: {}", calc::Calc::div(num1, num2));
            }

            '%' => {
                println!("Result: {}", calc::Calc::modulus(num1, num2));
            }

            _ => {
                println!("That operation is not supported!")
            }
        }
    }
}
