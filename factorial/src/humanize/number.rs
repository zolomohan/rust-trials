use num_traits::ToPrimitive;

/// Converts a number into a human-readable string with commas as thousand separators.
///
/// # Type Parameters
/// - `T`: A type that implements the `ToPrimitive` trait, which allows conversion to primitive numeric types.
///
/// # Arguments
/// - `number`: A reference to a number of type `T` that implements `ToPrimitive`.
///
/// # Returns
/// - `String`: A string representation of the number with commas separating every three digits.
///
/// # Panics
/// This function will panic if the number cannot be converted to a `usize` (e.g., if it's too large or negative).
///
/// # Example
/// ```
/// let num = 123456789;
/// let result = _number(&num);
/// assert_eq!(result, "123,456,789");
/// ```
pub fn number<T: ToPrimitive>(number: &T) -> String {
    // Convert the input number to a usize (unsigned size type) and handle any errors during conversion
    let number_string = number
        .to_usize() // Convert the number to a usize type
        .expect("Something went wrong during parsing the result") // Panic if conversion fails
        .to_string(); // Convert the usize to a string

    // Initialize an empty String to hold the human-readable version of the number
    let mut humanized_string = String::new();

    // Iterate through each character of the number string in reverse order (starting from the last digit)
    for (i, c) in number_string.chars().rev().enumerate() {
        humanized_string.push(c);

        // Insert a comma after every third digit, unless it's the last group of digits
        if (i + 1) % 3 == 0 && number_string.len() != (i + 1) {
            humanized_string.push(',');
        }
    }

    // Reverse the humanized string back to the correct order and return it
    return humanized_string.chars().rev().collect();
}
