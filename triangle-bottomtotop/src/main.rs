fn get_num_from_io(prompt: &str) -> u32 {
    let mut text = String::from("");

    println!("{}", prompt);

    std::io::stdin()
        .read_line(&mut text)
        .expect("Something went wrong during I/O.");

    let num = text
        .trim()
        .parse::<u32>()
        .expect("Something went wrong during I/O parsing.");

    return num;
}

fn main() {
    let num: u32 = get_num_from_io("Enter number:");

    for i in 0..num {
        println!(
            "{}",
            (0..(num - i)).fold(String::new(), |acc, _| format!("{acc}*"))
        );
    }
}
