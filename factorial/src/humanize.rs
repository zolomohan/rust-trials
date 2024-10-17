pub fn number(number: &usize) -> String {
    let number_string = number.to_string();
    let mut humanized_string = String::new();

    for (i, c) in number_string.chars().rev().enumerate() {
        humanized_string.push(c);

        if ((i + 1) % 3 == 0) {
            humanized_string.push(',');
        }
    }

    return humanized_string.chars().rev().collect();
}
