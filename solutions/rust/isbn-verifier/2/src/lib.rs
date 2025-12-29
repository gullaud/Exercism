/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut checksum = 0;
    let factor = isbn.chars().filter(|c| *c != '-').try_fold(10, |factor, c| {
        if let Some(d) = c.to_digit(10) {
            checksum += factor * d;
            dbg!(checksum);

        } else if factor == 1 && c == 'X' {
            checksum += 10;
        } else {
            return None;
        }
        if factor == 0 { return None; }
        Some(factor - 1)
    });
    factor == Some(0) && checksum % 11 == 0
}
