fn append_c(count: usize, last_seen: Option<char>, buffer : &mut String ) {
    if count > 1 {
        buffer.push_str(&count.to_string());
    }
    if let Some(item) = last_seen {
        buffer.push(item);
    }
}

pub fn encode(source: &str) -> String {
    let mut encoded = String::with_capacity(source.len());
    let mut last_seen = None;
    let mut count = 1;
    for c in source.chars() {
        if Some(c) == last_seen {
            count += 1;
        } else if last_seen.is_none() { 
                last_seen = Some(c);
                continue; 
        } else {
                append_c(count, last_seen, &mut encoded);
                last_seen = Some(c);
                count = 1;
        }
    }
    append_c(count, last_seen, &mut encoded);
    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::with_capacity(source.len());
    let iter = source.chars();
    let mut count_str = String::new();
    for d in iter {
        if d.is_ascii_digit() {
            count_str.push(d);
        } else if !count_str.is_empty() {
            let count = count_str.parse().unwrap();
            decoded.push_str(&std::iter::repeat_n(d,count).collect::<String>());
            count_str.clear();
        } else {
            decoded.push(d);
        }
    }
    decoded
}
