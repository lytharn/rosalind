extern crate rosalind;

#[test]
fn run_dna_x_number_of_dna_characters() {
    let args = vec![String::from("rosalind"), String::from("dna")];
    let config = rosalind::Config::new(&args).unwrap();
    let input = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
    assert_eq!(rosalind::run(config, input), "20 12 17 21");
}

#[test]
fn run_rna_x_dna_into_rna() {
    let args = vec![String::from("rosalind"), String::from("rna")];
    let config = rosalind::Config::new(&args).unwrap();
    assert_eq!(rosalind::run(config, "GATGGAACTTGACTACGTAAATT"), "GAUGGAACUUGACUACGUAAAUU");
}