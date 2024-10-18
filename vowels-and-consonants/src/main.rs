fn main() {
    let mut text = String::from("");

    println!("Enter text: ");

    std::io::stdin()
        .read_line(&mut text)
        .expect("Something went wrong during I/O.");

    let mut vowel_count = 0;
    let mut consonant_count = 0;

    for c in text.chars() {
        if c.is_alphabetic() {
            if ['a', 'e', 'i', 'o', 'u'].contains(&c) {
                vowel_count += 1;
            } else {
                consonant_count += 1;
            }
        } else {
            break;
        }
    }

    println!("Vowel Count: {}", vowel_count);
    println!("Consonant Count: {}", consonant_count);
}
