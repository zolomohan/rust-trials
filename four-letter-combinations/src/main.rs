fn reverse_string(word: &str) -> String {
    word.chars().rev().collect()
}

fn permutations(word: String) -> Vec<String> {
    if word.len() == 2 {
        return vec![word.clone(), reverse_string(&word)];
    }

    let mut all_permutations = vec![];

    for (i, c) in word.chars().enumerate() {
        let sub_permutations: Vec<String> =
            permutations(format!("{}{}", &word[0..i], &word[(i + 1)..]))
                .iter()
                .map(|sub_word| format!("{}{}", c, sub_word))
                .collect();

        all_permutations.extend(sub_permutations);
    }

    return all_permutations;
}

fn main() {
    println!("Enter a word (max 6 letters): ");

    let mut input_text = String::new();
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read input");

    let mut all_perms = permutations(input_text.trim().to_string());
    all_perms.sort();
    all_perms.dedup();

    println!("All Permutations ({}):", all_perms.len());
    println!("{:?}", all_perms);
}
