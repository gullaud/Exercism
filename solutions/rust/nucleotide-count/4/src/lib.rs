use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if ! ['G', 'T', 'A', 'C'].contains(&nucleotide) {
        return Err(nucleotide);
    }
   let garbage = dna.chars().filter(|c| !['G', 'T', 'A', 'C'].contains(c)).collect::<Vec<_>>();
    if !garbage.is_empty() {
        return Err(garbage[0]);
    }
    Ok(dna.chars().filter(|&c| c == nucleotide).collect::<Vec<_>>().len())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut strands = HashMap::from([('G', 0), ('T', 0), ('A', 0), ('C', 0)]);
    for c in dna.chars() {
        match c {
            'G' | 'T' | 'A' | 'C' => {
                let count = strands.entry(c).or_insert(0);
                *count += 1;
            }
            _ => { return Err(c); }

        }       
    }
    Ok(strands)
}
