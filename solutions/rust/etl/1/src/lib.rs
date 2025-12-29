use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut g = BTreeMap::<char, i32>::new(); 
    for (key, value) in h {
        for c in value {
            g.insert(c.to_ascii_lowercase(), *key);
        }
    }
    g
}
