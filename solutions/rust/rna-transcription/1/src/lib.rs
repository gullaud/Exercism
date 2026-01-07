#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    dna : String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    rna : String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (index, c) in dna.chars().enumerate() {
            if ! ['G', 'C', 'T', 'A'].contains(&c) {
                return Err(index);
            }
        }
        Ok(Dna {dna : String::from(dna)})
    }

    pub fn into_rna(self) -> Rna {
        let mut rna = String::with_capacity(self.dna.len());
        for c in self.dna.chars() {
            match c {
                'G' => { rna.push('C'); }
                'C' => { rna.push('G'); }
                'T' => { rna.push('A'); }
                'A' => { rna.push('U'); }
                _ => { panic!("Unexpected corruption of private data."); }
            }
        }
        Rna::new(&rna).unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (index, c) in rna.chars().enumerate() {
            if ! ['C', 'G', 'A', 'U'].contains(&c) {
                return Err(index);
            }
        }
        Ok(Rna {rna : String::from(rna)})
    }
}
