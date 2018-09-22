use super::*;

#[test]
fn empty_string_x_empty_string() {
    assert_eq!(run("".as_bytes()), "");
}

#[test]
fn dna_characters_x_reverse_complement() {
    assert_eq!(run("ACGT".as_bytes()), "ACGT");
    assert_eq!(run("AAAACCCGGT".as_bytes()), "ACCGGGTTTT");
}