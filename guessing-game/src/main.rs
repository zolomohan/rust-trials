use rand::{thread_rng, Rng};

fn prompt_and_check(&num_to_guess: &i32) {
    println!("Enter your guess:");

    let mut user_guess = String::from("");

    std::io::stdin()
        .read_line(&mut user_guess)
        .expect("Something went wrong during I/O.");

    let user_guess_num: i32 = user_guess
        .trim()
        .parse::<i32>()
        .expect("Something went wrong durung parsing I/O.");

    if (user_guess_num == num_to_guess) {
        println!("You guessed it correct!");
        return ();
    } else if (user_guess_num < num_to_guess) {
        println!("Your guess is lower than the number.");
    } else {
        println!("Your guess is higher than the number.");
    }

    prompt_and_check(&num_to_guess);
}

fn main() {
    let mut rng = thread_rng();
    let num_to_guess: i32 = rng.gen_range(1..101);

    prompt_and_check(&num_to_guess);
}
