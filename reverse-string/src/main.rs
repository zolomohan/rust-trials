fn reverse_print_string(input: &str, output: &mut String) {
    let last_character = input.chars().last();

    match last_character {
        Some(c) => {
            output.push(c);
            reverse_print_string(&input[0..input.len() - 1], output);
        }

        None => {
            return;
        }
    }
}

fn main() {
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong during I/O.");

    let mut output = String::new();
    reverse_print_string(&input.trim(), &mut output);

    println!("{}", output);
}
