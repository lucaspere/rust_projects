use std::collections::{BTreeMap, HashSet};

pub fn reverser_string(str: &str) -> String {
    str.chars().rev().collect()
}

pub fn palindrome_check(str: &str) -> bool {
    let cleaned = str
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();

    cleaned.chars().eq(cleaned.chars().rev())
}

pub fn word_count(str: &str) -> usize {
    let st: HashSet<&str> = str
        .trim()
        .split_whitespace()
        .filter(|c| c.chars().all(|c| c.is_alphabetic()))
        .collect();

    st.len()
}

pub fn title_case(str: &str) -> String {
    let non_alpha = str.chars().find(|c| !c.is_alphabetic()).unwrap_or_default();
    str.split(non_alpha)
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(&non_alpha.to_string())
}
pub fn longest_word(str: &str) -> String {
    let mut b_set: BTreeMap<usize, &str> = str
        .split_whitespace()
        .filter(|c| c.chars().all(|c| c.is_alphabetic()))
        .map(|word| (word.len(), word))
        .collect();

    let (_, longest) = b_set.pop_last().unwrap_or_default();

    longest.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_check() {
        assert!(palindrome_check("Racecar!"));
        assert!(palindrome_check("A man, a plan, a canal: Panama!"));
        assert!(palindrome_check("madam"));
        assert!(!palindrome_check("hello"));
        assert!(!palindrome_check("Rust"));
    }

    #[test]
    fn test_word_count() {
        assert_eq!(word_count("This is a sentence"), 4);
        assert_eq!(word_count("One word"), 2);
        assert_eq!(word_count(" Leading and trailing spaces "), 4);
        assert_eq!(word_count(""), 0);
    }

    #[test]
    fn test_title_case() {
        assert_eq!(title_case("hello world"), "Hello World");
        assert_eq!(title_case("one-two-three"), "One-Two-Three");
        assert_eq!(
            title_case("a sentence with some words"),
            "A Sentence With Some Words"
        );
        assert_eq!(title_case(""), "");
    }

    #[test]
    fn test_longest_word() {
        assert_eq!(longest_word("This is a long sentence"), "sentence");
        assert_eq!(longest_word("Two long words"), "words"); // Tie, return the first
        assert_eq!(longest_word("One"), "One");
        assert_eq!(longest_word(""), "");
    }
}
