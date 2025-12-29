use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut letters = HashSet::new();
    let mut is_isogram = true;
    candidate.chars().filter(|&c| !c.is_whitespace() && c != '-' && c != '_')
        .map(|c| c.to_ascii_lowercase())
        .try_fold(0, |_, c| {
            dbg!(c);
            if letters.contains(&c) {
                is_isogram = false;
                return None;
            }
            letters.insert(c);
            Some(0)
        });
    is_isogram
}
