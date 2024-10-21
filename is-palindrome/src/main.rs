pub fn get_string(prompt: &str) -> String {
    println!("{}", prompt);

    let mut text = String::from("");

    std::io::stdin()
        .read_line(&mut text)
        .expect("Something went wrong during I/O.");

    let num: String = text
        .trim()
        .parse()
        .expect("Something went wrong durung parsing I/O.");

    return num;
}

fn main() {
    let string = get_string("Enter Text: ");
    let mut is_palindrome = true;

    for (i, c) in string.chars().enumerate() {
        if c != string.chars().nth(string.len() - (i + 1)).unwrap() {
            is_palindrome = false;
            break;
        }
    }

    if (is_palindrome) {
        println!("This is a palindrome!");
    } else {
        println!("This is not a palindrome!");
    }
}
