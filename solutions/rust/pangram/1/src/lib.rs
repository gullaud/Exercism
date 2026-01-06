use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    sentence.to_lowercase()
            .chars()
            .filter(|c| !c.is_whitespace() && *c != '_' && !c.is_ascii_punctuation() && !c.is_ascii_digit())
            .collect::<HashSet<char>>().len() == 26
}

/// Determine whether a sentence is a pangram.
pub fn is_pangram_iterative(sentence: &str) -> bool {
    let mut dic = HashSet::new();
    for c in sentence.to_lowercase().chars() {
        if c.is_whitespace() || c == '_' || c.is_ascii_punctuation() {
            continue;
        }
        if !c.is_alphanumeric() {
            return false;
        }
        if c.is_alphabetic() { dic.insert(c); }
    }
    dic.len() == 26
}

