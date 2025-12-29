use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut letters = HashSet::new();
    candidate.chars()
        .filter(|&c| !c.is_whitespace() && c != '-' && c != '_')
        .map(|c| c.to_ascii_lowercase())
        .all(|c| letters.insert(c))
}
