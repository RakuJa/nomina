use capitalize::Capitalize;
use itertools::Itertools;
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
            if let Some(next_char) = next_chars
                // if next_chars is empty it will generate 0 => get = None
                .get(WyRand::new().generate_range(..next_chars.len()))
                .copied()
            {
                if next_char == '^' || next_char == '\0' {
                    break;
                }
                result.push(next_char);
                current = format!("{}{}", &current[1..], next_char);
            } else {
                break;
            }
        } else {
            // Word generation graceful end => word completely generated
            result.push('^');
            break;
        }
    }
    ensure_complete_name(result)
}
#[must_use]
/// Capitalize all the substrings contained in a string.
///
/// `sep` is the separator used to recognize substrings
/// ```Rust
/// let x = capitalize_each_substring("hi who are you?", " ") // Could also use None
/// println!(x) // "Hi Who Are You?"
/// let y = capitalize_each_substring("hi,who", ",")
/// println!(y) // "Hi,Who"
/// ```
pub fn capitalize_each_substring(s: &str, sep: &str) -> String {
    s.split(sep).map(capitalize_string).join(sep)
}

#[must_use]
/// Capitalize the first letter of a string
pub fn capitalize_string(s: &str) -> String {
    s.capitalize()
}

fn ensure_complete_name(n: String) -> String {
    if n.ends_with('^') {
        n.chars().take(n.len() - 1).collect::<String>()
    } else if let Some(index) = n.rfind(' ') {
        n.split_at_checked(index)
            .map(|(w, _incomplete_word)| w)
            .map(String::from)
            .unwrap_or(n)
    } else {
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("hi who are you", " ", String::from("Hi Who Are You"))]
    #[case("hi;who;are;you", ";", String::from("Hi;Who;Are;You"))]
    #[case("hi who;are you", ";", String::from("Hi who;Are you"))]
    fn capitalize_substring_correct_separator(
        #[case] input_str: &str,
        #[case] sep: &str,
        #[case] expected: String,
    ) {
        let result = capitalize_each_substring(input_str, sep);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case("hi Who are you", " ", String::from("Hi Who Are You"))]
    #[case("hi;who;Are;you", ";", String::from("Hi;Who;Are;You"))]
    fn capitalize_substring_correct_separator_some_substring_already_capitalized(
        #[case] input_str: &str,
        #[case] sep: &str,
        #[case] expected: String,
    ) {
        let result = capitalize_each_substring(input_str, sep);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case("hi who are you", ";", String::from("Hi who are you"))]
    #[case("hi;who;are;you", " ", String::from("Hi;who;are;you"))]
    fn capitalize_substring_wrong_separator(
        #[case] input_str: &str,
        #[case] sep: &str,
        #[case] expected: String,
    ) {
        let result = capitalize_each_substring(input_str, sep);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case("hi Who are you", ";", String::from("Hi who are you"))]
    #[case("hi;who;Are;you", " ", String::from("Hi;who;are;you"))]
    fn capitalize_substring_wrong_separator_some_substring_already_capitalized(
        #[case] input_str: &str,
        #[case] sep: &str,
        #[case] expected: String,
    ) {
        let result = capitalize_each_substring(input_str, sep);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(String::from("Gianni^"), String::from("Gianni"))]
    #[case(
        String::from("KaeryelAlenarYsildea^"),
        String::from("KaeryelAlenarYsildea")
    )]
    #[case(
        String::from("Kaeryel Alenar Ysildea^"),
        String::from("Kaeryel Alenar Ysildea")
    )]
    #[case(String::from("^"), String::from(""))]
    fn test_ensure_complete_name_with_correct_delimiter(
        #[case] input_name: String,
        #[case] expected: String,
    ) {
        let result = ensure_complete_name(input_name);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(String::from("Gianni"), String::from("Gianni"))]
    #[case(
        String::from("KaeryelAlenarYsildea"),
        String::from("KaeryelAlenarYsildea")
    )]
    #[case(String::from(""), String::from(""))]
    #[case(String::from("^Marco"), String::from("^Marco"))]
    fn test_ensure_complete_name_without_correct_delimiter_and_only_one_word(
        #[case] input_name: String,
        #[case] expected: String,
    ) {
        let result = ensure_complete_name(input_name);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(
        String::from("Gladewalker Dream Of"),
        String::from("Gladewalker Dream")
    )]
    #[case(String::from("Barkskin Listener"), String::from("Barkskin"))]
    #[case(
        String::from("Nestle In Wintern Ro"),
        String::from("Nestle In Wintern")
    )]
    #[case(
        String::from("Raindrop On Falling "),
        String::from("Raindrop On Falling")
    )]
    #[case(String::from("Wind-carried Wish"), String::from("Wind-carried"))]
    #[case(
        String::from("Watcher Of Falling S"),
        String::from("Watcher Of Falling")
    )]
    #[case(
        String::from("Golden Sap Flicker O"),
        String::from("Golden Sap Flicker")
    )]
    fn test_ensure_complete_name_without_correct_delimiter_and_multiple_words(
        #[case] input_name: String,
        #[case] expected: String,
    ) {
        let result = ensure_complete_name(input_name);
        assert_eq!(result, expected);
    }
}
