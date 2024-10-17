fn factorial(number: u64) -> u64 {
    if number == 1 {
        return 1;
    }

    return number * factorial(number - 1);
}

fn main() {
    let mut user_input: String = String::new();
    println!("Enter number to calculate factorial: ");

    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Something went wrong during input.");

    let number: u64 = user_input
        .trim()
        .parse::<u64>()
        .expect("Something went wrong during parsing input.");

    println!("The factorial of {} is {}", number, factorial(number));
}
