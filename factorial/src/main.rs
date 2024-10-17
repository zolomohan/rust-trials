mod humanize;
use num_bigint::BigUint;

fn factorial_bigint(number: usize) -> BigUint {
    let mut result: BigUint = BigUint::new(vec![1]);

    for n in 0..number {
        result = result * (number - n);
    }

    return result;
}

fn main() {
    let mut user_input: String = String::new();
    println!("Enter number to calculate factorial: ");

    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Something went wrong during input.");

    let number: usize = user_input
        .trim()
        .parse::<usize>()
        .expect("Something went wrong during parsing input.");

    let result: BigUint = factorial_bigint(number);

    println!(
        "The factorial of {} is {}",
        number,
        humanize::big_int(&result)
    );
}
