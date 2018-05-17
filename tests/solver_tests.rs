extern crate rosalind;

#[test]
fn run_dna_x_number_of_dna_characters() {
    let args = vec![String::from("rosalind"), String::from("dna")];
    let config = rosalind::Config::new(&args).unwrap();
    assert_eq!(rosalind::run(config, "ATGCATGATA"), "4 1 2 3");
}