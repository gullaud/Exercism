pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mol : Vec<char> = rna.chars().collect();
    let mut res: Vec<&str> = vec![];
    for slice in mol.chunks(3) {
        let codon : String = slice.iter().collect();
        if codon.len() < 3 {
            return None;
        }
        match codon.as_str() {
            "AUG" => { res.push("Methionine"); }
            "UUU" | "UUC" => { res.push("Phenylalanine"); }
            "UUA" | "UUG" => { res.push("Leucine"); }
            "UCU" | "UCC" | "UCA" | "UCG" => { res.push("Serine"); }
            "UAU" | "UAC" => { res.push("Tyrosine"); }
            "UGU" | "UGC" => { res.push("Cysteine"); }
            "UGG" => { res.push("Tryptophan"); }
            "UAA" | "UAG" | "UGA" => { return Some(res); }
            _ => {return  None;}
        }
    }
    Some(res)
}
