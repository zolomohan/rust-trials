use num_bigint::BigUint;

/// Efficiently formats a `BigUint` into a human-readable string with commas as thousand separators.
///
/// # Arguments
/// - `number`: A reference to a `BigUint` (unsigned big integer).
///
/// # Returns
/// - `String`: A formatted string representation of the big integer with commas separating every three digits.
///
/// # Example
/// ```
/// use num_bigint::BigUint;
///
/// let big_num = BigUint::parse_bytes(b"123456789123456789", 10).unwrap();
/// let result = big_int_eff(&big_num);
/// assert_eq!(result, "123,456,789,123,456,789");
/// ```
pub fn big_int_eff(number: &BigUint) -> String {
    // Convert the BigUint to a string
    let mut number_string = number.to_string();

    // Calculate the initial position for placing the first comma
    let mut next_comma_position = number_string.len() % 3;

    // Loop over the length of the number and insert commas at appropriate positions
    for _i in 0..number_string.len() / 3 {
        // Skip adding a comma if the position is at the start (i.e., when len() % 3 == 0)
        if next_comma_position == 0 {
            next_comma_position += 3; // Adjust comma position
            continue;
        }

        // Insert a comma at the calculated position
        number_string = format!(
            "{},{}",
            &number_string[0..next_comma_position], // Part before the comma
            &number_string[next_comma_position..number_string.len()]  // Part after the comma
        );

        // Move the comma position forward by 4 (3 digits + comma)
        next_comma_position += 4;
    }

    // Return the formatted string
    return number_string;
}
