mod humanize;
use num_bigint::BigUint;

fn factorial_bigint(num: usize) -> BigUint {
    (1..num + 1).fold(BigUint::new(vec![1]), |acc, el| acc * el)
}

fn main() {
    let mut input: String = String::new();
    println!("Enter number to calculate factorial: ");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong during input.");

    let num: usize = input
        .trim()
        .parse::<usize>()
        .expect("Something went wrong during parsing input.");

    let res: BigUint = factorial_bigint(num);

    println!("The factorial of {} is {}", num, humanize::big_int(&res));
}
