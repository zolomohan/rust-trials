use std::cmp::Ordering;

use rand::{thread_rng, Rng};

fn get_number_from_io(prompt: &str) -> i32 {
    println!("{}", prompt);

    let mut text = String::from("");

    std::io::stdin()
        .read_line(&mut text)
        .expect("Something went wrong during I/O.");

    let num: i32 = text
        .trim()
        .parse::<i32>()
        .expect("Something went wrong durung parsing I/O.");

    return num;
}

fn main() {
    let num_to_guess: i32 = thread_rng().gen_range(1..101);
    let mut correct = false;
    let mut no_of_guesses = 0;

    while !correct {
        let user_guess = get_number_from_io("Guess the Number: ");
        no_of_guesses += 1;

        match (num_to_guess - user_guess).abs() {
            0 => println!("Correct!"),
            1..10 => println!("Almost there!"),
            10..20 => println!("Pretty close!"),
            _ => println!("You're way off!"),
        }

        match user_guess.cmp(&num_to_guess) {
            Ordering::Less => println!("You're guess is lower :("),
            Ordering::Greater => println!("You're guess is higher :("),
            Ordering::Equal => {
                println!("You guessed the number in {} tries!", no_of_guesses);
                correct = true;
            }
        }
    }
}
