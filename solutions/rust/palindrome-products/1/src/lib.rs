use std::collections::HashSet;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value : u64,
    factors : HashSet<(u64, u64)>,
}

impl Palindrome {

    pub fn new(i:u64, j:u64) -> Self {
        Palindrome {
            value : i*j, 
            factors : HashSet::from([(i,j)])
        }
    } 

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

fn is_palindrome(number: u64) -> bool {
    let s = number.to_string();
    s.bytes().eq(s.bytes().rev())
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut tree : BTreeMap<u64, Palindrome> = BTreeMap::new();
    for i in min..=max {
        for j in i..=max {
            let n = j*i;
            if is_palindrome(n) {
                tree.entry(n)
                    .and_modify(|pal| { pal.factors.insert((i,j)); })
                    .or_insert_with(|| { Palindrome::new(i,j) } );
            }
        }
    }

    if let (Some((_, smallest)), Some((_, largest))) = (tree.first_key_value(), tree.last_key_value()) {
        return Some((smallest.clone(), largest.clone()));
    }
    None
}
