fn factorial(number: usize) -> usize {
    if number == 1 {
        return 1;
    }

    return number * factorial(number - 1);
}

fn humanize(&number: &usize) -> String {
    let number_string = number.to_string();

    println!("{:#?}", number_string.chars().into_iter());

    return number_string;
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

    println!("The factorial of {} is {}", number, result);
    println!("The factorial of {} is {}", number, humanize(&result));
}
