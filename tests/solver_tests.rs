extern crate rosalind;

fn run(arg: &str, input: &str) -> String {
    let args = vec![String::from("rosalind"), String::from(arg)];
    let config = rosalind::Config::new(&args).unwrap();
    rosalind::run(config, input)
}

#[test]
fn run_dna_x_number_of_dna_characters() {
    let input = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
    assert_eq!(run("dna", input), "20 12 17 21");
}

#[test]
fn run_rna_x_dna_into_rna() {
    assert_eq!(run("rna", "GATGGAACTTGACTACGTAAATT"), "GAUGGAACUUGACUACGUAAAUU");
}