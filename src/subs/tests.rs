use super::*;

#[test]
fn empty_string_x_empty_string() {
    assert_eq!(run("".as_bytes()), "");
}

#[test]
fn empty_substring_x_empty_string() { assert_eq!(run("ACGT\n".as_bytes()), "")}

#[test]
fn dna_characters_x_rna_characters() {
    assert_eq!(run("ACGT\nA".as_bytes()), "1");
    assert_eq!(run("ACGT\nT".as_bytes()), "4");
    assert_eq!(run("ACTACGTA\nTA".as_bytes()), "3 7");
}