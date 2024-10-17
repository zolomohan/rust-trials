const INR_TO_USD: f32 = 0.0125;
const USD_TO_INR: f32 = 80.0;

fn get_num_from_io(prompt: &str) -> u32 {
    println!("{prompt}");

    let mut text_from_io: String = String::from("");

    std::io::stdin()
        .read_line(&mut text_from_io)
        .expect("Something went wrong while getting user input.");

    let option_num: u32 = text_from_io
        .trim()
        .parse::<u32>()
        .expect("That's not a number!");

    return option_num;
}

fn main() {
    println!("Currency Converter\n");
    println!("1) INR to USD");
    println!("2) USD to INR");

    let option = get_num_from_io("Enter Option (number): ");

    match option {
        1 => {
            let amount = get_num_from_io("Enter Amount: ");
            println!("₹{} = ${}", amount, amount as f32 * INR_TO_USD);
        }

        2 => {
            let amount = get_num_from_io("Enter Amount: ");
            println!("${} = ₹{}", amount, amount as f32 * USD_TO_INR);
        }

        _ => {
            println!("Enter a valid option!");
        }
    }
}
