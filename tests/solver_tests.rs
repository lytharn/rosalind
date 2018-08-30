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
fn run_pdna_x_number_of_dna_characters() {
    let input = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
    assert_eq!(run("dnap", input), "20 12 17 21");
}

#[test]
fn run_rna_x_dna_into_rna() {
    assert_eq!(run("rna", "GATGGAACTTGACTACGTAAATT"), "GAUGGAACUUGACUACGUAAAUU");
}

#[test]
fn run_recv_x_dna_into_reverse_complement() {
    assert_eq!(run("revc", "AAAACCCGGT"), "ACCGGGTTTT");
}

#[test]
fn run_subs_x_dna_and_substring_to_substring_pos() {
    assert_eq!(run("subs", "GATATATGCATATACTT\nATAT"), "2 4 10");
}