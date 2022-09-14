
pub struct DNA {
    dna_str: String,
    nuc_a: usize,
    nuc_c: usize,    
    nuc_g: usize,
    nuc_t: usize,
}

impl DNA {
    pub fn new(dna_string: String) -> DNA {
        DNA { dna_str: dna_string, nuc_a: 0, nuc_c: 0, nuc_g: 0, nuc_t: 0 }

    }
    pub fn count(&mut self) {
    self.nuc_a = self.dna_str.matches("A").count();    
    self.nuc_c = self.dna_str.matches("C").count();    
    self.nuc_g = self.dna_str.matches("G").count();    
    self.nuc_t = self.dna_str.matches("T").count();    
    }

    pub fn print_nuc(&self) {
        println!("{} {} {} {}", self.nuc_a,
                                self.nuc_c,
                                self.nuc_g,
                                self.nuc_t);
    }
    pub fn to_rna(&mut self) -> String {
        self.dna_str.replace("T", "U")

    }

    pub fn rev_complement(&mut self) -> String {
        
        let reverse_complement: String = self.dna_str.chars().rev()
            .map(|x| match x {
                'A' => 'T',
                'C' => 'G',
                'G' => 'C',
                'T' => 'A',
                _ => todo!()
            }).collect();

        reverse_complement
        
    }
    
}






