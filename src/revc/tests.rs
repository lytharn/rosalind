use super::*;

#[test]
fn empty_string_x_empty_string() {
    assert_eq!(run(""), "");
}

#[test]
fn dna_characters_x_reverse_complement() {
    assert_eq!(run("ACGT"), "ACGT");
    assert_eq!(run("AAAACCCGGT"), "ACCGGGTTTT");
}