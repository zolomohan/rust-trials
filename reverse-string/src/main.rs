fn reverse_print_string(text: &str) {
    let last_character = text.chars().last();

    match last_character {
        Some(c) => {
            println!("{:?}", c);
            reverse_print_string(&text[0..text.len() - 1]);
        }

        None => {
            return;
        }
    }
}

fn main() {
    let mut text = String::new();

    std::io::stdin()
        .read_line(&mut text)
        .expect("Something went wrong during I/O.");

    reverse_print_string(&text.trim());
}
