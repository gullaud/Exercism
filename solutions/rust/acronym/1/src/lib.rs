fn process_word(word:&str) -> Vec<char> {
    let mut res = Vec::new();
    let mut first_found = false;
    if word == word.to_uppercase() {
        return vec![word.chars().nth(0).unwrap()];
    }
    for c in word.chars() {
        if c.is_ascii_alphabetic() && first_found == false {
            res.push(c); first_found = true;
        } 
        else if c.is_uppercase() {
            res.push(c);
        }
    }
    res
}

pub fn abbreviate(phrase: &str) -> String {
    let words:Vec<&str> = phrase.split_whitespace().flat_map(|p| p.split('-').filter(|c| c.len() > 0).collect::<Vec<&str>>()).collect();
    words.iter().flat_map(|w| process_word(w)).collect::<String>().to_uppercase()
}
