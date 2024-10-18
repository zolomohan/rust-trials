mod io_custom;

struct Coin {
    value: f64,
    amount: f64,
}

fn main() {
    let two_dollar_coins: f64 = io_custom::get_number("Enter No. of $2 coins: ");
    let one_dollar_coins: f64 = io_custom::get_number("Enter No. of $1 coins: ");
    let fifty_cents_coins: f64 = io_custom::get_number("Enter No. of 50c coins: ");
    let twenty_cents_coins: f64 = io_custom::get_number("Enter No. of 20c coins: ");
    let ten_cents_coins: f64 = io_custom::get_number("Enter No. of 10c coins: ");
    let five_cents_coins: f64 = io_custom::get_number("Enter No. of 5c coins: ");

    let coins: Vec<Coin> = vec![
        Coin {
            value: 200.0,
            amount: two_dollar_coins,
        },
        Coin {
            value: 100.0,
            amount: one_dollar_coins,
        },
        Coin {
            value: 50.0,
            amount: fifty_cents_coins,
        },
        Coin {
            value: 20.0,
            amount: twenty_cents_coins,
        },
        Coin {
            value: 10.0,
            amount: ten_cents_coins,
        },
        Coin {
            value: 5.0,
            amount: five_cents_coins,
        },
    ];

    let total_money: f64 = coins
        .iter()
        .fold(0.0, |acc, el| acc + (el.amount * el.value));

    println!("Total: ${}", total_money as f64 / 100.0);
}
