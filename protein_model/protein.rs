use std::collections::HashMap;

pub struct Protein {
    protein_str: String,
    codon_ocurrence : HashMap<char, usize>,
}

impl Protein {
    pub fn new(p_str: String, codon_ocurr: HashMap<char, usize>) -> Protein {
        Protein {protein_str: p_str, codon_ocurrence: codon_ocurr}
    }

    pub fn setup_codon_ocurr(&mut self) {
           self.codon_ocurrence.insert('F', 2);
           self.codon_ocurrence.insert('L', 2);
           self.codon_ocurrence.insert('S', 4);
           self.codon_ocurrence.insert('Y', 2);
           self.codon_ocurrence.insert('\n', 3);
           self.codon_ocurrence.insert('C', 2);
           self.codon_ocurrence.insert('W', 1);
           self.codon_ocurrence.insert('L', 4);
           self.codon_ocurrence.insert('P', 4);
           self.codon_ocurrence.insert('H', 2);
           self.codon_ocurrence.insert('Q', 2);
           self.codon_ocurrence.insert('R', 6);
           self.codon_ocurrence.insert('I', 3);
           self.codon_ocurrence.insert('M', 1);
           self.codon_ocurrence.insert('T', 4);
           self.codon_ocurrence.insert('N', 2);
           self.codon_ocurrence.insert('K', 2);
           self.codon_ocurrence.insert('S', 2);
           self.codon_ocurrence.insert('R', 2);
           self.codon_ocurrence.insert('V', 4);
           self.codon_ocurrence.insert('A', 4);
           self.codon_ocurrence.insert('D', 2);
           self.codon_ocurrence.insert('E', 2);
           self.codon_ocurrence.insert('G', 4);
    }

    pub fn total_mrna_protein(&mut self) -> usize {
        
        self.setup_codon_ocurr();
        let mut over = String::from("");
        let mut total: usize = 0;

        for (key, val) in self.codon_ocurrence.iter() {
             if over.matches(&key.to_string()).count() == 0 {
                let mut matches = self.protein_str.matches(&key.to_string()).count();
                println!("{}, {}", key, val);
                println!("{}", matches);
                if matches != 0 { 
                    if total == 0 {
                        total = (total + matches * val) % 100000;
                    } else {
                        total = (total * matches * val) % 100000;
                    }
                    println!(" total =  {}", total);
                    over.push(*key);
                    println!("{}", over);
                        }
                    }
             }
        total 
    
    }
}
