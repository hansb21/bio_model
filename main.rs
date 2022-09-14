mod dna_model;
mod rna_model;
mod protein_model;

pub use crate::dna_model::DNA;
pub use crate::rna_model::RNA;
pub use crate::protein_model::Protein;


fn main() {
    
    let dna_string = String::from("GCGGTCCACGTCCTCTCTGTCTTGACAGGCGTACATCAGAGTGAAGGGCTCGACGCCAGGTGGTACGTAACCGCAATCAGTCTTTTAGTATTAAACGAAGGGGGGTCTGCGAACGACATTTGGATGCCCCGATTAGAACGCATCCAGCGTGACCTCCAACGTCATTCAGATCTGAACGCGACAGTCCAATTTACAAGGTTCCTCCGCAGGGCGCCACGCTCTTTGTACATTTTCAGTTTTATGGATGATAAAACATACTGTGATAGCTATTGCAGGGGGACCATTCTACGTCGGACTTCCCACTCTTTAGCCACCAACGTTTCGACAACTGAGTTACCTCGTCACCGCAAGCGGTTTCCCTACTGACGGCTTCATACATATACACAAATGCTTTCATTTGTGACGTTTAGTACCCCCTCGTGAGTCGATCAGGATCTGAGTATGGCTGATCCTCTAGCCACTCTAGGCAATAGGAATCAAGACATGGTTCCGCAGATTTGAAGATCCCCTGGGGGCAATTGCTTAGTGTACGGTGGTTACAACGGTAAACGCGGTGAGTGTTGTTGCGCCCTGAGGTACCCGGAGCGCATGCGTCCAACCGTCATACAAGCTCATCTAAGATCAACCTCTCTCGGTAACTTTTGCAGAACAGGACGTTGTGGATTTGAGAGGGTTCACTAACTTAGTCGCTGCCCGACTACTGGCAGGGTATCTCAGGCCCAAATCGTCGGCGTCATCGCATGTTTCGTCGTCCGCACTGTAGTAAGTGAACAGCCCTGGGACAATCTAAGTAATACTAAAAGGGAAATTTCGCGGGTGGACGTTGGGAGACGACACGTCTCCAAGTATTTAGCAGAAT");
    
    let mut dna_sample = DNA::DNA::new(dna_string);
    dna_sample.count();
    //dna_sample.print_nuc();
    let rna_str = String::from("AUGAAAUUAGUAGAAGUGUUGUCAACCCGGGAUGAUACGGUAACGAGCUUAGCCUGCACAUAUGGCCGAUGCCUGUCUCUGAACACAUGGCGGGAUAUAACCAUUCGAGAAAAUACUACUUCUAGGAUCGAACAUACAACAAAGUCAACGGUAGAGCGAUCUCCCAAUGACCUCCAGGGAACCGAAGUGGCAGUCGCGAUGUGCGUCUGCUGUUAUGCGUGCGGUGAUCUACGACCAUGUAAAAUUCAAAUGAACGCGAGCAUCCGCUCGUUCAAAAGUUCAGGGCGGCUAAGCGGUCCAGCACAAGCCCAUGGUCGCUUGACGUCCUUAUCGGGUGAGUCAGACUUAUCCCAGGGGCGCGACGUGUUCGUGGCUGAACUCCUUGUAGCAGAAGGGCGGGGCGACCCAGUACUUCUCACCUCCCAUCGUGACCUUUUCAAUGAAUGGAUUAGGGGACAUCAUAUACUACCUCUUGUGGCACGAGGGCCCAGAGUUAGCGCGUCUGGAAAGGUAGGGCUAAUCUGGGACGCAUGCUGCCUUUUGGUCGCGGUACGCCGGACGCGUUAUCUUAAUCUCCACUGCAUACGCGUUAUAUAUAUCAGAAUGGACGCGAUCUCUUGCAUUGAGAGUGUAGGUGACACCAGGCGUCUUGGUAGAUUCAGGUCAGUUUGGAGACGUGGAGUGGGAGAGUGCGCACCCGAUCUUCGAUCCGCAUUAUCCCUAAGACCGAGUGUAAGACUUGGCUAUCGUGCUCUAAACGUAGUGAUCCGCACGGUGGGUAUAUGCGCACUGGGAACUUUUCUUACGCUCCGAGAAUCAAGUUUCAUAAAACCUUACUUGGGGGAACUUGGAUCGCUGGACACGCCUUUCGGGGCGAGGGAGAGUAGUCCUCCUGUCAUCGUAACAUAUGCUCGAGCAUUCUACAGGCUGGACAGUUCUGAACUGGCUAUCGCCGUUACUUACGUACAGGCGCCUUCCCUUUGGGGCCAUUUUACCAGUUAUGGAGAGCUUGCCGCUCCGCAGACAGAGAUAGGGCAAACCAGGUAUGAAAGGCGCUCCAUUAGUACUCAGACAUCUGUCAGAGCUCAUGACAAUCAGAAUACGGUAACAAUCCGAGACAGAUCUGAGUACGGCUACUACCAGAGCCUGGUGACAGACUAUUUUUGCUAUUUCACCGUACAGUCAUUUUAUAAUCAUUGUACAUGCAGGAUUCAGCCAUCCGAUCUUAACUGCCUUUGGGCACGCACAUAUAGGCCAUUGUUUGAAUACACCACGGAAGUAGCUUCAAAGCCCAACGGGCCCUCUGGCGACCCAGUUAGUGCUCGUCCUGUACUAGUAUCGGAUAGCUAUUUUCCCGGCGUCCUAUGGGGGAGCGGAACUCCGCCACCAUUCACUCUAGUGUCAAGACCGAGAAGAAGCAAGAGUGCCCGUCUAUUUCAUGGGCUCCAGUCGUUAAGGCCCACGUCCGGAAGACGCCCGGACUACGGCUCAUUCUUACUUAACUGCGAGGGUAAUACGCACCUAGUCAUAGAAAGUCUGUCCAGUUACCCAGUCGCUUUCGUUCCUCCCCUUCCAGUUACGGGGCGUUCGAGCCCUGCAUUGGAGCCACGCGUCGCCAAUGGUUCUCUGCUUGGUUUCAAUUAUGGACAGAUCGCGACGUCUUUCGGUGGCUGGACUGUUCAAGGCAACAAGUGCCUAAAAACACUCCCGAGGGUAUUCACUUCUGCGGCAGUCAUAGGUCGGGCCGUGUGCUGUGACUUUCUCCGAGCUACCAGUGACCCCUGUCGGCAACUUGGUGCCCCCGGGAACCCUAGACGUACACGACAACUAUUCUUUCCACAAGCCCGCACGCCGCCAAGCCACUCAACCGACAACGACGUUCAGGCAGAGGUAUCCCGAGCGAUUGACGGAGGAAGCCACGCGCAAACUAUCAUUCCAUAUCAAUACCUGGGUUAUCUGGUGCCGUGCCUAGAGACUCGCGGCAUAAGAGUAAUCUCUUCAGUCCACUUCUUCCCAGGAAUGUUUUGGGAAAUGUUGGGGCCAUGGGAAUUCUGUCGAUAUUUUGGUAUGCGGUCUGAGACGUGUUCCUUGGCUCUCCCCUCUAAACGCGAGAACAGAAACUCUAAAGAGCUGCCGAUUGGUGACGAUUCGACGUCAGACCCAUUACCAAAACACUCCAUGCGCACGAGUCACCAAAACCUUCAGCAGGGUUUCAUGCACUUUUCCCGUUGUAAAGAUAUACGUCGGAAGGAUACUGUGAUAGUCUCUCUUGUCAGAAUUUCAGAUCUCCUCCUAGUAGCGCUCUACGACCUGUUCAACCCGGACUGCGUAGACGCCACCUUACGCGUAGGGUGGCGUCAGGUUCCUGUGUAUCAAACUCGAUGCAGAGAAGCUGAAUGGGUACAUCAUAAAGCGGGCGUUUUGGAUCUCAUCCGACGCAACUCUGAAACGCUGUUUAUGGCUCCUACAUACAUCGCUGGGGGGGUCGUUGACGGUACAUAUAACAAGACAUUUCCGAAGGGAAGCACUACUAUUUACUGGGCGACAAAGACCAGUAUGAUACCUGCCGACACAAACUCAAUUACGCGCUCAGUUCUAGAGGAAAAAAGACAUGGUCUAUCCGUGGAAGAAGGGGAAUAUCGUGCAGAUCCAGCAUCCCCCGUUACAGUCGGUUCGAGAACAGCGGAUACGAGAAAGCACAGUGUGCAUUCGUUCAGUGUAACCCGCCAUACAUUCCAUGUGGUGGUCGAUAACUCAUCCGGACAAGGUCUCCCCCCUAGACCGGAAGCUAAGAGCCUAUCGAGAUACCUUUGUCUAAUUAACCCUUUGCAAACCUUAUUGGUCCUGCGCGGUUUAGCUUUCGCAAUUGGGCGUAACGCAAUAUAUAAAGCCAAUCUCACGUUCGGGUGGAUGCACAUGCCGGGCGAUAUCCCGUUACACCAAGUCAAUGCCGAACCAACACACACUAAUGAGAGUGACCGGAUCUGCGUCUUACGCAUAUCCUUAACCUCCACAAUGAAUCGCGUGAUCGAAACGACAAAGAUAGGCGUCAUGGUCGAAAGCCACUUAUUGUUGGCACGCACAUUCUCCCCUCACCGUAAGCAACACACCUCUGCGAAGGUUUCGCAAAUCAUAAAGCAUCGAGCACCCCUCUCAAUCACGACGGCACCGGGCAUAACGCUUCGCGUACAGCCUGACGGCGCAACAUCUGUACUAGGAUCGCGCUGGGAACGCAUCCAAAUAAUGCCAAAAGGCACUUGUGCCUACCGUUACUGCUGGGGUUUCGGAUCCGACCGGACUAGACUCAUAAGAUUUAGCGGAGGGCCCGCGGACCCUGUAACCGUACAUGUCCGUGGACAAUCGUGGACAUGCACGUCUACGCCCGCAUUAGGAAUAUUGAAGCUUCACAUCUCGAUACCCGCUCUUGUAUCUCAGGUAAACAAUUGGAUGGACGUAACGGCCGUGAUCAGCAGCCUAUCCUAUCGCUGGCUUAUGCGCAUUGACACUGCCGGGAGUCACCGCGACCACUUCGAUUUUCUAUCGGGCUUUUGCCCACAUAGCCGGGGCUUGUUACAGGAGUUCCAAAAAGAGGACGUACAACAAAACCGAUCUUGCGAGAACCCAAAGCGGCAAAACUUUGACCAUCUCAGAGCAUGCUACACCUUCUUAGACUCUACUACAGACAAAGGGCGGACUGGACGUAUAUUCCUAGAAAACGCAUGCUGGAUCAAUCAGUCAAACAACGCUGUCGAUCCCUGUCCUUCUGCGACGUCCGCUUAUGUAGACCAAGAUGAAGACUACCCCGGCGAUGUAGUGGCCCCUCCUUCACCUUUCCAUUUCGUGACGCCUGAAUCACGCCGAGCGGAGAGCGCAAUGGGUGGUGUACUUCACAGGAGGCCAACGACGCGAGGCACUGCAACUUGUCCAGCAAAACAGUUGAGGGUAAGCAUGAGAUUACACCAGAUUGACACUUAUUGUGAAUGCAUUGACGUCUUUCGGCGGCUUGAGUAUUCCGCUCUCGACUUCGGGACUCUCAUUGAUUACUCCCCGCUAAGAGCACAGUAUAUACCCCAAGCUCUGACAGCGUAUCACAAGGUAACCUGCUCUCAGAUCCAAUUUAUCACGGAACAGCUUUCCCGCCCUAAUCUUCCGGUAGUCUUAAGUUCCUCUGUACGUUCUCGUGUUUGGUCCGGUUCCUUAUGUGUAUUAGCUAAGGCAAGGGUACUGCAACUUAUUACGCAUCCUCACCGUUUCUCAGUGUGGUUCGUCAGCUACAUAAGACGAUGCAGGCUACACGUUAACAGGGUGGCUUUUCACGGAUCAUUUCUGCAUAGUCUCGUCAAUCGCCCUCCUCGUUCCGAAAUCUUACAUUUGGUACAUCUGAAAUCCGUGGCCAACCUGUUGGGGUAUGAUAUGAUUUCGAGCGCGACCCAACGGCCCGUUGCGAAUUUCAUGACCUAUACAUGUAUGCUCCUCGAAGUCCCGCCAUUUUCACCUAGUCACUAUAAAGAGGUGACGACACUAAAGACCUUGGGCAUCACUGAGGGACGUAACAGUAAGCAGUUUCUGGUUCUCCACUCAGGUACAUGUUCGGUGAUGAUCAGUAUCAGAAUAGGCCAAUUGUAUAUAGUGACCGCCCCCAGUAGGUACAAGGGGGGGACUCGUUCCAAAAUGAGACAUAAUACAGAGACGGCUUCCAGGCUGUAUAAAACUCCUAUUUCUCUCCGUCUGCCAAAAUGCAUGCUCUCGAUAUCCGAAGGGGCAUCCCUAUUCUGCCUCCGCGCCGGUCCCACGAAGCGUACUGGGGAAAUGGACCAGCAACGACGCGGAAGGAUUUCGGCCGGGGGUGCGCGGAGACUAGAAUAUGUGCGGGCCAACCUCGUUUGGACUUCUCUGACAGCGCGUUCGAGCGUCUGCGUCCCAAAACGUCACUUCGGAUGCCGCACGACAGACACAUACAACGUGUCGACUCGUGGCAGCGUUAUUAUCUCCUUGGAUAACGGCGGUAUGGUGCUCCUGGGUCCGGGUCCCCAACGCCGUAUUCAAAUCCGUAGUGCGGCAAUCAGGGGUUCGCCAGAUAGGAAGGCCAUAAGGUACCGCUUAUUACGGAUACGCCUGAGUUUAGAAAUUAUGAGCGGUAUCGCUCAUCUGGGGGGACGGUUCCCGAUCACAGCUCAUCUGUACUCGUCGAGAGAUCCUCUCUUUAAAGAGCCCAACGCUCGAAGGUUAAUGGACGGCACAGUAUUUCAGAUUUCGGUUCACAGGCCGGGAGCUUGUAAAACCCCGAGGUGUGGAGUGCUCGCGCGCGACACAACGUUGCCUCACUCUCAUCUCACACAGGUUAGACGCGACGGGCCUGGAUUCAGAAACAAGCGUGGGGAGAAUUGCUGGGCCAUCUCCGACACGGCAGUCGGCUUAAUAUGUGUGGAUGAAAGCCGGCCAAAUUGUGGACCCGAGGGGUUGGUAGAGAGACGUAACUAUCUCCGGUCACUCGAAUUGGAUCCAUGGGUAUCUGAAACCGUCGUCAAUUGCCCCAAGCUUCUAAACACUCUCAAGAAGUCGAGGGACUGCUACACGGCGAUUGAAGUAUUAUCGUUCAACUCACUUUGCGGGCCCGCCUUGCCACGGUUGUUUGUUGACCGGACUCUCAGGUUUUAUUUAAGUGUGACGCAAACUAAGGAAACUUUGCUGAGGUUUUCGGAAGACGGCUACCGCAGAAGCAAUGGAGCCAAGUGCCCUAAAAUAAAGUGCUGGGAGCGCGGCACAGUUCUUGUGUCUGUUCUAUCUCUCCGGGUACCGAGGUGUUCGUCGAUCAAGCGCGACCUUGAGUACUUUCUCGACCCUGUGAUCGGCUCAGGCGUACCAAACCCGACUCCAACCUACAUUAAACCUCGCUACUUAAUCCGGAACCUUCGAAUUAGAAAACAAGAAUGUCCCUGGGCGGCGCGUUGUGUGGCGCUAGCUCACAGCCCUUUCAAGGCUGAUCUCAGUUGCAGCCCUGGCAGUCGGGUAGGCUAUGCACCAAGGCCACCAACUUUUGGGCGCGUUUUGGGUCGAAAAUUCAUGCGCAUGACAUUAAACUUCCAACAGUAUUGUUUGUGUGACAACCGGUUCGUACUUACGACCCUAAAGGACAUCAGAAAGCUGAGUUUCGUUGCUUUCACGCGAUGGCGGAUAACAUUGGAACAUGGAUUCGUGGACCUCGGGCAUCGCCCAGUCAACCUACAGGCACUUUUCCAAUGCUCGCGAAGUCGUUUGGGCUAUCCGAUUUGCCACAUUUUGGGGGUGCGGAGCCAGCCUUGCUUAAUAUAUUCUUUCUGGAGCAGGGCCGUUGAAAAUGACGUGUCCGGCCCGCUUCCUGAACCAAGUGAGAGUAGCUCACAAACGGCGCAGAAUUACUGUAAUGGUUUGUACGCAGGGUAUGCCAGUCCCUACGUGCGCAUGCCUACUGUAUCACCGGGCCAGGCGUGUGUACGGAGAACUCAUUGUUGUCGAAGUCGGCAUACCAAAAGAUGUGUAUAUGUUGGUCGUGUAUCCCUUAUCCAGUUGCAGGGGUUGUUUGUGGCAAUCACUCUGUCGCUGCAAUGCUAUCCUUCACAUAUGUCGGUUAAGCUGCGUGGAGUUCGGUUUGCUGUAUCUCAUGCACUGGACCGGUGUUCUCAGUGUUCUCCGCCCUCGUGGGUGACAACGCCUGACCAGGACAAUCUGCCUCAAGUCAAGUCUAGCCUCUUUGUGGAACAUGUCGAAAGACUAGUAGGGGAAGUCGGAUUGAAUGGCACGCGUACCUUCACACCGCGCCCCUACGUGCAACGCAAACAACCUCAUCCCUACGUAGUUCAGCAGUGUCUCCUAGUGUCGGGUUCACCCAGUUAUGUGGCCCCACUCGUAGCCUGUGCUGAGGUUUUUUGGUUGGUCCCUUCUCCUGGUAUUAUAAAGCUCGAGGUUCUAACGUUGUAUGGAACUCCAUAUAGCCUAAAAUGCAAGCUGCUGUUCGAGAAAUCGCUAAUUCAAUUGGGGGUUCUUCUUCACCAAAACUGUUUUAAAUCGGUGCGGCGCGCGAACUUGUCAAGCUCUACCGGGGACCCUGUCAGCAUGGUGACACACGGAGUCCCUAUAGGACUCGUGGGUUCCAGCUUGGGCUCAGACCACACCACGUGUGGUAGCCGCUGCCCCGUAAGAUGUGUGCUACGGGUAAAUGUGGAAGCUUUCUGGUACAUAACGGAUGCGUACCGCGAUAGCCUAAUCCGGAACGCUUCGUUGGAGGUCUCGCUGGUUUGUAGACGGGCUCGCACGCGAAUAAUUCAGGAGAUUAUGGCGUCGGUAACCGUUCGAGGGCAGGUUCGGUUAUUGCGUACAUGGCCGCAUGUGCUGUAUGGUGUGAAGGAAAUAUCCUGUGGUUUUCGGGCCAGAAUAGUAGCGCAACGAUUUGCCUUGUUACUGCUCAGAGAUUCACAAACUGAUUUCGUUAGCUCGUGGGAGAUAUACAUUCAAGCAUUGCUAUAUGUAUACCCUAUACAGCUGGUAUGGCAAGGAGGUAAGCUUUCUGGCGCUUACAGUGUACGCGGACGUCGCUGGUAUUGUAAUUCCGGUGUUAUUGUCAACGGGACUCUGCUCUCAGAUGAGGGCCCCCGUAUAUUAGUCACAAGUGUUGCGUCAUUCUCUCGCCUAGAGCUCGGCCAAGAAUCAUUACACUCCCCUACCGUCCGUUCCAGCUUACAAAGAUCUUCGUACACACCCUACUUUGACGAGGCAACGACCUUGGCUUGGCUCUACGCAGAUCUGUCGGAUUGCAUUCUAACGCCUGUUGUACAUGAAGCCGGAACAGGUGAGUUUGGGACACGCCACUUCGUGUCGCUUCUUGAGAUCCACCGGUGUGGUAAAGGCUGCCUGUUAGUACAUGGGCCAUCUCAGCUCUACCAACACGUAAAGGGGACACGCUUCGCACUUGUAGGUUCUGCGCAGCGCUUGAAUGGCUCAUUAACGCAUCAGGAAUUUCAAAUUCCGCAUCGACCACAUGGAUUCUUCGUGUUUUCAGAAGUUCGAUCUUCAAUGGAACGCCUAGAUCCGAACCUUGCCGCUGUGCAGCAAUGCUUACCCGCCGAGCUUUGGACCUCCGAAGAUUAUGAACGUUCAAUAUCCGUCGUCAGUGGAUCGGCGGCGUUAAAUUUCGGUGUAGAAGCGGCGACUAGAUCUAGCCAUGUUUUGAAGUCGUGCACUCUGCAAGGGCUCCGAGGCUCUCAGCUUGAUAAAGAGAGUGCGUCUGCGAAACAAACCACAUUAGGUUUCAAAUUCGUACCCCCUUGCCUUAUCUCUGGUAGCGCGCCAAACCUAAUCGAGCUGACUCACCACAUGCUAGCCUUCAGCCGGAAAGACAGAGGCUCCCGGAAAGAGUCAAUGAAGAACAACCGAACUCCGGCUAUUCGGCAACUAGACGAGGAGUUCAACUCACUACGUCGUAUAUCAGUGGACCCCGUUAGGAGGCUUCUGGCAGCAUCGGGUGUGGUUACAUCUCAGGAAAUCAACUCGUCUCAGCGCAUUUCCAUCUAUUUCAAACCGGCGAGGCAUUCAGACGAUGGAUUACCCGACUUGGCCAUCACGCAGCUUUCUCUUUCUAGUGGAACUCGUAUACCGACAGGAUUUCAUUAUAGCAGACGUACCAGCAAAUUUAAGGUCAGAAUAGCUCGUUCCAUAAGGAAGAUACUGCCGUGUGUCGACACAUGCAGACUAAUCGGCAAAAUGAGUGCACGACACCAUCCAGUCAGCAACGUUGAGUACAGCUCUGACUCGGGGUGGUACCUAUGUUUGCGAGCUAUAUGUAAAAUCCCGCUAAUAUCGUGCCUGGUUAAGGUGGAUCCGGCUAUGAAGAAGCCGACCGUGCCUACUGACUACCGGAAACUGUUGCAGGAGUAUGUAUGUACGAACACACUAAUAUCUCCGGCCCGUGAUGUAUCCAGUUACACGUUUCUGACAGCAAACGUAGGCGACCUGAGUAUCCGAAUUUCUCUGCUCUCUAAACAACACUGUCAAAAGCCAGCCUCAACAUGUAAGAUACGUGUGGCUUAUAUCUCGUUUGCAUGGUCCUCUACAGCGUUCAUCGGCGCGGAGCAUCGUAUCCGCACCCCGAUCGGCAGCACGACAAGCCGAUCAGAGAUACGACAAAUAUCUUCGCACUACGAACCGAGGCUUUACCGAGUGAGUCUGGGGCACAACCGAAGCAAUAACGAAGGCGCUCUGAAUUCAGAAAGGAGUCUAAAAGUGAGUUGGAUGGGCGGGAGAAUUUCUCUGCUUUGUGAAGGAAUGUGUACUUUGCCUGAAUUUCUAGCUGAGGUCCUCGAUAACACGAUUCCGACGCAGCUCCUGGCUGGCGCAAGCGGCACUUUUCUUGCUGCCUCACACGGCCUUGGCGGAGCAGCCUAUCACAAAAAGAGGCCGUGCUGCGUGGACGCACGUCCCGGUGGGGGAGCAUGUACUAGGGAGCGCCGUAUUGUACUUGUGGACAAGCCCGGUACCCCUGCCUGCUUUCGGAGAUGA");
    let mut rna_sample = RNA::RNA::new(0, rna_str);
    let protein = rna_sample.to_protein();
    println!("{}", protein);
    let complement = dna_sample.rev_complement();
}