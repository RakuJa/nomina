use capitalize::Capitalize;
use nanorand::{Rng, WyRand};
use std::collections::HashMap;

/// Builds a Markov chain of characters from a list of names.
/// `order` determines how many characters to use as context (e.g., 2 = bi-gram).
#[must_use]
pub fn build_chain(names: &[&str], order: usize) -> HashMap<String, Vec<char>> {
    let mut chain: HashMap<String, Vec<_>> = HashMap::new();

    for &name in names {
        let padded = format!("{}{}", "^".repeat(order), name.to_lowercase());
        let chars: Vec<char> = padded.chars().collect();

        for window in chars.windows(order + 1) {
            let (key_slice, next) = window.split_at(order);
            let key = key_slice.iter().collect();
            chain.entry(key).or_default().push(next[0]);
        }
    }

    chain
}

/// Generates a new name using the Markov chain.
#[must_use]
pub fn generate_name<S: std::hash::BuildHasher>(
    chain: &HashMap<String, Vec<char>, S>,
    order: usize,
    max_len: usize,
) -> String {
    let mut current: String = "^".repeat(order);
    let mut result = String::new();

    for _ in 0..max_len {
        if let Some(next_chars) = chain.get(&current) {
            if let Some(next_char) = if next_chars.is_empty() {
                None
            } else {
                Some(next_chars[WyRand::new().generate_range(..next_chars.len())])
            } {
                if next_char == '^' || next_char == '\0' {
                    break;
                }
                result.push(next_char);
                current = format!("{}{}", &current[1..], next_char);
            } else {
                break;
            }
        } else {
            break;
        }
    }
    result.capitalize()
}
