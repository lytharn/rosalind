extern crate rosalind;

fn run(arg: &str, input: &[u8]) -> String {
    let args = vec!["rosalind".to_string(), "file_path".to_string(), arg.to_string()];
    let config = rosalind::Config::new(args.into_iter()).unwrap();
    rosalind::run(config, input)
}

#[test]
fn run_dna_x_number_of_dna_characters() {
    let input = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC".as_bytes();
    assert_eq!(run("dna", input), "20 12 17 21");
}

#[test]
fn run_dnap_x_number_of_dna_characters() {
    let input = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC".as_bytes();
    assert_eq!(run("dnap", input), "20 12 17 21");
}

#[test]
fn run_rna_x_dna_into_rna() {
    assert_eq!(run("rna", "GATGGAACTTGACTACGTAAATT".as_bytes()), "GAUGGAACUUGACUACGUAAAUU");
}

#[test]
fn run_rnap_x_dna_into_rna() {
    assert_eq!(run("rnap", "GATGGAACTTGACTACGTAAATT".as_bytes()), "GAUGGAACUUGACUACGUAAAUU");
}

#[test]
fn run_recv_x_dna_into_reverse_complement() {
    assert_eq!(run("revc", "AAAACCCGGT".as_bytes()), "ACCGGGTTTT");
}

#[test]
fn run_recvp_x_dna_into_reverse_complement() {
    assert_eq!(run("revcp", "AAAACCCGGT".as_bytes()), "ACCGGGTTTT");
}

#[test]
fn run_subs_x_dna_and_substring_to_substring_pos() {
    assert_eq!(run("subs", "GATATATGCATATACTT\nATAT".as_bytes()), "2 4 10");
}

#[test]
fn run_subsp_x_dna_and_substring_to_substring_pos() {
    assert_eq!(run("subsp", "GATATATGCATATACTT\nATAT".as_bytes()), "2 4 10");
}
