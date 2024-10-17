mod humanize;

fn factorial(number: usize) -> usize {
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

    let number: usize = user_input
        .trim()
        .parse::<usize>()
        .expect("Something went wrong during parsing input.");

    let result: usize = factorial(number);

    println!(
        "The factorial of {} is {}",
        number,
        humanize::number(&result)
    );
}
