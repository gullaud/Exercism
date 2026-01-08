fn append_c(count: usize, last_seen: Option<char>, buffer : &mut String ) {
    if count > 1 {
        buffer.push_str(&count.to_string());
    }
    if last_seen.is_some() {
        buffer.push(last_seen);
    }
}

pub fn encode(source: &str) -> String {
    let mut encoded = String::with_capacity(source.len());
    let mut last_seen = None;
    let mut count = 1;
    for c in source.chars() {
        if Some(c) == last_seen {
            count += 1;
        } else {
            if last_seen == None { 
                last_seen = Some(c);
                continue; 
            } else {
                append_c(count, last_seen, &mut encoded);
                last_seen = Some(c);
                count = 1;
            }
        }
    }
    append_c(count, last_seen, &mut encoded);
    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::with_capacity(source.len());
    let mut iter = source.chars();
    let mut count_str = String::new();
    while let Some(d) = iter.next() {
        if d.is_digit(10) {
            count_str.push(d);
        } else {
            if count_str.len() > 0 {
                let count = count_str.parse().unwrap();
                decoded.push_str(&std::iter::repeat(d).take(count).collect::<String>());
                count_str.clear();
            } else {
                decoded.push(d);
            }
        }
    }
    decoded
}
