use num_bigint::BigUint;
use num_traits::ToPrimitive;

pub fn number<T: ToPrimitive>(number: &T) -> String {
    let number_string = number
        .to_usize()
        .expect("Something went wront during parsing the result")
        .to_string();

    let mut humanized_string = String::new();

    for (i, c) in number_string.chars().rev().enumerate() {
        humanized_string.push(c);

        if ((i + 1) % 3 == 0 && number_string.len() != (i + 1)) {
            humanized_string.push(',');
        }
    }

    return humanized_string.chars().rev().collect();
}

pub fn big_int(number: &BigUint) -> String {
    let number_string = number.to_string();
    let mut humanized_string = String::new();

    for (i, c) in number_string.chars().rev().enumerate() {
        humanized_string.push(c);

        if ((i + 1) % 3 == 0 && number_string.len() != (i + 1)) {
            humanized_string.push(',');
        }
    }

    return humanized_string.chars().rev().collect();
}

pub fn big_int_eff(number: &BigUint) -> String {
    let mut number_string = number.to_string();

    let no_of_iterations = number_string.len() / 3;
    let mut next_comma_index = number_string.len() % 3;

    for _i in 0..no_of_iterations {
        number_string = format!(
            "{},{}",
            &number_string[0..next_comma_index],
            &number_string[next_comma_index..number_string.len()]
        );

        next_comma_index = next_comma_index + 4;
    }

    return number_string;
}
