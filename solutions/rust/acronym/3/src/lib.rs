// "GNU Image Manipulation Program"
pub fn abbreviate(phrase: &str) -> String {
    phrase.split(|c:char| c.is_whitespace() || c == '-' || c == '_')
        .flat_map(|word| word.chars().take(1).chain(
            word.chars()
                .skip_while(|c| c.is_uppercase())
                .filter(|c| c.is_uppercase())
        )).collect::<String>().to_uppercase()
}

fn process_word(word:&str) -> Vec<char> {
    let mut res = Vec::new();
    let mut first_found = false;
    if word == word.to_uppercase() {
        return vec![word.chars().next().unwrap()];
    }
    for c in word.chars() {
        if c.is_ascii_alphabetic() && !first_found {
            res.push(c); first_found = true;
        } 
        else if c.is_uppercase() {
            res.push(c);
        }
    }
    res
}

pub fn abbreviat0(phrase: &str) -> String {
    let words:Vec<&str> = phrase.split_whitespace().flat_map(|p| p.split('-').filter(|c| !c.is_empty()).collect::<Vec<&str>>()).collect();
    words.iter().flat_map(|w| process_word(w)).collect::<String>().to_uppercase()
}
