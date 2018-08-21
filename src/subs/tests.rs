use super::*;

#[test]
fn empty_string_x_empty_string() {
    assert_eq!(run(""), "");
}

#[test]
fn empty_substring_x_empty_string() { assert_eq!(run("ACGT\n"), "")}

#[test]
fn dna_characters_x_rna_characters() {
    assert_eq!(run("ACGT\nA"), "1");
    assert_eq!(run("ACGT\nT"), "4");
    assert_eq!(run("ACTACGTA\nTA"), "3 7");
}