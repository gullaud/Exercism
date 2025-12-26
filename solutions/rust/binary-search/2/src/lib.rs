pub fn find<T>(slice: &[T], key: T) -> Option<usize> 
where T : std::cmp::PartialEq + std::cmp::PartialOrd + Copy
{
    if slice.is_empty() {
        return None;
    }
    let index = slice.len()/2;
    if slice[index] == key {
        return Some(index);
    }
    if slice[index] < key {
        let ui = find(&slice[index+1..], key);
        if let Some(val) = ui {
            return Some(index + 1 + val);
        }
    }
    if slice[index] > key {
        let li = find(&slice[..index], key);
        if li.is_some() {
            return li;
        }
    }
    None
}