use std::collections::HashMap;

pub struct RNA {
    id: usize,
    rna_str: String,
}

impl RNA {

    pub fn new(id: usize, rna_string: String) -> RNA {
        RNA {id: id, rna_str: rna_string}
    }

    pub fn to_protein(&mut self) -> String {
        let mut protein = String::from("");
        let mut aux = String::from("");
        for (i, c) in self.rna_str.chars().enumerate() {
            aux.push(c);
            if (i + 1) % 3 == 0 {
                match aux.as_ref() {
                    "UUU" => protein.push('F'),
                    "UUC" => protein.push('F'),
                    "UUA" => protein.push('L'),
                    "UUG" => protein.push('L'),
                    "UCU" => protein.push('S'),
                    "UCC" => protein.push('S'),
                    "UCA" => protein.push('S'),
                    "UCG" => protein.push('S'),
                    "UAU" => protein.push('Y'),
                    "UAC" => protein.push('Y'),
                    "UAA" => protein.push('\n'), //stop
                    "UAG" => protein.push('\n'), //stop
                    "UGU" => protein.push('C'),
                    "UGC" => protein.push('C'),
                    "UGA" => protein.push('\n'), //stop
                    "UGG" => protein.push('W'),
                    "CUU" => protein.push('L'),
                    "CUC" => protein.push('L'),
                    "CUA" => protein.push('L'),
                    "CUG" => protein.push('L'),
                    "CCU" => protein.push('P'),
                    "CCC" => protein.push('P'),
                    "CCA" => protein.push('P'),
                    "CCG" => protein.push('P'),
                    "CAU" => protein.push('H'),
                    "CAC" => protein.push('H'),
                    "CAA" => protein.push('Q'),
                    "CAG" => protein.push('Q'),
                    "CGU" => protein.push('R'),
                    "CGC" => protein.push('R'),
                    "CGA" => protein.push('R'),
                    "CGG" => protein.push('R'),
                    "AUU" => protein.push('I'),
                    "AUC" => protein.push('I'),
                    "AUA" => protein.push('I'),
                    "AUG" => protein.push('M'),
                    "ACU" => protein.push('T'),
                    "ACC" => protein.push('T'),
                    "ACA" => protein.push('T'),
                    "ACG" => protein.push('T'),
                    "AAU" => protein.push('N'),
                    "AAC" => protein.push('N'),
                    "AAA" => protein.push('K'),
                    "AAG" => protein.push('K'),
                    "AGU" => protein.push('S'),
                    "AGC" => protein.push('S'),
                    "AGA" => protein.push('R'),
                    "AGG" => protein.push('R'),
                    "GUU" => protein.push('V'),
                    "GUC" => protein.push('V'),
                    "GUA" => protein.push('V'),
                    "GUG" => protein.push('V'),
                    "GCU" => protein.push('A'),
                    "GCC" => protein.push('A'),
                    "GCA" => protein.push('A'),
                    "GCG" => protein.push('A'),
                    "GAU" => protein.push('D'),
                    "GAC" => protein.push('D'),
                    "GAA" => protein.push('E'),
                    "GAG" => protein.push('E'),
                    "GGU" => protein.push('G'),
                    "GGC" => protein.push('G'),
                    "GGA" => protein.push('G'),
                    "GGG" => protein.push('G'),
                    _ => todo!()
                    }
                aux.clear();  
                }
            }

        protein
    }

    pub fn setup_codon_table(&self) -> HashMap<&str, char> {
        let codon_table = HashMap::from([
            ("UUU", 'F'),
            ("UUC", 'F'),
            ("UUA", 'L'),
            ("UUG", 'L'),
            ("UCU", 'S'),
            ("UCC", 'S'),
            ("UCA", 'S'),
            ("UCG", 'S'),
            ("UAU", 'Y'),
            ("UAC", 'Y'),
            ("UAA", '\n'),
            ("UAG", '\n'),
            ("UGU", 'C'),
            ("UGC", 'C'),
            ("UGA", '\n'),
            ("UGG", 'W'),
            ("CUU", 'L'),
            ("CUC", 'L'),
            ("CUA", 'L'),
            ("CUG", 'L'),
            ("CCU", 'P'),
            ("CCC", 'P'),
            ("CCA", 'P'),
            ("CCG", 'P'),
            ("CAU", 'H'),
            ("CAC", 'H'),
            ("CAA", 'Q'),
            ("CAG", 'Q'),
            ("CGU", 'R'),
            ("CGC", 'R'),
            ("CGA", 'R'),
            ("CGG", 'R'),
            ("AUU", 'I'),
            ("AUC", 'I'),
            ("AUCA", 'I'),
            ("AUCG", 'M'),
            ("ACCU", 'T'),
            ("ACCC", 'T'),
            ("ACCA", 'T'),
            ("ACCG", 'T'),
            ("AACU", 'N'),
            ("AACC", 'N'),
            ("AACA", 'K'),
            ("AACG", 'K'),
            ("AGCU", 'S'),
            ("AGCC", 'S'),
            ("AGCA", 'R'),
            ("AGCG", 'R'),
            ("GUU", 'V'),
            ("GUC", 'V'),
            ("GUA", 'V'),
            ("GUG", 'V'),
            ("GCU", 'A'),
            ("GCC", 'A'),
            ("GCA", 'A'),
            ("GCG", 'A'),
            ("GAU", 'D'),
            ("GAC", 'D'),
            ("GAA", 'E'),
            ("GAG", 'E'),
            ("GGU", 'G'),
            ("GGC", 'G'),
            ("GGA", 'G'),
            ("GGG", 'G'),
        ]);

        codon_table
    }
}