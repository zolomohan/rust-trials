use num_traits::Float;
use std::str::FromStr;

pub fn get_number<T: Float + FromStr>(prompt: &str) -> T
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    println!("{}", prompt);

    let mut text = String::from("");

    std::io::stdin()
        .read_line(&mut text)
        .expect("Something went wrong during I/O.");

    let num: T = text
        .trim()
        .parse()
        .expect("Something went wrong durung parsing I/O.");

    return num;
}

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
