use num_bigint::BigUint;

/// Converts a `BigUint` number into a human-readable string with commas as thousand separators.
///
/// # Arguments
/// - `number`: A reference to a `BigUint` (unsigned big integer).
///
/// # Returns
/// - `String`: A string representation of the big integer with commas separating every three digits.
///
/// # Example
/// ```
/// use num_bigint::BigUint;
///
/// let big_num = BigUint::parse_bytes(b"123456789123456789", 10).unwrap();
/// let result = _big_int(&big_num);
/// assert_eq!(result, "123,456,789,123,456,789");
/// ```
pub fn _big_int(number: &BigUint) -> String {
    // Convert the BigUint to a string
    let number_string = number.to_string();

    // Initialize an empty string to build the human-readable version of the number
    let mut humanized_string = String::new();

    // Iterate through the number string in reverse order
    for (i, c) in number_string.chars().rev().enumerate() {
        humanized_string.push(c);

        // Insert a comma after every third digit, except for the last group of digits
        if (i + 1) % 3 == 0 && number_string.len() != (i + 1) {
            humanized_string.push(',');
        }
    }

    // Reverse the string back to its original order and return the result
    return humanized_string.chars().rev().collect();
}
