#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    let aliquot_sum : u64 = (1..num).filter(|n| num % n == 0).sum();
    dbg!(aliquot_sum);
    match num {
        0 => {return None;}
        a if a == aliquot_sum => {return Some(Classification::Perfect);}
        b if b > aliquot_sum => {return Some(Classification::Deficient);}
        _ => { return Some(Classification::Abundant); }
    }
}
