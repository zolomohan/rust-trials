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
    let num = get_number_from_io("Enter Number to Generate Fibonacci Serires: ");

    let mut first = 0;
    let mut second = 1;

    print!("{} {} ", first, second);

    for _ in 0..num - 2 {
        let output = first + second;
        first = second;
        second = output;

        print!("{} ", output);
    }
}
